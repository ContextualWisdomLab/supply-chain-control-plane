# UX Baseline — Supply Chain Control Plane

## 1. Product experience objective

The interface helps a user move from an observed disruption to a verified action. It does not present an ornamental control tower, a wall of undifferentiated cards, or an unexplained AI risk score.

Every screen answers at least one operational question:

- What changed?
- How reliable and current is the evidence?
- What may be affected and through which path?
- What consequence is estimated, under which model and horizon?
- Which responses are feasible and what do they trade off?
- What decision or action is required from me?
- What has been executed and did it work?

## 2. Information hierarchy

```text
case and decision context
→ current material state
→ evidence and uncertainty
→ affected commitments
→ feasible alternatives
→ next authorized action
→ execution and outcome
```

Internal bounded-context names are not the primary navigation. Users see their tasks and supply-chain concepts; implementation boundaries remain available in diagnostics and administration.

## 3. Primary navigation

```text
Command Center
Disruption Cases
Network and Commitments
Scenarios
Action Queue
Evidence and Sources
Models and Quality
Integrations
Administration
```

## 4. Core screens

### Command Center

Purpose: show material open cases, commitments at risk, actions due, stale evidence, failed writebacks, and decisions awaiting the current user.

Required content:

- explicit data freshness and knowledge cutoff;
- case priority and state;
- affected commitment count and value only when supported;
- action owner and due state;
- unresolved evidence or identity conflicts;
- failed/ambiguous writeback warnings;
- links that name the concrete task, such as `Review three affected customer orders`, not `Learn more`.

### Disruption Case Workspace

Sections:

```text
Case summary
Evidence timeline
Facts and hypotheses
Affected network paths
Impact assessments
Recovery scenarios
Decision record
Actions and outcomes
```

Facts, inferences, proposals, decisions, and outcomes have distinct text labels and semantics. Color is supplemental.

### Network Impact Explorer

Views:

- path list;
- accessible exact-value table;
- graph or layered dependency view;
- site/item/order/commitment filters;
- valid-time and knowledge-cutoff control;
- source authority and truth-status filter;
- evidence drawer.

The graph does not imply statistical distance through node position. Every edge exposes relation type, validity, source, evidence, and assessment use.

### Scenario Compare

The comparison emphasizes:

- feasibility;
- binding constraints;
- service and commitment effects;
- cost and working-capital effects;
- implementation lead time;
- uncertainty and model limitations;
- required approvals and actions.

The system does not force all objectives into one visual score. Users can compare a Pareto set and inspect the policy that orders options when such a policy exists.

### Decision Review

The decision screen contains:

- problem statement and knowledge cutoff;
- options considered;
- evidence and model basis;
- known gaps and residual risk;
- approval policy and required roles;
- selected option and expiry;
- exact actions to be created.

The approval control uses specific language such as `Approve inventory reallocation for 12 orders until 18:00 KST`, not a generic `Approve` detached from scope.

### Action Queue

Each item shows:

- action description;
- target operating system;
- owner or adapter;
- authorization and decision expiry;
- due time;
- dispatch state;
- external receipt or ambiguity;
- next recovery action.

### Evidence Drawer

The drawer shows source type, authority, capture time, availability time, valid time, digest, classification, truth status, and permitted excerpt. Protected locators are revealed only through an authorized drill-through action with a separate audit receipt.

### Integrations Workbench

The workbench exposes adapter version, source system, capabilities, schema support, last successful capture, lag, failures, replay conflict, writeback risk class, and kill switch. It does not expose raw credentials.

## 5. User-visible state language

Examples:

| Internal state | User-facing language |
| --- | --- |
| `partial_data` | `Some source systems have not reported for this cutoff.` |
| `assessment_abstained` | `The model does not have enough reliable data to estimate this impact.` |
| `scenario_infeasible` | `This plan violates two required constraints.` |
| `ambiguous_writeback` | `The external system did not confirm whether the change was applied. Reconcile before retrying.` |
| `identity_conflict` | `Two source records may refer to the same item. A data steward must resolve them.` |
| `access_denied` | `You do not have permission to view this evidence for the selected purpose.` |
| `stale_projection` | `This view is 9 minutes behind the authoritative records.` |

Messages explain the next permissible action and avoid implementation details.

## 6. Empty, loading, error, and recovery states

- Empty screens explain what evidence or setup creates the first useful item.
- Loading identifies the workload and offers cancellation for long-running jobs.
- Partial data remains usable where safe and visibly bounded.
- Errors preserve entered work and provide a concrete retry or escalation path.
- Access denied does not reveal hidden object existence.
- Offline or interrupted review restores the last acknowledged draft without claiming submission.
- Failed writeback never disappears from the action queue.

## 7. Accessibility

Target: WCAG 2.2 AA / ISO/IEC 40500:2025.

Required:

- complete keyboard path and visible focus;
- minimum touch target and no drag-only operation;
- accessible authentication and redundant-entry reduction;
- graph alternatives as path list and exact-value table;
- text equivalents for icons and statuses;
- live-region announcements for job and writeback state changes;
- chart/table values available without hover;
- zoom and reflow without clipping;
- print, PDF, SVG, CSV, JSON, and GraphML outputs where applicable;
- reduced-motion support;
- no status communicated through color alone.

## 8. Internationalization

Supported product locales:

| Language | Locale |
| --- | --- |
| Korean | `ko-KR` |
| English | `en-US` |
| Japanese | `ja-JP` |
| Simplified Chinese | `zh-CN` |
| Vietnamese | `vi-VN` |
| Spanish | `es-ES` |
| German | `de-DE` |
| French | `fr-FR` |

Requirements:

- translation resources are versioned database records with review, approval, deployment, and rollback;
- server/native surfaces request screen keys and cache bounded locale resources;
- the browser does not download the complete translation catalog;
- locale affects display formatting, not stored units or domain identity;
- CJK line breaking and font fallback are tested;
- German/French/Spanish text expansion is tested;
- dates include timezone and absolute timestamp when operationally material;
- translated labels do not shorten or alter domain meaning to fit a control.

## 9. Component system

Reusable components are built and validated in Storybook before page composition:

```text
CaseStateBanner
TruthStatusLabel
EvidenceReference
EvidenceDrawer
KnowledgeCutoffControl
ImpactPathList
ImpactGraph
ExactValueTable
EstimateWithUncertainty
BindingConstraintList
ScenarioComparisonTable
DecisionBasisPanel
ApprovalScopeSummary
ActionCommitmentRow
WritebackReceiptPanel
ProjectionFreshnessNotice
ModelStatusPanel
```

Each component has normal, loading, empty, error, access-denied, stale, partial, mobile, and translated stories where applicable.

## 10. Visual design principles

- Density follows the operational task; large datasets use tables and progressive disclosure rather than repeated rounded cards.
- Hierarchy comes from typography, spacing, grouping, and alignment, not decorative gradients or glow.
- Network and scenario visualizations exist only when they clarify a decision.
- Evidence and uncertainty stay near the claim they qualify.
- Destructive or high-risk actions have clear scope, preview, and consequence.
- Product identity should remain recognizable without relying on generic AI imagery, glass panels, background grids, or fake terminals.

## 11. Figma and implementation evidence

The first UI implementation milestone shall create:

- a Figma file and recorded file ID;
- design tokens and component mapping;
- Storybook component inventory;
- desktop, mobile, and intermediate-width frames;
- Korean and English primary frames plus expansion/CJK stress frames;
- interaction flows for case, scenario, approval, writeback ambiguity, and recovery;
- screenshots and E2E evidence linked to the exact implementation head.

No Figma screen is treated as functional evidence until the implemented interaction, states, accessibility, and responsive behavior pass Storybook and E2E verification.


## 12. Planning delivery gate

| Gate | Planning result | Basis |
| --- | --- | --- |
| Intentionality | PASS | every primary screen and component is tied to a supply-chain task or evidence need |
| Functional completeness | NOT CLAIMED | no implemented UI, Figma file, Storybook, or E2E evidence exists yet |
| Content fitness | PASS | no placeholder marketing sections, invented metrics, testimonials, or decorative dashboard blocks are specified |
| Resilience | PASS at specification level | loading, empty, partial, stale, conflict, access-denied, error, mobile, locale, keyboard, and recovery states are required |
| Evidence | PASS at specification level | quantitative, security, and performance claims require exact evidence and limitations |
| Product identity | PASS at specification level | the experience centers evidence, cutoff-aware impact, feasible recovery, authorization, writeback ambiguity, and outcome closure rather than generic AI styling |

UI delivery remains incomplete until Task 14 produces a recorded Figma file ID, tokens, reusable Storybook components, responsive/eight-locale states, screenshots, accessibility evidence, functional E2E, and measured performance on the exact implementation head.
