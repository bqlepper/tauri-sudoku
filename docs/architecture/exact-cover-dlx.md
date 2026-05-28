# Sudoku Solver Design: Exact Cover + Dancing Links

## Purpose

This document is the durable design record for the Sudoku exact-cover/DLX solver.
It is intended to hold final implementation details after the refactor is complete.

## Problem Model

Sudoku is represented as an exact cover problem:

1. 729 candidate assignment rows (`9 x 9 x 9`).
2. 324 constraint columns (`9 x 9 x 4`).
3. Each candidate row satisfies exactly 4 constraints.

## Constraint Families

1. Cell occupancy: each cell contains exactly one value.
2. Row-value uniqueness: each row contains each value exactly once.
3. Column-value uniqueness: each column contains each value exactly once.
4. Box-value uniqueness: each 3x3 box contains each value exactly once.

## Solver Strategy

1. Convert clues to forced candidate rows.
2. Cover corresponding columns and incompatible rows using DLX operations.
3. Search remaining matrix with early-stop limit for solution counting.
4. Map selected candidate rows back to Sudoku grid values.

## Result Semantics

Solution count status (mutually exclusive):

1. `Contradiction` (0)
2. `Unique` (1)
3. `MultipleBelowCap(n)` (2..cap-1)
4. `AtLeastCap(cap)` (>= cap, early stopped)

Board progress state (orthogonal):

1. `SolvedNow`
2. `UnsolvedNow`

## Current References

1. https://cs.indstate.edu/~bdhome/SUDOKU.pdf
2. https://www.stolaf.edu/people/hansonr/sudoku/exactcovermatrix.htm

## Notes for Post-Refactor Update

When implementation is complete, update this doc with:

1. Exact index formulas used in code.
2. Data structure layout for nodes/headers.
3. Complexity and performance notes from test runs.
4. Any compatibility behaviors preserved from legacy solver logic.
