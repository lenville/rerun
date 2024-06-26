// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/re_types/definitions/rerun/components/opacity.fbs".

#pragma once

#include "../datatypes/float32.hpp"
#include "../result.hpp"

#include <cstdint>
#include <memory>

namespace rerun::components {
    /// **Component**: Degree of transparency ranging from 0.0 (fully transparent) to 1.0 (fully opaque).
    ///
    /// The final opacity value may be a result of multiplication with alpha values as specified by other color sources.
    /// Unless otherwise specified, the default value is 1.
    struct Opacity {
        rerun::datatypes::Float32 opacity;

      public:
        Opacity() = default;

        Opacity(rerun::datatypes::Float32 opacity_) : opacity(opacity_) {}

        Opacity& operator=(rerun::datatypes::Float32 opacity_) {
            opacity = opacity_;
            return *this;
        }

        Opacity(float value_) : opacity(value_) {}

        Opacity& operator=(float value_) {
            opacity = value_;
            return *this;
        }

        /// Cast to the underlying Float32 datatype
        operator rerun::datatypes::Float32() const {
            return opacity;
        }
    };
} // namespace rerun::components

namespace rerun {
    static_assert(sizeof(rerun::datatypes::Float32) == sizeof(components::Opacity));

    /// \private
    template <>
    struct Loggable<components::Opacity> {
        static constexpr const char Name[] = "rerun.components.Opacity";

        /// Returns the arrow data type this type corresponds to.
        static const std::shared_ptr<arrow::DataType>& arrow_datatype() {
            return Loggable<rerun::datatypes::Float32>::arrow_datatype();
        }

        /// Serializes an array of `rerun::components::Opacity` into an arrow array.
        static Result<std::shared_ptr<arrow::Array>> to_arrow(
            const components::Opacity* instances, size_t num_instances
        ) {
            return Loggable<rerun::datatypes::Float32>::to_arrow(
                &instances->opacity,
                num_instances
            );
        }
    };
} // namespace rerun
