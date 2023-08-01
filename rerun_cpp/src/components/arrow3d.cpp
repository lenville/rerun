// NOTE: This file was autogenerated by re_types_builder; DO NOT EDIT.
// Based on "crates/re_types/definitions/rerun/components/arrow3d.fbs"

#include "arrow3d.hpp"

#include "../datatypes/arrow3d.hpp"
#include "../rerun.hpp"

#include <arrow/api.h>

namespace rr {
    namespace components {
        const char *Arrow3D::NAME = "rerun.arrow3d";

        const std::shared_ptr<arrow::DataType> &Arrow3D::to_arrow_datatype() {
            static const auto datatype = rr::datatypes::Arrow3D::to_arrow_datatype();
            return datatype;
        }

        arrow::Result<std::shared_ptr<arrow::StructBuilder>> Arrow3D::new_arrow_array_builder(
            arrow::MemoryPool *memory_pool
        ) {
            if (!memory_pool) {
                return arrow::Status::Invalid("Memory pool is null.");
            }

            return arrow::Result(
                rr::datatypes::Arrow3D::new_arrow_array_builder(memory_pool).ValueOrDie()
            );
        }

        arrow::Status Arrow3D::fill_arrow_array_builder(
            arrow::StructBuilder *builder, const Arrow3D *elements, size_t num_elements
        ) {
            if (!builder) {
                return arrow::Status::Invalid("Passed array builder is null.");
            }
            if (!elements) {
                return arrow::Status::Invalid("Cannot serialize null pointer to arrow array.");
            }

            static_assert(sizeof(rr::datatypes::Arrow3D) == sizeof(Arrow3D));
            ARROW_RETURN_NOT_OK(rr::datatypes::Arrow3D::fill_arrow_array_builder(
                builder,
                reinterpret_cast<const rr::datatypes::Arrow3D *>(elements),
                num_elements
            ));

            return arrow::Status::OK();
        }

        arrow::Result<rr::DataCell> Arrow3D::to_data_cell(
            const Arrow3D *instances, size_t num_instances
        ) {
            // TODO(andreas): Allow configuring the memory pool.
            arrow::MemoryPool *pool = arrow::default_memory_pool();

            ARROW_ASSIGN_OR_RAISE(auto builder, Arrow3D::new_arrow_array_builder(pool));
            if (instances && num_instances > 0) {
                ARROW_RETURN_NOT_OK(
                    Arrow3D::fill_arrow_array_builder(builder.get(), instances, num_instances)
                );
            }
            std::shared_ptr<arrow::Array> array;
            ARROW_RETURN_NOT_OK(builder->Finish(&array));

            auto schema =
                arrow::schema({arrow::field(Arrow3D::NAME, Arrow3D::to_arrow_datatype(), false)});

            rr::DataCell cell;
            cell.component_name = Arrow3D::NAME;
            ARROW_ASSIGN_OR_RAISE(
                cell.buffer,
                rr::ipc_from_table(*arrow::Table::Make(schema, {array}))
            );

            return cell;
        }
    } // namespace components
} // namespace rr
