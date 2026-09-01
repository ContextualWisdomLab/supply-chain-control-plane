//! Evidence-first supply-chain disruption impact domain core.
//!
//! The core deliberately reports causal reachability, not probabilistic severity or
//! heuristic risk weights. Quantitative scoring belongs behind an explicitly cited
//! model contract once validated evidence exists.
#![forbid(unsafe_code)]
#![deny(missing_docs)]

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::error::Error;
use std::fmt::{Display, Formatter};

/// Result of an item-level idempotent upsert command.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UpsertOutcome {
    /// The semantic item did not exist and was inserted.
    Inserted,
    /// An identical semantic item already existed, so no state changed.
    Unchanged,
}

/// Failures that protect graph and evidence invariants.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GraphError {
    /// A required semantic field contained no non-whitespace text.
    BlankField(&'static str),
    /// A node key is already present.
    DuplicateNode(String),
    /// An event key is already present.
    DuplicateEvent(String),
    /// A dependency edge is already present.
    DuplicateDependency {
        /// Upstream node key.
        upstream_node: String,
        /// Downstream node key.
        downstream_node: String,
    },
    /// A node replay reused a semantic key with a different value.
    ConflictingNode(String),
    /// An event replay reused a semantic key with different immutable content.
    ConflictingEvent(String),
    /// A dependency replay reused the edge identity with different immutable evidence.
    ConflictingDependency {
        /// Upstream node key.
        upstream_node: String,
        /// Downstream node key.
        downstream_node: String,
    },
    /// A referenced node does not exist.
    UnknownNode(String),
    /// A referenced event does not exist.
    UnknownEvent(String),
    /// A node cannot depend on itself directly.
    SelfDependency(String),
}

impl Display for GraphError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BlankField(field) => write!(formatter, "{field} must not be blank"),
            Self::DuplicateNode(key) => write!(formatter, "node already exists: {key}"),
            Self::DuplicateEvent(key) => write!(formatter, "event already exists: {key}"),
            Self::DuplicateDependency {
                upstream_node,
                downstream_node,
            } => write!(
                formatter,
                "dependency already exists: {upstream_node} -> {downstream_node}"
            ),
            Self::ConflictingNode(key) => {
                write!(formatter, "node replay conflicts with existing value: {key}")
            }
            Self::ConflictingEvent(key) => {
                write!(formatter, "event replay conflicts with existing value: {key}")
            }
            Self::ConflictingDependency {
                upstream_node,
                downstream_node,
            } => write!(
                formatter,
                "dependency replay conflicts with existing value: {upstream_node} -> {downstream_node}"
            ),
            Self::UnknownNode(key) => write!(formatter, "unknown node: {key}"),
            Self::UnknownEvent(key) => write!(formatter, "unknown event: {key}"),
            Self::SelfDependency(key) => write!(formatter, "self dependency is forbidden: {key}"),
        }
    }
}

impl Error for GraphError {}

fn required_text(value: &str, field: &'static str) -> Result<String, GraphError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(GraphError::BlankField(field));
    }
    Ok(trimmed.to_owned())
}

/// A supply-network node category in the disruption-impact bounded context.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SupplyNodeKind {
    /// A supplier organization or supplier scope.
    Supplier,
    /// A physical or logical operating facility.
    Facility,
    /// A material, component, or sellable item.
    Item,
    /// An inventory position whose availability matters to fulfillment.
    InventoryPosition,
    /// A shipment or transport movement.
    Shipment,
    /// A customer or internal fulfillment order.
    Order,
}

/// An observed supply event category.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SupplyEventKind {
    /// Production stopped at the affected node.
    ProductionStopped,
    /// Inventory became unavailable at the affected node.
    InventoryUnavailable,
    /// A shipment is delayed relative to its committed movement.
    ShipmentDelayed,
    /// An order has direct evidence of fulfillment risk.
    OrderAtRisk,
}

/// Immutable evidence pointer for an observed event or dependency fact.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EvidenceReference {
    source_record_key: String,
    source_locator: String,
}

impl EvidenceReference {
    /// Creates an evidence reference from a stable source-record key and source locator.
    pub fn new(source_record_key: &str, source_locator: &str) -> Result<Self, GraphError> {
        Ok(Self {
            source_record_key: required_text(source_record_key, "source_record_key")?,
            source_locator: required_text(source_locator, "source_locator")?,
        })
    }

    /// Returns the stable source-record key.
    #[must_use]
    pub fn source_record_key(&self) -> &str {
        &self.source_record_key
    }

    /// Returns the source locator used for evidence drill-through.
    #[must_use]
    pub fn source_locator(&self) -> &str {
        &self.source_locator
    }
}

/// A disruption-relevant event with explicit observation evidence.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SupplyEvent {
    event_key: String,
    affected_node: String,
    event_kind: SupplyEventKind,
    observed_at: String,
    evidence: EvidenceReference,
}

impl SupplyEvent {
    /// Creates an event after validating semantic identity and observation time text.
    pub fn new(
        event_key: &str,
        affected_node: &str,
        event_kind: SupplyEventKind,
        observed_at: &str,
        evidence: EvidenceReference,
    ) -> Result<Self, GraphError> {
        Ok(Self {
            event_key: required_text(event_key, "supply_event_key")?,
            affected_node: required_text(affected_node, "affected_supply_node_key")?,
            event_kind,
            observed_at: required_text(observed_at, "observed_at")?,
            evidence,
        })
    }

    /// Returns the stable event key.
    #[must_use]
    pub fn event_key(&self) -> &str {
        &self.event_key
    }

    /// Returns the directly affected node key.
    #[must_use]
    pub fn affected_node(&self) -> &str {
        &self.affected_node
    }

    /// Returns the event category.
    #[must_use]
    pub const fn event_kind(&self) -> SupplyEventKind {
        self.event_kind
    }

    /// Returns the source-system observation timestamp text.
    #[must_use]
    pub fn observed_at(&self) -> &str {
        &self.observed_at
    }

    /// Returns the immutable evidence pointer.
    #[must_use]
    pub const fn evidence(&self) -> &EvidenceReference {
        &self.evidence
    }
}

/// A deterministic downstream reachability result for one affected node.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ImpactRecord {
    node_key: String,
    hop_count: usize,
    dependency_path: Vec<String>,
    dependency_evidence: Vec<EvidenceReference>,
    event_evidence: EvidenceReference,
}

impl ImpactRecord {
    /// Returns the potentially impacted downstream node key.
    #[must_use]
    pub fn node_key(&self) -> &str {
        &self.node_key
    }

    /// Returns dependency hops from the directly affected source node.
    #[must_use]
    pub const fn hop_count(&self) -> usize {
        self.hop_count
    }

    /// Returns the shortest admitted dependency path from source to this node, inclusive.
    #[must_use]
    pub fn dependency_path(&self) -> &[String] {
        &self.dependency_path
    }

    /// Returns one evidence reference for every edge in the dependency path.
    #[must_use]
    pub fn dependency_evidence(&self) -> &[EvidenceReference] {
        &self.dependency_evidence
    }

    /// Returns the evidence reference for the disruption event that produced this impact.
    #[must_use]
    pub const fn event_evidence(&self) -> &EvidenceReference {
        &self.event_evidence
    }
}

/// In-memory aggregate for evidence-backed disruption reachability.
#[derive(Clone, Debug, Default)]
pub struct SupplyGraph {
    nodes: BTreeMap<String, SupplyNodeKind>,
    dependencies: BTreeMap<String, BTreeMap<String, EvidenceReference>>,
    events: BTreeMap<String, SupplyEvent>,
}

impl SupplyGraph {
    /// Creates an empty supply graph.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            nodes: BTreeMap::new(),
            dependencies: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    /// Adds a uniquely keyed supply node.
    pub fn add_node(
        &mut self,
        node_key: &str,
        node_kind: SupplyNodeKind,
    ) -> Result<(), GraphError> {
        let node_key = required_text(node_key, "supply_node_key")?;
        if self.nodes.contains_key(&node_key) {
            return Err(GraphError::DuplicateNode(node_key));
        }
        self.nodes.insert(node_key, node_kind);
        Ok(())
    }

    /// Idempotently inserts a supply node and rejects same-key semantic conflicts.
    ///
    /// This command is intended for replayable ingestion boundaries. An exact replay returns
    /// [`UpsertOutcome::Unchanged`]; reusing a semantic key for a different node kind fails
    /// closed instead of mutating the admitted fact.
    pub fn upsert_node(
        &mut self,
        node_key: &str,
        node_kind: SupplyNodeKind,
    ) -> Result<UpsertOutcome, GraphError> {
        let node_key = required_text(node_key, "supply_node_key")?;
        match self.nodes.get(&node_key) {
            Some(existing_kind) if *existing_kind == node_kind => Ok(UpsertOutcome::Unchanged),
            Some(_) => Err(GraphError::ConflictingNode(node_key)),
            None => {
                self.nodes.insert(node_key, node_kind);
                Ok(UpsertOutcome::Inserted)
            }
        }
    }

    /// Returns the registered kind for a supply node after normalizing surrounding whitespace.
    #[must_use]
    pub fn node_kind(&self, node_key: &str) -> Option<SupplyNodeKind> {
        self.nodes.get(node_key.trim()).copied()
    }

    /// Adds an evidence-backed dependency where `downstream_node` depends on `upstream_node`.
    pub fn add_dependency(
        &mut self,
        upstream_node: &str,
        downstream_node: &str,
        evidence: EvidenceReference,
    ) -> Result<(), GraphError> {
        let upstream_node = required_text(upstream_node, "upstream_supply_node_key")?;
        let downstream_node = required_text(downstream_node, "downstream_supply_node_key")?;
        if upstream_node == downstream_node {
            return Err(GraphError::SelfDependency(upstream_node));
        }
        if !self.nodes.contains_key(&upstream_node) {
            return Err(GraphError::UnknownNode(upstream_node));
        }
        if !self.nodes.contains_key(&downstream_node) {
            return Err(GraphError::UnknownNode(downstream_node));
        }

        let downstream_by_key = self.dependencies.entry(upstream_node.clone()).or_default();
        if downstream_by_key.contains_key(&downstream_node) {
            return Err(GraphError::DuplicateDependency {
                upstream_node,
                downstream_node,
            });
        }
        downstream_by_key.insert(downstream_node, evidence);
        Ok(())
    }

    /// Idempotently inserts an evidence-backed dependency without rewriting admitted evidence.
    ///
    /// Exact replay of the same edge and evidence is a no-op. Reusing the edge identity with a
    /// different evidence reference fails closed so ingestion retries cannot silently rewrite
    /// provenance.
    pub fn upsert_dependency(
        &mut self,
        upstream_node: &str,
        downstream_node: &str,
        evidence: EvidenceReference,
    ) -> Result<UpsertOutcome, GraphError> {
        let upstream_node = required_text(upstream_node, "upstream_supply_node_key")?;
        let downstream_node = required_text(downstream_node, "downstream_supply_node_key")?;
        if upstream_node == downstream_node {
            return Err(GraphError::SelfDependency(upstream_node));
        }
        if !self.nodes.contains_key(&upstream_node) {
            return Err(GraphError::UnknownNode(upstream_node));
        }
        if !self.nodes.contains_key(&downstream_node) {
            return Err(GraphError::UnknownNode(downstream_node));
        }

        let downstream_by_key = self.dependencies.entry(upstream_node.clone()).or_default();
        match downstream_by_key.get(&downstream_node) {
            Some(existing_evidence) if existing_evidence == &evidence => Ok(UpsertOutcome::Unchanged),
            Some(_) => Err(GraphError::ConflictingDependency {
                upstream_node,
                downstream_node,
            }),
            None => {
                downstream_by_key.insert(downstream_node, evidence);
                Ok(UpsertOutcome::Inserted)
            }
        }
    }

    /// Records an evidence-backed event without silently overwriting a prior event key.
    pub fn record_event(&mut self, event: SupplyEvent) -> Result<(), GraphError> {
        if !self.nodes.contains_key(event.affected_node()) {
            return Err(GraphError::UnknownNode(event.affected_node().to_owned()));
        }
        if self.events.contains_key(event.event_key()) {
            return Err(GraphError::DuplicateEvent(event.event_key().to_owned()));
        }
        self.events.insert(event.event_key().to_owned(), event);
        Ok(())
    }

    /// Idempotently inserts an event while keeping an admitted event immutable by semantic key.
    ///
    /// An exact event replay is a no-op. Any changed affected node, event kind, observation time,
    /// or evidence for an existing event key is a conflict rather than an overwrite.
    pub fn upsert_event(&mut self, event: SupplyEvent) -> Result<UpsertOutcome, GraphError> {
        if !self.nodes.contains_key(event.affected_node()) {
            return Err(GraphError::UnknownNode(event.affected_node().to_owned()));
        }
        match self.events.get(event.event_key()) {
            Some(existing_event) if existing_event == &event => Ok(UpsertOutcome::Unchanged),
            Some(_) => Err(GraphError::ConflictingEvent(event.event_key().to_owned())),
            None => {
                self.events.insert(event.event_key().to_owned(), event);
                Ok(UpsertOutcome::Inserted)
            }
        }
    }

    /// Computes unique potentially impacted nodes by directed dependency reachability.
    ///
    /// The result intentionally makes no claim about severity, probability, timing, or
    /// recoverability. Records are ordered by shortest hop count and then semantic node key.
    /// Each record carries the first deterministic shortest dependency path, its edge evidence,
    /// and the source evidence for the disruption event.
    pub fn downstream_impacts(&self, event_key: &str) -> Result<Vec<ImpactRecord>, GraphError> {
        let normalized_event_key = event_key.trim();
        let event = self
            .events
            .get(normalized_event_key)
            .ok_or_else(|| GraphError::UnknownEvent(normalized_event_key.to_owned()))?;
        let source_node = event.affected_node().to_owned();
        let mut visited = BTreeSet::from([source_node.clone()]);
        let mut queue = VecDeque::from([(
            source_node.clone(),
            vec![source_node],
            Vec::<EvidenceReference>::new(),
        )]);
        let mut impacts = Vec::new();

        while let Some((node_key, dependency_path, evidence_path)) = queue.pop_front() {
            let Some(downstream_nodes) = self.dependencies.get(&node_key) else {
                continue;
            };
            for (downstream_node, edge_evidence) in downstream_nodes {
                if visited.insert(downstream_node.clone()) {
                    let mut downstream_path = dependency_path.clone();
                    downstream_path.push(downstream_node.clone());
                    let mut downstream_evidence = evidence_path.clone();
                    downstream_evidence.push(edge_evidence.clone());
                    let hop_count = downstream_path.len() - 1;
                    impacts.push(ImpactRecord {
                        node_key: downstream_node.clone(),
                        hop_count,
                        dependency_path: downstream_path.clone(),
                        dependency_evidence: downstream_evidence.clone(),
                        event_evidence: event.evidence().clone(),
                    });
                    queue.push_back((
                        downstream_node.clone(),
                        downstream_path,
                        downstream_evidence,
                    ));
                }
            }
        }

        impacts.sort_by(|left, right| {
            left.hop_count
                .cmp(&right.hop_count)
                .then_with(|| left.node_key.cmp(&right.node_key))
        });
        Ok(impacts)
    }
}
