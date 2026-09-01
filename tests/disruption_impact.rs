use supply_chain_control_plane::{
    EvidenceReference, GraphError, SupplyEvent, SupplyEventKind, SupplyGraph, SupplyNodeKind,
};

fn evidence() -> EvidenceReference {
    EvidenceReference::new("source-record-001", "erp://shipment/alpha")
        .expect("fixture evidence is valid")
}

fn dependency_evidence(source_record_key: &str) -> EvidenceReference {
    EvidenceReference::new(source_record_key, &format!("erp://dependency/{source_record_key}"))
        .expect("fixture dependency evidence is valid")
}

#[test]
fn disruption_reachability_is_directional_deterministic_cycle_safe_and_explainable() {
    let mut graph = SupplyGraph::new();
    graph
        .add_node("supplier-alpha", SupplyNodeKind::Supplier)
        .unwrap();
    graph
        .add_node("facility-east", SupplyNodeKind::Facility)
        .unwrap();
    graph.add_node("item-widget", SupplyNodeKind::Item).unwrap();
    graph.add_node("order-1042", SupplyNodeKind::Order).unwrap();

    graph
        .add_dependency(
            "supplier-alpha",
            "facility-east",
            dependency_evidence("edge-supplier-facility"),
        )
        .unwrap();
    graph
        .add_dependency(
            "facility-east",
            "item-widget",
            dependency_evidence("edge-facility-item"),
        )
        .unwrap();
    graph
        .add_dependency(
            "item-widget",
            "order-1042",
            dependency_evidence("edge-item-order"),
        )
        .unwrap();
    graph
        .add_dependency(
            "order-1042",
            "facility-east",
            dependency_evidence("edge-order-facility-cycle"),
        )
        .unwrap();

    graph
        .record_event(SupplyEvent::new(
            "event-supplier-stop-001",
            "supplier-alpha",
            SupplyEventKind::ProductionStopped,
            "2026-09-01T12:30:00Z",
            evidence(),
        )
        .unwrap())
        .unwrap();

    let impact = graph
        .downstream_impacts("event-supplier-stop-001")
        .unwrap();

    assert_eq!(
        impact
            .iter()
            .map(|record| (record.node_key(), record.hop_count()))
            .collect::<Vec<_>>(),
        vec![
            ("facility-east", 1),
            ("item-widget", 2),
            ("order-1042", 3),
        ]
    );
    assert_eq!(
        impact[2].dependency_path(),
        ["supplier-alpha", "facility-east", "item-widget", "order-1042"]
    );
    assert_eq!(
        impact[2]
            .dependency_evidence()
            .iter()
            .map(EvidenceReference::source_record_key)
            .collect::<Vec<_>>(),
        vec![
            "edge-supplier-facility",
            "edge-facility-item",
            "edge-item-order",
        ]
    );
    assert_eq!(
        impact[2].event_evidence().source_record_key(),
        "source-record-001"
    );
    assert_eq!(
        impact[2].event_evidence().source_locator(),
        "erp://shipment/alpha"
    );
}

#[test]
fn graph_rejects_unproven_or_structurally_invalid_state() {
    assert!(EvidenceReference::new(" ", "erp://record/1").is_err());
    assert!(EvidenceReference::new("record-1", " ").is_err());

    let mut graph = SupplyGraph::new();
    graph
        .add_node("supplier-alpha", SupplyNodeKind::Supplier)
        .unwrap();
    assert_eq!(
        graph.add_node("supplier-alpha", SupplyNodeKind::Supplier),
        Err(GraphError::DuplicateNode("supplier-alpha".into()))
    );
    assert_eq!(
        graph.add_dependency(
            "supplier-alpha",
            "missing-node",
            dependency_evidence("edge-missing"),
        ),
        Err(GraphError::UnknownNode("missing-node".into()))
    );
    assert_eq!(
        graph.add_dependency(
            "supplier-alpha",
            "supplier-alpha",
            dependency_evidence("edge-self"),
        ),
        Err(GraphError::SelfDependency("supplier-alpha".into()))
    );

    let event = SupplyEvent::new(
        "event-001",
        "missing-node",
        SupplyEventKind::ShipmentDelayed,
        "2026-09-01T12:31:00Z",
        evidence(),
    )
    .unwrap();
    assert_eq!(
        graph.record_event(event),
        Err(GraphError::UnknownNode("missing-node".into()))
    );
}

#[test]
fn duplicate_edges_and_events_are_not_silently_overwritten() {
    let mut graph = SupplyGraph::new();
    graph
        .add_node("supplier-alpha", SupplyNodeKind::Supplier)
        .unwrap();
    graph
        .add_node("facility-east", SupplyNodeKind::Facility)
        .unwrap();
    graph
        .add_dependency(
            "supplier-alpha",
            "facility-east",
            dependency_evidence("edge-original"),
        )
        .unwrap();
    assert_eq!(
        graph.add_dependency(
            "supplier-alpha",
            "facility-east",
            dependency_evidence("edge-replacement"),
        ),
        Err(GraphError::DuplicateDependency {
            upstream_node: "supplier-alpha".into(),
            downstream_node: "facility-east".into(),
        })
    );

    let event = SupplyEvent::new(
        "event-001",
        "supplier-alpha",
        SupplyEventKind::InventoryUnavailable,
        "2026-09-01T12:32:00Z",
        evidence(),
    )
    .unwrap();
    graph.record_event(event.clone()).unwrap();
    assert_eq!(
        graph.record_event(event),
        Err(GraphError::DuplicateEvent("event-001".into()))
    );
}
