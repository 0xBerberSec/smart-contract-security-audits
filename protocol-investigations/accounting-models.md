# Accounting Models and Measured Value

**Method overview — underlying technical case withheld.**

## Research question

Different views of the same system can measure different quantities: principal, claimable rewards, idle balances, shares or a conservative reported valuation. A numerical difference is a starting question, not a deficit by itself.

## Approach

I first identify the units, update timing and components of each view. I reconstruct the intended identity, then compare it with before-and-after measurements along the relevant call path. Reverts and optional read errors remain visible rather than becoming zero-valued measurements.

## Counter-tests and interpretation

A useful control compares the recomputed identity and the reported metric under the same state snapshot. The explanation must account for legitimate yield, fees, rounding and transfers, and state which quantities were not measured.

## Evidence boundary

This illustrates accounting decomposition and bounded negative-result analysis. It does not provide a protocol-specific formula, claim a reproduced vulnerability or establish a protocol’s general safety. This page describes a method; it is not independently verifiable execution evidence.

For a separate executable teaching example of the workflow, see the [Refund Escrow lab](../educational-demos/refund-escrow/README.md). It is independent of this withheld study.

[All investigation overviews](README.md) · [Testing approach](../research-tools-testing/testing.md) · [Portfolio](../README.md)
