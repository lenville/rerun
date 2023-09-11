# DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/python.rs
# Based on "crates/re_types/definitions/rerun/archetypes/line_strips3d.fbs".


from __future__ import annotations

from attrs import define, field

from .. import components
from .._baseclasses import (
    Archetype,
)

__all__ = ["LineStrips3D"]


@define(str=False, repr=False)
class LineStrips3D(Archetype):
    """
    A batch of line strips with positions and optional colors, radii, labels, etc.

    Example
    -------
    Many strips:
    ```python
    import rerun as rr
    import rerun.experimental as rr2

    rr.init("rerun_example_line_strip3d", spawn=True)

    rr2.log(
       "strips",
       rr2.LineStrips3D(
           [
               [
                   [0, 0, 2],
                   [1, 0, 2],
                   [1, 1, 2],
                   [0, 1, 2],
               ],
               [
                   [0, 0, 0],
                   [0, 0, 1],
                   [1, 0, 0],
                   [1, 0, 1],
                   [1, 1, 0],
                   [1, 1, 1],
                   [0, 1, 0],
                   [0, 1, 1],
               ],
           ],
           colors=[[255, 0, 0], [0, 255, 0]],
           radii=[0.025, 0.005],
           labels=["one strip here", "and one strip there"],
       ),
    )
    ```

    Many individual segments:
    ```python
    #!/usr/bin/env python3
    import numpy as np
    import rerun as rr
    import rerun.experimental as rr2

    rr.init("rerun_example_line_segments3d", spawn=True)

    rr2.log(
       "segments",
       rr2.LineStrips3D(
           np.array(
               [
                   [[0, 0, 0], [0, 0, 1]],
                   [[1, 0, 0], [1, 0, 1]],
                   [[1, 1, 0], [1, 1, 1]],
                   [[0, 1, 0], [0, 1, 1]],
               ],
           )
       ),
    )
    ```
    """

    strips: components.LineStrip3DArray = field(
        metadata={"component": "primary"},
        converter=components.LineStrip3DArray.from_similar,  # type: ignore[misc]
    )
    """
    All the actual 3D line strips that make up the batch.
    """

    radii: components.RadiusArray | None = field(
        metadata={"component": "secondary"},
        default=None,
        converter=components.RadiusArray.from_similar,  # type: ignore[misc]
    )
    """
    Optional radii for the line strips.
    """

    colors: components.ColorArray | None = field(
        metadata={"component": "secondary"},
        default=None,
        converter=components.ColorArray.from_similar,  # type: ignore[misc]
    )
    """
    Optional colors for the line strips.
    """

    labels: components.TextArray | None = field(
        metadata={"component": "secondary"},
        default=None,
        converter=components.TextArray.from_similar,  # type: ignore[misc]
    )
    """
    Optional text labels for the line strips.
    """

    class_ids: components.ClassIdArray | None = field(
        metadata={"component": "secondary"},
        default=None,
        converter=components.ClassIdArray.from_similar,  # type: ignore[misc]
    )
    """
    Optional `ClassId`s for the lines.

    The class ID provides colors and labels if not specified explicitly.
    """

    instance_keys: components.InstanceKeyArray | None = field(
        metadata={"component": "secondary"},
        default=None,
        converter=components.InstanceKeyArray.from_similar,  # type: ignore[misc]
    )
    """
    Unique identifiers for each individual line strip in the batch.
    """

    __str__ = Archetype.__str__
    __repr__ = Archetype.__repr__
