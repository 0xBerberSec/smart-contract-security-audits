# Testing and Interpretation

## Build the experiment

Start with one property and its concrete state. Identify the contract revision and dependencies, choose actors and privileges, and make each mock or state mutation explicit. Prefer protocol fixtures when they preserve the relevant behavior; a substituted dependency limits the conclusion.

## Measure execution

Capture the command, environment and relevant inputs. For a fork, record the chain, block and RPC requirements. Compare before-and-after balances and state, and retain the assertion, transaction outcome and useful trace. Local transactions and administrative impersonation must be described as such.

## Interpret PASS and FAIL

| Observation | Interpretation |
| --- | --- |
| Safety assertion passes | The property held for the exercised case, not every state |
| Anomaly-demonstration assertion passes | A local behavior was reproduced under the fixture’s assumptions |
| Expected-panic test passes | The expected failure occurred; normal execution did not succeed |
| Assertion fails | Investigate a property violation, test error or invalid assumption |
| RPC/setup fails before the body | The property was not tested |
| Snapshot exists | Some execution occurred; final success is not established by that alone |
| Fuzzer reports no crash | Assess corpus growth, reachable actions and coverage before drawing conclusions |

## Challenge the interpretation

Use a baseline and an appropriate control. Vary ordering or permissions, trace all contributors to an apparent profit, and state what remains unmeasured. Preserve negative results and distinguish a bounded counter-test from a safety guarantee.

## Available public examples

The [educational lab](../educational-demos/refund-escrow/README.md) demonstrates this reasoning with executable original code and [actual outputs](../educational-demos/refund-escrow/EVIDENCE.md). The [investigation overviews](../protocol-investigations/README.md) remain generic descriptions of private third-party work; no private test bodies or protocol exploit are released. Historical results are never relabeled as new executions.

[Methodology](../methodology/README.md) · [Evidence classes](../docs/SEVERITY_AND_CLAIMS.md) · [Portfolio](../README.md)
