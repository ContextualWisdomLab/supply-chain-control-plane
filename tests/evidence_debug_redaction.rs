use supply_chain_control_plane::EvidenceReference;

#[test]
fn evidence_debug_output_redacts_the_source_locator() {
    let secret_locator = "https://internal.example/evidence/order-42?token=do-not-log";
    let evidence = EvidenceReference::new("source-record-42", secret_locator)
        .expect("synthetic evidence reference should be valid");

    let debug_output = format!("{evidence:?}");

    assert!(debug_output.contains("source-record-42"));
    assert!(debug_output.contains("<redacted>"));
    assert!(!debug_output.contains(secret_locator));
    assert!(!debug_output.contains("do-not-log"));
}
