# Research process and AI contribution

This artifact was prepared for 0xBerberSec with Codex assistance on 30 October 2026. It demonstrates a workflow; it is not a claim that this educational vulnerability was found during earlier protocol audits or without assistance.

| Stage | Concrete work in this example | Review artifact |
| --- | --- | --- |
| Value flow | Define deposits, refundable credits, pooled backing and recipients | [Analysis](ANALYSIS.md) |
| Invariant | Tie ETH backing to the complete tracked actor set | [Fixed invariant and handler](test/FixedInvariant.t.sol) |
| Hypothesis | Introduce the known external-call-before-consumption bug | [Two implementations](src/RefundEscrow.sol) |
| Targeted test | Bound the callback and measure entitlement, receipts and deficit | [Tests](test/RefundEscrow.t.sol) |
| Counter-tests | Remove callback or other funding; exercise ordinary and unauthorized callers | [Tests](test/RefundEscrow.t.sol) |
| Correction | Move consumption before the call; check nested rejection and rollback | [Source](src/RefundEscrow.sol), [tests](test/RefundEscrow.t.sol) |
| Economic interpretation | Attribute the extra ETH to victim backing, excluding gas | [Accounting table](ANALYSIS.md) |
| Challenge the harness | Make the vulnerable safety assertion fail separately; use fail-on-revert for the bounded fixed campaign | [Counterexample](counterexamples/VulnerableSolvency.t.sol), [configuration](foundry.toml) |
| Evidence | Execute the commands and retain source hashes, outputs and exit codes | [Execution record](evidence/review-2026-10-30/RUN.json) |

Codex assisted the educational design, Solidity implementation, test construction, control selection, execution tooling and documentation. Generated code and explanations were treated as drafts requiring compilation, assertion review and actual execution. The recorded commands provide evidence of this artifact's behavior, not external validation of the researcher or a substitute for peer review.

An initial local harness run emitted warnings because optional Foundry invariant-configuration accessors were absent. Explicit empty accessors were added in the small local support contract, and the final captured run completed without those warnings. The support exposes the configuration ABI; no forge-std code or external Solidity package is required.

The proposed workflow is transparent about a useful boundary: a test asserting an anomalous outcome can pass, while the separate safety-property test must fail on the deliberately vulnerable contract. The replay runner checks the exact failure reason, so an arbitrary setup error cannot stand in for that counterexample.

This is a new original teaching specification using a standard vulnerability class. No private protocol code, parameter, exploit sequence or test was copied into this demonstration. Private investigations remain subject to a separate disclosure review.

Tool references: [Foundry](https://github.com/foundry-rs/foundry), [invariant configuration ABI](https://github.com/foundry-rs/forge-std/blob/master/src/StdInvariant.sol). These are references, not runtime library dependencies.

[README](README.md) · [Actual results](EVIDENCE.md)
