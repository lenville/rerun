use std::collections::BTreeMap;

use re_data_store::LatestAtQuery;
use re_entity_db::EntityPath;
use re_space_view::diff_component_filter;
use re_types::{archetypes::BarChart, components::Color, datatypes::TensorData};
use re_viewer_context::{
    auto_color_for_entity_path, IdentifiedViewSystem, QueryContext, SpaceViewSystemExecutionError,
    TypedComponentFallbackProvider, ViewContext, ViewContextCollection, ViewQuery,
    VisualizerAdditionalApplicabilityFilter, VisualizerQueryInfo, VisualizerSystem,
};

/// A bar chart system, with everything needed to render it.
#[derive(Default)]
pub struct BarChartVisualizerSystem {
    pub charts: BTreeMap<EntityPath, (TensorData, Color)>,
}

impl IdentifiedViewSystem for BarChartVisualizerSystem {
    fn identifier() -> re_viewer_context::ViewSystemIdentifier {
        "BarChart".into()
    }
}

struct BarChartVisualizerEntityFilter;

impl VisualizerAdditionalApplicabilityFilter for BarChartVisualizerEntityFilter {
    fn update_applicability(&mut self, event: &re_data_store::StoreEvent) -> bool {
        diff_component_filter(event, |tensor: &re_types::components::TensorData| {
            tensor.is_vector()
        })
    }
}

impl VisualizerSystem for BarChartVisualizerSystem {
    fn visualizer_query_info(&self) -> VisualizerQueryInfo {
        VisualizerQueryInfo::from_archetype::<BarChart>()
    }

    fn applicability_filter(&self) -> Option<Box<dyn VisualizerAdditionalApplicabilityFilter>> {
        Some(Box::new(BarChartVisualizerEntityFilter))
    }

    fn execute(
        &mut self,
        ctx: &ViewContext<'_>,
        query: &ViewQuery<'_>,
        _context_systems: &ViewContextCollection,
    ) -> Result<Vec<re_renderer::QueueableDrawData>, SpaceViewSystemExecutionError> {
        re_tracing::profile_function!();

        for data_result in query.iter_visible_data_results(ctx, Self::identifier()) {
            // TODO(#5607): what should happen if the promise is still pending?
            let query = LatestAtQuery::new(query.timeline, query.latest_at);
            let query_ctx = ctx.query_context(data_result, &query);

            let tensor = ctx
                .recording()
                .latest_at_component::<re_types::components::TensorData>(
                    &data_result.entity_path,
                    &query,
                );

            let color = ctx
                .recording()
                .latest_at_component::<re_types::components::Color>(
                    &data_result.entity_path,
                    &query,
                );

            if let Some(tensor) = tensor {
                if tensor.is_vector() {
                    self.charts.insert(
                        data_result.entity_path.clone(),
                        (
                            tensor.value.0.clone(),
                            color.map_or_else(|| self.fallback_for(&query_ctx), |c| c.value),
                        ),
                    );
                    // shallow clones
                }
            }
        }

        Ok(Vec::new())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_fallback_provider(&self) -> &dyn re_viewer_context::ComponentFallbackProvider {
        self
    }
}

impl TypedComponentFallbackProvider<Color> for BarChartVisualizerSystem {
    fn fallback_for(&self, ctx: &QueryContext<'_>) -> Color {
        auto_color_for_entity_path(ctx.target_entity_path)
    }
}

re_viewer_context::impl_component_fallback_provider!(BarChartVisualizerSystem => [Color]);
