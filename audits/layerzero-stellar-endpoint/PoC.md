# Proof of Concept

[Back to LayerZero report](./README.md)

## Overview

The following test demonstrates that a ULN configuration can pass validation while later making `quote()` fail under normal Soroban budget limits because `required_dvns` and `optional_dvns` are iterated as a combined set.

## Environment

- Protocol: LayerZero Stellar Endpoint
- Component: ULN302
- Language/runtime: Rust / Soroban
- Test target: `uln302`

## Steps

Add the test to:

```text
contracts/protocol/stellar/contracts/message-libs/uln-302/src/tests/send_uln302/quote.rs
```

```rust
#[test]
#[should_panic(expected = "ExceededLimit")]
fn test_valid_combined_required_and_optional_dvn_config_makes_quote_exceed_budget() {
    let mut setup = setup();

    // Use unlimited budget only for fixture creation.
    setup.env.budget().reset_unlimited();

    let oapp = Address::generate(&setup.env);

    let mut required_dvns = Vec::new(&setup.env);
    let mut optional_dvns = Vec::new(&setup.env);

    // Each list passes per-list validation, but the combined runtime set is too large.
    for _ in 0..14 {
        required_dvns.push_back(setup.register_dvn(1));
        optional_dvns.push_back(setup.register_dvn(1));
    }

    let executor = setup.register_executor(1);
    setup.treasury.set_native_fee(&0);
    setup.treasury.set_zro_fee(&0);

    // Restore normal budget before setting the ULN config.
    setup.env.budget().reset_default();

    setup.set_default_send_uln_config(
        DST_EID,
        UlnConfig::new(CONFIRMATIONS, &required_dvns, &optional_dvns, 1),
    );

    // Executor config setup is not the subject of this test.
    setup.env.budget().reset_unlimited();
    setup.set_default_executor_config(DST_EID, ExecutorConfig::new(MAX_MESSAGE_SIZE, &executor));

    let packet = OutboundPacket {
        nonce: 1,
        src_eid: 1,
        sender: oapp,
        dst_eid: DST_EID,
        receiver: BytesN::from_array(&setup.env, &[0u8; 32]),
        guid: BytesN::from_array(&setup.env, &[1u8; 32]),
        message: Bytes::from_array(&setup.env, &[1, 2, 3, 4]),
    };

    let options = create_type3_options(
        &setup.env,
        &vec![&setup.env, required_dvns.get_unchecked(0)],
        true,
    );

    let pay_in_zro = false;

    // Restore normal budget before runtime execution.
    setup.env.budget().reset_default();

    let _ = setup.uln302.quote(&packet, &options, &pay_in_zro);
}
```

Run with:

```bash
cargo test -p uln302 --features testutils test_valid_combined_required_and_optional_dvn_config_makes_quote_exceed_budget -- --nocapture
```

## Expected Result

Validation should reject configurations that cannot execute under normal runtime limits.

## Observed Result

Observed result:

```text
HostError: Error(Budget, ExceededLimit)
```

This demonstrates that:

- The combined DVN configuration is accepted by validation
- Failure occurs in the normal runtime `quote()` path
- No malicious DVN is required
- No invalid DVN is required
- No duplicate DVN is required
- The root cause is the combined iteration over `required_dvns` and `optional_dvns`

## Conclusion

The PoC demonstrates a validation/runtime mismatch: a configuration can be accepted by validation but fail deterministically in the normal runtime path.

[Back to LayerZero report](./README.md)
