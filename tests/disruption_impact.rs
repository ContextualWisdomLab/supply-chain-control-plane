use supply_chain_control_plane::{
    EvidenceReference, GraphError, SupplyEvent, SupplyEventKind, SupplyGraph, SupplyNodeKind,
    UpsertOutcome,
};

fn evidence() -> EvidenceReference {
    EvidenceReference::new("source-record-001", "erp://shipment/alpha")
        .expect("fixture evidence is valid")
}

fn dependency_evidence(source_record_key: &str) -> EvidenceReference {
    EvidenceReference::new(
        source_record_key,
        &format!("erp://dependency/{source_record_key}"),
    )
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
        .record_event(
            SupplyEvent::new(
                "event-supplier-stop-001",
                "supplier-alpha",
                SupplyEventKind::ProductionStopped,
                "2026-09-01T12:30:00Z",
                evidence(),
            )
            .unwrap(),
        )
        .unwrap();

    let impact = graph.downstream_impacts("event-supplier-stop-001").unwrap();

    assert_eq!(
        impact
            .iter()
            .map(|record| (record.node_key(), record.hop_count()))
            .collect::<Vec<_>>(),
        vec![("facility-east", 1), ("item-widget", 2), ("order-1042", 3),]
    );
    assert_eq!(
        impact[2].dependency_path(),
        [
            "supplier-alpha",
            "facility-east",
            "item-widget",
            "order-1042"
        ]
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

#[test]
fn item_level_upserts_are_idempotent_for_exact_replays_and_fail_on_conflicts() {
    let mut graph = SupplyGraph::new();

    assert_eq!(
        graph
            .upsert_node(" supplier-alpha ", SupplyNodeKind::Supplier)
            .unwrap(),
        UpsertOutcome::Inserted
    );
    assert_eq!(
        graph
            .upsert_node("supplier-alpha", SupplyNodeKind::Supplier)
            .unwrap(),
        UpsertOutcome::Unchanged
    );
    assert_eq!(
        graph.upsert_node("supplier-alpha", SupplyNodeKind::Facility),
        Err(GraphError::ConflictingNode("supplier-alpha".into()))
    );

    graph
        .upsert_node("facility-east", SupplyNodeKind::Facility)
        .unwrap();
    let edge_evidence = dependency_evidence("edge-replay");
    assert_eq!(
        graph
            .upsert_dependency("supplier-alpha", "facility-east", edge_evidence.clone())
            .unwrap(),
        UpsertOutcome::Inserted
    );
    assert_eq!(
        graph
            .upsert_dependency(" supplier-alpha ", " facility-east ", edge_evidence.clone())
            .unwrap(),
        UpsertOutcome::Unchanged
    );
    assert_eq!(
        graph.upsert_dependency(
            "supplier-alpha",
            "facility-east",
            dependency_evidence("edge-conflict"),
        ),
        Err(GraphError::ConflictingDependency {
            upstream_node: "supplier-alpha".into(),
            downstream_node: "facility-east".into(),
        })
    );

    let event = SupplyEvent::new(
        "event-replay",
        "supplier-alpha",
        SupplyEventKind::ProductionStopped,
        "2026-09-01T13:00:00Z",
        evidence(),
    )
    .unwrap();
    assert_eq!(
        graph.upsert_event(event.clone()).unwrap(),
        UpsertOutcome::Inserted
    );
    assert_eq!(graph.upsert_event(event).unwrap(), UpsertOutcome::Unchanged);
    let conflicting_event = SupplyEvent::new(
        "event-replay",
        "supplier-alpha",
        SupplyEventKind::ProductionStopped,
        "2026-09-01T13:01:00Z",
        evidence(),
    )
    .unwrap();
    assert_eq!(
        graph.upsert_event(conflicting_event),
        Err(GraphError::ConflictingEvent("event-replay".into()))
    );

    let impacts = graph.downstream_impacts("event-replay").unwrap();
    assert_eq!(impacts.len(), 1);
    assert_eq!(impacts[0].node_key(), "facility-east");
    assert_eq!(
        impacts[0].dependency_evidence(),
        std::slice::from_ref(&edge_evidence)
    );
}

#[test]
fn upsert_validation_preserves_graph_invariants() {
    let mut graph = SupplyGraph::new();
    graph
        .upsert_node("supplier-alpha", SupplyNodeKind::Supplier)
        .unwrap();

    assert_eq!(
        graph.upsert_node(" ", SupplyNodeKind::Supplier),
        Err(GraphError::BlankField("supply_node_key"))
    );
    assert_eq!(
        graph.upsert_dependency(
            "supplier-alpha",
            "supplier-alpha",
            dependency_evidence("edge-self-upsert"),
        ),
        Err(GraphError::SelfDependency("supplier-alpha".into()))
    );
    assert_eq!(
        graph.upsert_dependency(
            "missing-upstream",
            "supplier-alpha",
            dependency_evidence("edge-missing-upstream"),
        ),
        Err(GraphError::UnknownNode("missing-upstream".into()))
    );
    assert_eq!(
        graph.upsert_dependency(
            "supplier-alpha",
            "missing-downstream",
            dependency_evidence("edge-missing-downstream"),
        ),
        Err(GraphError::UnknownNode("missing-downstream".into()))
    );

    let missing_node_event = SupplyEvent::new(
        "event-missing-node",
        "missing-downstream",
        SupplyEventKind::InventoryUnavailable,
        "2026-09-01T13:02:00Z",
        evidence(),
    )
    .unwrap();
    assert_eq!(
        graph.upsert_event(missing_node_event),
        Err(GraphError::UnknownNode("missing-downstream".into()))
    );
}

#[test]
fn equal_length_routes_choose_a_stable_shortest_path_with_aligned_evidence() {
    let mut graph = SupplyGraph::new();
    for (node_key, node_kind) in [
        ("supplier-source", SupplyNodeKind::Supplier),
        ("facility-beta", SupplyNodeKind::Facility),
        ("facility-alpha", SupplyNodeKind::Facility),
        ("order-target", SupplyNodeKind::Order),
    ] {
        graph.add_node(node_key, node_kind).unwrap();
    }

    graph
        .add_dependency(
            "supplier-source",
            "facility-beta",
            dependency_evidence("edge-source-beta"),
        )
        .unwrap();
    graph
        .add_dependency(
            "supplier-source",
            "facility-alpha",
            dependency_evidence("edge-source-alpha"),
        )
        .unwrap();
    graph
        .add_dependency(
            "facility-beta",
            "order-target",
            dependency_evidence("edge-beta-order"),
        )
        .unwrap();
    graph
        .add_dependency(
            "facility-alpha",
            "order-target",
            dependency_evidence("edge-alpha-order"),
        )
        .unwrap();
    graph
        .record_event(
            SupplyEvent::new(
                "event-source-stop",
                "supplier-source",
                SupplyEventKind::ProductionStopped,
                "2026-09-01T12:40:00Z",
                evidence(),
            )
            .unwrap(),
        )
        .unwrap();

    let impacts = graph.downstream_impacts("event-source-stop").unwrap();
    let target = impacts
        .iter()
        .find(|impact| impact.node_key() == "order-target")
        .expect("target is reachable");

    assert_eq!(
        target.dependency_path(),
        ["supplier-source", "facility-alpha", "order-target"]
    );
    assert_eq!(
        target
            .dependency_evidence()
            .iter()
            .map(EvidenceReference::source_record_key)
            .collect::<Vec<_>>(),
        vec!["edge-source-alpha", "edge-alpha-order"]
    );
    assert!(
        impacts
            .iter()
            .all(|impact| impact.dependency_evidence().len() == impact.hop_count())
    );
}

#[test]
fn admitted_values_are_trimmed_accessible_and_unknown_events_fail_closed() {
    let evidence = EvidenceReference::new(" source-record-9 ", " evidence://record/9 ").unwrap();
    assert_eq!(evidence.source_record_key(), "source-record-9");
    assert_eq!(evidence.source_locator(), "evidence://record/9");

    let event = SupplyEvent::new(
        " event-9 ",
        " shipment-9 ",
        SupplyEventKind::OrderAtRisk,
        " 2026-09-01T12:45:00Z ",
        evidence.clone(),
    )
    .unwrap();
    assert_eq!(event.event_key(), "event-9");
    assert_eq!(event.affected_node(), "shipment-9");
    assert_eq!(event.event_kind(), SupplyEventKind::OrderAtRisk);
    assert_eq!(event.observed_at(), "2026-09-01T12:45:00Z");
    assert_eq!(event.evidence(), &evidence);

    let mut graph = SupplyGraph::new();
    graph
        .add_node("shipment-9", SupplyNodeKind::Shipment)
        .unwrap();
    graph
        .add_node("inventory-9", SupplyNodeKind::InventoryPosition)
        .unwrap();
    assert_eq!(
        graph.node_kind("shipment-9"),
        Some(SupplyNodeKind::Shipment)
    );
    assert_eq!(
        graph.node_kind("inventory-9"),
        Some(SupplyNodeKind::InventoryPosition)
    );
    assert_eq!(graph.node_kind("missing"), None);
    assert_eq!(
        graph.downstream_impacts("missing-event"),
        Err(GraphError::UnknownEvent("missing-event".into()))
    );
}

#[test]
fn lookup_boundaries_normalize_keys_consistently_with_admission() {
    let mut graph = SupplyGraph::new();
    graph
        .add_node(" shipment-lookup ", SupplyNodeKind::Shipment)
        .unwrap();
    graph
        .record_event(
            SupplyEvent::new(
                " event-lookup ",
                " shipment-lookup ",
                SupplyEventKind::ShipmentDelayed,
                "2026-09-01T12:46:00Z",
                evidence(),
            )
            .unwrap(),
        )
        .unwrap();

    assert_eq!(
        graph.node_kind(" shipment-lookup "),
        Some(SupplyNodeKind::Shipment)
    );
    assert_eq!(
        graph.downstream_impacts(" event-lookup ").unwrap(),
        Vec::new()
    );
}

#[test]
fn graph_errors_have_stable_operator_facing_messages() {
    let errors = [
        (
            GraphError::BlankField("field_name"),
            "field_name must not be blank",
        ),
        (
            GraphError::DuplicateNode("node-a".into()),
            "node already exists: node-a",
        ),
        (
            GraphError::DuplicateEvent("event-a".into()),
            "event already exists: event-a",
        ),
        (
            GraphError::DuplicateDependency {
                upstream_node: "node-a".into(),
                downstream_node: "node-b".into(),
            },
            "dependency already exists: node-a -> node-b",
        ),
        (
            GraphError::ConflictingNode("node-a".into()),
            "node replay conflicts with existing value: node-a",
        ),
        (
            GraphError::ConflictingEvent("event-a".into()),
            "event replay conflicts with existing value: event-a",
        ),
        (
            GraphError::ConflictingDependency {
                upstream_node: "node-a".into(),
                downstream_node: "node-b".into(),
            },
            "dependency replay conflicts with existing value: node-a -> node-b",
        ),
        (
            GraphError::UnknownNode("node-a".into()),
            "unknown node: node-a",
        ),
        (
            GraphError::UnknownEvent("event-a".into()),
            "unknown event: event-a",
        ),
        (
            GraphError::SelfDependency("node-a".into()),
            "self dependency is forbidden: node-a",
        ),
    ];

    for (error, expected) in errors {
        assert_eq!(error.to_string(), expected);
    }
}
