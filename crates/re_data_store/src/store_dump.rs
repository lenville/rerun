use std::collections::BTreeMap;

use arrow2::Either;
use re_log_types::{
    DataCellColumn, DataRow, DataTable, ErasedTimeVec, RowIdVec, TableId, TimeInt, TimeRange,
    Timeline,
};

use crate::{
    store::{IndexedBucketInner, PersistentIndexedTable, PersistentIndexedTableInner},
    DataStore, IndexedBucket,
};

// ---

impl DataStore {
    /// Serializes the entire datastore into one big sorted list of [`DataRow`].
    ///
    /// Individual [`re_log_types::DataRow`]s that were split apart due to bucketing are merged back together.
    ///
    /// Beware: this is extremely costly, don't use this in hot paths.
    pub fn to_rows(&self) -> re_log_types::DataReadResult<Vec<DataRow>> {
        use re_log_types::RowId;
        re_tracing::profile_function!();

        let mut rows = ahash::HashMap::<RowId, DataRow>::default();
        for table in self.to_data_tables(None) {
            for row in table.to_rows().collect::<Vec<_>>() {
                let row = row?;
                match rows.entry(row.row_id()) {
                    std::collections::hash_map::Entry::Occupied(mut entry) => {
                        for (timeline, time) in row.timepoint() {
                            entry.get_mut().timepoint.insert(*timeline, *time);
                        }
                    }
                    std::collections::hash_map::Entry::Vacant(entry) => {
                        entry.insert(row);
                    }
                }
            }
        }

        let mut rows = rows.into_values().collect::<Vec<_>>();
        {
            re_tracing::profile_scope!("sort_rows");
            rows.sort_by_key(|row| (row.timepoint.clone(), row.row_id));
        }

        Ok(rows)
    }

    /// Serializes the entire datastore into one big sorted [`DataTable`].
    ///
    /// Individual [`re_log_types::DataRow`]s that were split apart due to bucketing are merged back together.
    ///
    /// Beware: this is extremely costly, don't use this in hot paths.
    pub fn to_data_table(&self) -> re_log_types::DataReadResult<DataTable> {
        re_tracing::profile_function!();

        let rows = self.to_rows()?;

        Ok(re_log_types::DataTable::from_rows(
            re_log_types::TableId::new(),
            rows,
        ))
    }

    /// Serializes the entire datastore into an iterator of [`DataTable`]s, where each table
    /// corresponds 1-to-1 to an internal bucket.
    // TODO(#1793): This shouldn't dump cluster keys that were autogenerated.
    pub fn to_data_tables(
        &self,
        time_filter: Option<(Timeline, TimeRange)>,
    ) -> impl Iterator<Item = DataTable> + '_ {
        let timeless = self.dump_timeless_tables();
        let temporal = if let Some(time_filter) = time_filter {
            Either::Left(self.dump_temporal_tables_filtered(time_filter))
        } else {
            Either::Right(self.dump_temporal_tables())
        };

        timeless.chain(temporal)
    }

    fn dump_timeless_tables(&self) -> impl Iterator<Item = DataTable> + '_ {
        self.timeless_tables.values().map(|table| {
            re_tracing::profile_scope!("timeless_table");

            let PersistentIndexedTable {
                ent_path,
                cluster_key: _,
                inner,
            } = table;

            let inner = &*inner.read();
            let PersistentIndexedTableInner {
                col_insert_id: _,
                col_row_id,
                col_num_instances,
                columns,
                is_sorted: _,
            } = inner;

            DataTable {
                table_id: TableId::new(),
                col_row_id: col_row_id.clone(),
                col_timelines: Default::default(),
                col_entity_path: std::iter::repeat_with(|| ent_path.clone())
                    .take(inner.num_rows() as _)
                    .collect(),
                col_num_instances: col_num_instances.clone(),
                columns: columns.clone().into_iter().collect(), // shallow
            }
        })
    }

    fn dump_temporal_tables(&self) -> impl Iterator<Item = DataTable> + '_ {
        self.tables.values().flat_map(|table| {
            re_tracing::profile_scope!("temporal_table");

            table.buckets.values().map(move |bucket| {
                re_tracing::profile_scope!("temporal_bucket");

                bucket.sort_indices_if_needed();

                let IndexedBucket {
                    timeline,
                    cluster_key: _,
                    inner,
                } = bucket;

                let IndexedBucketInner {
                    is_sorted: _,
                    time_range: _,
                    col_time,
                    col_insert_id: _,
                    col_row_id,
                    max_row_id: _,
                    col_num_instances,
                    columns,
                    size_bytes: _,
                } = &*inner.read();

                DataTable {
                    table_id: TableId::new(),
                    col_row_id: col_row_id.clone(),
                    col_timelines: [(*timeline, col_time.iter().copied().map(Some).collect())]
                        .into(),
                    col_entity_path: std::iter::repeat_with(|| table.ent_path.clone())
                        .take(col_row_id.len())
                        .collect(),
                    col_num_instances: col_num_instances.clone(),
                    columns: columns.clone().into_iter().collect(), // shallow
                }
            })
        })
    }

    fn dump_temporal_tables_filtered(
        &self,
        (timeline_filter, time_filter): (Timeline, TimeRange),
    ) -> impl Iterator<Item = DataTable> + '_ {
        self.tables
            .values()
            .filter_map(move |table| {
                re_tracing::profile_scope!("temporal_table_filtered");

                if table.timeline != timeline_filter {
                    return None;
                }

                Some(table.buckets.values().filter_map(move |bucket| {
                    re_tracing::profile_scope!("temporal_bucket_filtered");

                    bucket.sort_indices_if_needed();

                    let IndexedBucket {
                        timeline,
                        cluster_key: _,
                        inner,
                    } = bucket;

                    let IndexedBucketInner {
                        is_sorted: _,
                        time_range,
                        col_time,
                        col_insert_id: _,
                        col_row_id,
                        max_row_id: _,
                        col_num_instances,
                        columns,
                        size_bytes: _,
                    } = &*inner.read();

                    if !time_range.intersects(time_filter) {
                        return None;
                    }

                    let col_row_id: RowIdVec =
                        filter_column(col_time, col_row_id.iter(), time_filter).collect();

                    // NOTE: Shouldn't ever happen due to check above, but better safe than
                    // sorry…
                    debug_assert!(!col_row_id.is_empty());
                    if col_row_id.is_empty() {
                        return None;
                    }

                    let col_timelines = [(
                        *timeline,
                        filter_column(col_time, col_time.iter(), time_filter)
                            .map(Some)
                            .collect(),
                    )]
                    .into();

                    let col_entity_path = std::iter::repeat_with(|| table.ent_path.clone())
                        .take(col_row_id.len())
                        .collect();

                    let col_num_instances =
                        filter_column(col_time, col_num_instances.iter(), time_filter).collect();

                    let mut columns2 = BTreeMap::default();
                    for (component, column) in columns {
                        let column = filter_column(col_time, column.iter(), time_filter).collect();
                        columns2.insert(*component, DataCellColumn(column));
                    }

                    Some(DataTable {
                        table_id: TableId::new(),
                        col_row_id,
                        col_timelines,
                        col_entity_path,
                        col_num_instances,
                        columns: columns2,
                    })
                }))
            })
            .flatten()
    }
}

fn filter_column<'a, T: 'a + Clone>(
    col_time: &'a ErasedTimeVec,
    column: impl Iterator<Item = &'a T> + 'a,
    time_filter: TimeRange,
) -> impl Iterator<Item = T> + 'a {
    col_time
        .iter()
        .zip(column)
        .filter(move |(&time, _)| time_filter.contains(TimeInt::new_temporal(time)))
        .map(|(_, v)| v.clone())
}
