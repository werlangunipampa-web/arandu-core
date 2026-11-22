//! A0.8: a minimal semantic graph of synthetic, attributed statements.
//!
//! This is NOT the canonical ARANDU ontology, an OWL reasoner, or an ECA
//! learning engine. No inference, promotion, persistence or external writes.

use crate::ontology_contract::{EpistemicStatus, Provenance};
use std::collections::BTreeMap;

/// A locally identified concept. No live Seed identifiers are accepted here.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Concept {
    pub id: String,
    pub label: String,
}

/// An explicit relation from a source concept to a target concept.
/// Relation predicates are data, not executable commands.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticStatement {
    pub subject: String,
    pub predicate: String,
    pub object: String,
    pub status: EpistemicStatus,
    pub provenance: Provenance,
}

/// Validation results never imply that a statement is scientifically true.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphError {
    EmptyIdentifier,
    DuplicateConcept,
    UnknownConcept,
    MissingProvenance,
}

/// Immutable-by-default query surface. Mutation is explicit in the builder.
#[derive(Debug, Clone, Default)]
pub struct SemanticGraph {
    concepts: BTreeMap<String, Concept>,
    statements: Vec<SemanticStatement>,
}

impl SemanticGraph {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// # Errors
    ///
    /// Rejects empty identifiers/labels and duplicates.
    pub fn add_concept(&mut self, concept: Concept) -> Result<(), GraphError> {
        if concept.id.trim().is_empty() || concept.label.trim().is_empty() {
            return Err(GraphError::EmptyIdentifier);
        }
        if self.concepts.contains_key(&concept.id) {
            return Err(GraphError::DuplicateConcept);
        }
        self.concepts.insert(concept.id.clone(), concept);
        Ok(())
    }

    /// # Errors
    ///
    /// Rejects incomplete statements, missing sources and unknown endpoints.
    /// This checks structural integrity, not truth or canonical ontology fit.
    pub fn add_statement(&mut self, statement: SemanticStatement) -> Result<(), GraphError> {
        if statement.subject.trim().is_empty()
            || statement.predicate.trim().is_empty()
            || statement.object.trim().is_empty()
        {
            return Err(GraphError::EmptyIdentifier);
        }
        if statement.provenance.source_id.trim().is_empty()
            || statement.provenance.recorded_at.trim().is_empty()
        {
            return Err(GraphError::MissingProvenance);
        }
        if !self.concepts.contains_key(&statement.subject)
            || !self.concepts.contains_key(&statement.object)
        {
            return Err(GraphError::UnknownConcept);
        }
        self.statements.push(statement);
        Ok(())
    }

    #[must_use]
    pub fn concept(&self, id: &str) -> Option<&Concept> {
        self.concepts.get(id)
    }

    #[must_use]
    pub fn outgoing(&self, concept_id: &str) -> Vec<&SemanticStatement> {
        self.statements
            .iter()
            .filter(|statement| statement.subject == concept_id)
            .collect()
    }

    #[must_use]
    pub fn concept_count(&self) -> usize {
        self.concepts.len()
    }

    #[must_use]
    pub fn statement_count(&self) -> usize {
        self.statements.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn graph() -> SemanticGraph {
        let mut graph = SemanticGraph::new();
        graph
            .add_concept(Concept {
                id: "synthetic-previous-knowledge".into(),
                label: "Previous knowledge (synthetic)".into(),
            })
            .expect("fixture concept is valid");
        graph
            .add_concept(Concept {
                id: "synthetic-new-concept".into(),
                label: "New concept (synthetic)".into(),
            })
            .expect("fixture concept is valid");
        graph
    }

    fn statement() -> SemanticStatement {
        SemanticStatement {
            subject: "synthetic-previous-knowledge".into(),
            predicate: "synthetic-relationship".into(),
            object: "synthetic-new-concept".into(),
            status: EpistemicStatus::Hypothesis,
            provenance: Provenance {
                source_id: "synthetic-fixture".into(),
                recorded_at: "synthetic-time".into(),
            },
        }
    }

    #[test]
    fn records_attributed_relation_without_inference_or_promotion() {
        let mut graph = graph();
        graph.add_statement(statement()).expect("valid fixture");
        assert_eq!(graph.concept_count(), 2);
        assert_eq!(graph.statement_count(), 1);
        let outgoing = graph.outgoing("synthetic-previous-knowledge");
        assert_eq!(outgoing.len(), 1);
        assert_eq!(outgoing[0].status, EpistemicStatus::Hypothesis);
        assert_eq!(outgoing[0].provenance.source_id, "synthetic-fixture");
        assert!(graph.outgoing("synthetic-new-concept").is_empty());
    }

    #[test]
    fn rejects_missing_endpoints_and_provenance_without_mutating_graph() {
        let mut graph = graph();
        let mut invalid = statement();
        invalid.object = "unknown".into();
        assert_eq!(graph.add_statement(invalid), Err(GraphError::UnknownConcept));
        let mut invalid = statement();
        invalid.provenance.source_id.clear();
        assert_eq!(graph.add_statement(invalid), Err(GraphError::MissingProvenance));
        assert_eq!(graph.statement_count(), 0);
    }

    #[test]
    fn rejects_duplicate_and_blank_concepts() {
        let mut graph = graph();
        assert_eq!(
            graph.add_concept(Concept {
                id: "synthetic-new-concept".into(),
                label: "Duplicate".into(),
            }),
            Err(GraphError::DuplicateConcept)
        );
        assert_eq!(
            graph.add_concept(Concept {
                id: " ".into(),
                label: "Blank".into(),
            }),
            Err(GraphError::EmptyIdentifier)
        );
        assert_eq!(graph.concept_count(), 2);
    }
}
