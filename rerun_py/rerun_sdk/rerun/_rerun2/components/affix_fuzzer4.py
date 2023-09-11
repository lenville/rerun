# DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/python.rs
# Based on "crates/re_types/definitions/rerun/testing/components/fuzzy.fbs".


from __future__ import annotations

from .. import datatypes
from .._baseclasses import (
    BaseDelegatingExtensionArray,
    BaseDelegatingExtensionType,
)

__all__ = ["AffixFuzzer4Array", "AffixFuzzer4Type"]


class AffixFuzzer4Type(BaseDelegatingExtensionType):
    _TYPE_NAME = "rerun.testing.components.AffixFuzzer4"
    _DELEGATED_EXTENSION_TYPE = datatypes.AffixFuzzer1Type


class AffixFuzzer4Array(BaseDelegatingExtensionArray[datatypes.AffixFuzzer1ArrayLike]):
    _EXTENSION_NAME = "rerun.testing.components.AffixFuzzer4"
    _EXTENSION_TYPE = AffixFuzzer4Type
    _DELEGATED_ARRAY_TYPE = datatypes.AffixFuzzer1Array


AffixFuzzer4Type._ARRAY_TYPE = AffixFuzzer4Array

# TODO(cmc): bring back registration to pyarrow once legacy types are gone
# pa.register_extension_type(AffixFuzzer4Type())
