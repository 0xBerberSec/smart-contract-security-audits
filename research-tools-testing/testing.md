# Testing and Interpretation

## Build the experiment

I start from one security property and the state needed to test it. I pin the relevant contract revision, identify dependencies, and document the actors and permissions. Any mock or manual state change is part of the test setup. I use protocol fixtures when they preserve the behavior being examined; substitutions narrow what the result establishes.

## Measure execution

I record the command, environment and relevant inputs. Fork-based tests identify the chain, block and RPC requirements. I compare balances and state before and after execution, then retain the assertion, transaction outcome and useful traces. Local transactions and administrative impersonation are labeled explicitly.

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

I compare each hypothesis against a baseline and an appropriate control. Where relevant, I vary call ordering or permissions and trace the sources of any apparent profit. I retain negative results with the unmeasured conditions stated explicitly; a bounded counter-test is not a general safety guarantee.

## Available public examples

- **Educational executable example:** [Refund Escrow](../educational-demos/refund-escrow/README.md), with original synthetic contracts and [actual outputs](../educational-demos/refund-escrow/EVIDENCE.md).
- **Public real-protocol case studies:** [Mezo](../protocol-investigations/mezo-independent-research.md), with selected Pool and Router local-fork experiments and a [minimal reproduction harness](../protocol-investigations/mezo-reproduction/README.md). The historical quote assertions are conditional, and old UniversalRouter London PASS labels were traced to `NotActivated`. Separate [complementary controls](../protocol-investigations/mezo-reproduction/supplemental/README.md) measure successful quotes and verify exact errors in an explicitly qualified execution environment.
- **Private third-party investigations:** selected [generic overviews](../protocol-investigations/README.md#generic-methodological-overviews) explain reasoning while underlying sensitive dossiers remain withheld.

Historical results and fresh clean-copy executions are labeled separately. These examples do not establish a confirmed exploitable Mezo finding or certify protocol security.

[Methodology](../methodology/README.md) · [Evidence classes](../docs/SEVERITY_AND_CLAIMS.md) · [Portfolio](../README.md)
