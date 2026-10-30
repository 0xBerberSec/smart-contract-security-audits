# Time-Dependent Execution Boundaries

**Method overview — underlying technical case withheld.**

## Research question

Protocols can use several notions of time: an oracle observation, a stored flag, an epoch boundary or an accounting update. The relevant question is which one controls each execution path.

## Approach

I trace when state is read and updated, compare creation with execution, and identify the actors able to advance each state. A focused experiment isolates the ordering assumption instead of treating all time-dependent paths as equivalent.

## Counter-tests and interpretation

The control should exercise the expected ordering and test the relevant guard. A result from a local configuration must remain qualified by its oracle fixtures, parameters and actor permissions.

## Evidence boundary

This illustrates temporal reasoning and test design. No private transition sequence, vulnerable function, parameter or production precondition is disclosed. This page describes a method; it is not independently verifiable execution evidence.

For a separate executable teaching example of the workflow, see the [Refund Escrow lab](../educational-demos/refund-escrow/README.md). It is independent of this withheld study.

[All investigation overviews](README.md) · [Testing approach](../research-tools-testing/testing.md) · [Portfolio](../README.md)
