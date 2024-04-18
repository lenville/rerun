// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/rust/to_archetype.rs

#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]

use crate::{LatestAtResults, PromiseResolver, PromiseResult};
use re_types_core::{Archetype, Loggable as _};
use std::sync::Arc;

impl crate::ToArchetype<re_types::blueprint::archetypes::SpaceViewBlueprint>
    for LatestAtResults
{
    #[inline]
    fn to_archetype(
        &self,
        resolver: &PromiseResolver,
    ) -> PromiseResult<crate::Result<re_types::blueprint::archetypes::SpaceViewBlueprint>> {
        re_tracing::profile_function!(<re_types::blueprint::archetypes::SpaceViewBlueprint>::name());

        // --- Required ---

        use re_types::blueprint::components::SpaceViewClass;
        let class_identifier = match self.get_required(<SpaceViewClass>::name()) {
            Ok(class_identifier) => class_identifier,
            Err(query_err) => return PromiseResult::Ready(Err(query_err)),
        };
        let class_identifier = match class_identifier.to_dense::<SpaceViewClass>(resolver) {
            PromiseResult::Pending => return PromiseResult::Pending,
            PromiseResult::Error(promise_err) => return PromiseResult::Error(promise_err),
            PromiseResult::Ready(query_res) => match query_res {
                Ok(data) => {
                    let Some(first) = data.first().cloned() else {
                        return PromiseResult::Error(std::sync::Arc::new(
                            re_types_core::DeserializationError::missing_data(),
                        ));
                    };
                    first
                }
                Err(query_err) => return PromiseResult::Ready(Err(query_err)),
            },
        };

        // --- Recommended/Optional ---

        use re_types::components::Name;
        let display_name = if let Some(display_name) = self.get(<Name>::name()) {
            match display_name.to_dense::<Name>(resolver) {
                PromiseResult::Pending => return PromiseResult::Pending,
                PromiseResult::Error(promise_err) => return PromiseResult::Error(promise_err),
                PromiseResult::Ready(query_res) => match query_res {
                    Ok(data) => data.first().cloned(),
                    Err(query_err) => return PromiseResult::Ready(Err(query_err)),
                },
            }
        } else {
            None
        };

        use re_types::blueprint::components::SpaceViewOrigin;
        let space_origin = if let Some(space_origin) = self.get(<SpaceViewOrigin>::name()) {
            match space_origin.to_dense::<SpaceViewOrigin>(resolver) {
                PromiseResult::Pending => return PromiseResult::Pending,
                PromiseResult::Error(promise_err) => return PromiseResult::Error(promise_err),
                PromiseResult::Ready(query_res) => match query_res {
                    Ok(data) => data.first().cloned(),
                    Err(query_err) => return PromiseResult::Ready(Err(query_err)),
                },
            }
        } else {
            None
        };

        use re_types::blueprint::components::Visible;
        let visible = if let Some(visible) = self.get(<Visible>::name()) {
            match visible.to_dense::<Visible>(resolver) {
                PromiseResult::Pending => return PromiseResult::Pending,
                PromiseResult::Error(promise_err) => return PromiseResult::Error(promise_err),
                PromiseResult::Ready(query_res) => match query_res {
                    Ok(data) => data.first().cloned(),
                    Err(query_err) => return PromiseResult::Ready(Err(query_err)),
                },
            }
        } else {
            None
        };

        // ---

        let arch = re_types::blueprint::archetypes::SpaceViewBlueprint {
            class_identifier,
            display_name,
            space_origin,
            visible,
        };

        PromiseResult::Ready(Ok(arch))
    }
}
