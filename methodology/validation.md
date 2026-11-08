# Finding Validation

I separate four questions:

1. **Evidence:** what did the test or observation actually demonstrate?
2. **Security:** does it violate the intended property under a credible threat model?
3. **External status:** was this individual finding submitted and officially accepted?
4. **Disclosure:** is this exact technical content authorized for release?

## Review sequence

For each candidate finding, I check the source revision and whether the fixture reflects the behavior under review. I inspect the assertions and execution outcome, including whether the test body ran. I identify required permissions, configuration and reachable state, trace the movement of value, and test alternative explanations. I compare prior work by root cause and version before assessing novelty.

An expected revert, privileged simulation or successful test alone does not establish an exploitable vulnerability. A local proof can be useful while remaining conditional on a mock, source gap or operational assumption.

## Conclusions

I keep official validation, independent local evidence, historical results, observations, hypotheses, rejected paths and blocked research separate. I record submission status independently from disclosure permission. When I cannot locate a submission receipt, the status remains unknown; I do not assume the submission never existed.

When I reject a hypothesis, I record the counterevidence and what new observation would justify reopening it. Similarity to a known issue prompts a comparison of the underlying cause; it does not automatically establish duplication.

[Evidence classes](../docs/SEVERITY_AND_CLAIMS.md) · [Disclosure policy](../docs/DISCLOSURE.md) · [Portfolio](../README.md)
