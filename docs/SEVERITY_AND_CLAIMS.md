# Evidence Classes and Claim Discipline

| Class | Required basis | Appropriate claim |
| --- | --- | --- |
| Educational demonstration | Original synthetic protocol and recorded local execution | A known vulnerability class is reproduced in a synthetic protocol; this is not a third-party discovery or contest result |
| Officially validated finding | Attributable external ruling or official result | The claim is limited to the accepted result identified in the official source |
| Independent finding with local evidence | Identifiable test, setup and result supporting a specific behavior | The observed behavior and its prerequisites are documented without implying external endorsement |
| Historical documented result | A saved execution record or a clearly identified historical report | Historical reports are distinguished from raw execution evidence and fresh reproductions |
| Observed security behavior | A measured or documented anomaly | The observation is described with any unresolved exploitability or impact conditions |
| Unverified hypothesis | A plausible idea without sufficient demonstration | The hypothesis remains unresolved rather than being presented as a confirmed vulnerability |
| Refuted / expected / duplicate | Concrete rebuttal, control, design explanation or duplication basis | The recorded control or source comparison explains why the vulnerability interpretation was rejected |
| Source-limited / blocked research | Missing source, dependency, fixture, RPC or access | The record identifies the property that could not be tested or verified |

These labels describe evidence, not a severity ladder. A result can be locally demonstrated, historically reported, unverified externally and withheld from disclosure at the same time.

A passing exploit-style test is not automatically a realistic exploitation path. A checkout HEAD is not automatically the audited revision. A reverted or skipped scenario is not successful-path coverage. A test that finds no issue within a limited scope cannot establish the security of an entire protocol.

If an original submission cannot be located, I treat its status as **unknown**; I do not conclude that no submission was ever made. Prior publication by a researcher is not sponsor consent. An aggregate report is not proof of individual credit. Local severity estimates are not official ratings.

[Validation process](../methodology/validation.md) · [Evidence register](EVIDENCE.md) · [Portfolio](../README.md)
