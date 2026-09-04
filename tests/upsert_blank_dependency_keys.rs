use supply_chain_control_plane::{EvidenceReference, GraphError, SupplyGraph, SupplyNodeKind};

fn dependency_evidence() -> EvidenceReference {
    EvidenceReference::new("edge-blank-key", "erp://dependency/blank-key")
        .expect("fixture dependency evidence is valid")
}

#[test]
fn upsert_dependency_rejects_blank_upstream_key() {
    let mut graph = SupplyGraph::new();
    graph
        .upsert_node("facility-east", SupplyNodeKind::Facility)
        .unwrap();

    assert_eq!(
        graph.upsert_dependency(" ", "facility-east", dependency_evidence()),
        Err(GraphError::BlankField("upstream_supply_node_key"))
    );
}

#[test]
fn upsert_dependency_rejects_blank_downstream_key() {
    let mut graph = SupplyGraph::new();
    graph
        .upsert_node("supplier-alpha", SupplyNodeKind::Supplier)
        .unwrap();

    assert_eq!(
        graph.upsert_dependency("supplier-alpha", " ", dependency_evidence()),
        Err(GraphError::BlankField("downstream_supply_node_key"))
    );
}
