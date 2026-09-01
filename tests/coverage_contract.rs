use supply_chain_control_plane::{
    EvidenceReference, GraphError, SupplyEvent, SupplyEventKind, SupplyGraph, SupplyNodeKind,
};

fn evidence(record_key: &str) -> EvidenceReference {
    EvidenceReference::new(record_key, &format!("source://{record_key}"))
        .expect("fixture evidence must be valid")
}

#[test]
fn constructors_identify_each_blank_semantic_field() {
    let event_evidence = evidence("event-evidence");
    assert_eq!(
        SupplyEvent::new(
            " ",
            "node-a",
            SupplyEventKind::OrderAtRisk,
            "2026-09-01T12:00:00Z",
            event_evidence.clone(),
        ),
        Err(GraphError::BlankField("supply_event_key"))
    );
    assert_eq!(
        SupplyEvent::new(
            "event-a",
            " ",
            SupplyEventKind::OrderAtRisk,
            "2026-09-01T12:00:00Z",
            event_evidence.clone(),
        ),
        Err(GraphError::BlankField("affected_supply_node_key"))
    );
    assert_eq!(
        SupplyEvent::new(
            "event-a",
            "node-a",
            SupplyEventKind::OrderAtRisk,
            " ",
            event_evidence,
        ),
        Err(GraphError::BlankField("observed_at"))
    );

    let mut graph = SupplyGraph::new();
    assert_eq!(
        graph.add_node(" ", SupplyNodeKind::Supplier),
        Err(GraphError::BlankField("supply_node_key"))
    );
    assert_eq!(
        graph.add_dependency(" ", "node-b", evidence("edge-one")),
        Err(GraphError::BlankField("upstream_supply_node_key"))
    );
    assert_eq!(
        graph.add_dependency("node-a", " ", evidence("edge-two")),
        Err(GraphError::BlankField("downstream_supply_node_key"))
    );
}

#[test]
fn unknown_upstream_and_leaf_event_paths_fail_closed() {
    let mut graph = SupplyGraph::new();
    graph.add_node("leaf", SupplyNodeKind::Order).unwrap();

    assert_eq!(
        graph.add_dependency("missing", "leaf", evidence("edge-missing-upstream")),
        Err(GraphError::UnknownNode("missing".to_owned()))
    );

    graph
        .record_event(
            SupplyEvent::new(
                "event-leaf",
                "leaf",
                SupplyEventKind::OrderAtRisk,
                "2026-09-01T12:01:00Z",
                evidence("event-leaf-evidence"),
            )
            .unwrap(),
        )
        .unwrap();

    assert!(graph.downstream_impacts("event-leaf").unwrap().is_empty());
}
