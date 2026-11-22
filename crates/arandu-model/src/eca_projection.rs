//! A0.9 experimental, read-only projection of explicit semantic relations.
//!
//! This adapter is a synthetic research fixture, NOT an Ausubelian learning
//! algorithm. A relationship is a *candidate for human review*, not evidence
//! of assimilation, progressive differentiation or integrative reconciliation.
//! Application boundaries must authorize access before calling this module.

use crate::ontology_contract::{EpistemicStatus, Provenance};
use crate::semantic_graph::SemanticGraph;

/// Explicit, attributed connection proposed for educational investigation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LearningRelationCandidate {
    pub previous_concept_id: String,
    pub relation_id: String,
    pub new_concept_id: String,
    pub epistemic_status: EpistemicStatus,
    pub provenance: Provenance,
}

/// Returns only statements already recorded in the graph.
/// No semantic relation is inferred; the graph remains unchanged.
#[must_use]
pub fn project_relations(
    graph: &SemanticGraph,
    previous_concept_id: &str,
) -> Vec<LearningRelationCandidate> {
    graph
        .outgoing(previous_concept_id)
        .into_iter()
        .map(|statement| LearningRelationCandidate {
            previous_concept_id: statement.subject.clone(),
            relation_id: statement.predicate.clone(),
            new_concept_id: statement.object.clone(),
            epistemic_status: statement.status,
            provenance: statement.provenance.clone(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::semantic_graph::{Concept, SemanticStatement};

    #[test]
    fn projects_only_explicit_attributed_relations_without_changing_graph() {
        let mut graph = SemanticGraph::new();
        for id in ["prior", "new", "unrelated"] {
            graph
                .add_concept(Concept {
                    id: id.into(),
                    label: id.into(),
                })
                .expect("synthetic concept");
        }
        graph
            .add_statement(SemanticStatement {
                subject: "prior".into(),
                predicate: "synthetic-relates-to".into(),
                object: "new".into(),
                status: EpistemicStatus::Hypothesis,
                provenance: Provenance {
                    source_id: "synthetic-source".into(),
                    recorded_at: "synthetic-time".into(),
                },
            })
            .expect("synthetic statement");

        let before = graph.statement_count();
        let candidates = project_relations(&graph, "prior");
        assert_eq!(candidates.len(), 1);
        assert_eq!(candidates[0].previous_concept_id, "prior");
        assert_eq!(candidates[0].new_concept_id, "new");
        assert_eq!(candidates[0].epistemic_status, EpistemicStatus::Hypothesis);
        assert_eq!(candidates[0].provenance.source_id, "synthetic-source");
        assert!(project_relations(&graph, "unrelated").is_empty());
        assert_eq!(graph.statement_count(), before);
    }

    #[test]
    fn unknown_concept_yields_no_candidates() {
        let graph = SemanticGraph::new();
        assert!(project_relations(&graph, "absent").is_empty());
    }
}
