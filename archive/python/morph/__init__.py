"""
ManQuanFa Morph Layer - replaces vector math with morphism math
"""
from .morphism import Object, Morphism, Path, identity, is_isomorphism
from .relational import (
    Relation_, Tuple_, Name,
    selection, projection, natural_join, division, transitive_closure,
    RelOp, Query,
)
from .ternary_arbiter import TernaryArbiter, judge
from .binary_mirror import BinaryMirror, ConvergenceType
from .convergence import ReferenceFrame, detect_phase, growth_pattern

__all__ = [
    "Object", "Morphism", "Path", "identity", "is_isomorphism",
    "Relation_", "Tuple_", "Name",
    "selection", "projection", "natural_join", "division",
    "transitive_closure", "RelOp", "Query",
    "TernaryArbiter", "judge",
    "BinaryMirror", "ConvergenceType",
    "ReferenceFrame", "detect_phase", "growth_pattern",
]
