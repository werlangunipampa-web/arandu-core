//! A0 experimental interoperability contracts.
//!
//! These structures are NOT the canonical ARANDU ontology. They define a
//! read-only boundary for later mapping to validated ontology manifests.
//! This module never loads or mutates a live Seed.

/// A stable reference to a Seed individual. It is not a code generation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SeedReference {
    pub seed_id: String,
}

/// A technical generation can change while the individual Seed remains.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GenerationReference {
    pub generation_id: String,
}

/// A Body is a separately authorized host of a Seed.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BodyReference {
    pub body_id: String,
}

/// No epistemic category is silently promoted to a stronger one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EpistemicStatus {
    Observation,
    Hypothesis,
    Simulation,
    Interpretation,
    Validated,
    Suspended,
}

/// Where a statement came from; it must not be confused with its truth.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Provenance {
    pub source_id: String,
    pub recorded_at: String,
}

/// A non-executable reference to a knowledge statement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KnowledgeReference {
    pub concept_id: String,
    pub status: EpistemicStatus,
    pub provenance: Provenance,
}

/// Describes a lookup without granting any permission to mutate state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OntologyQuery {
    pub concept_id: String,
}

/// Fail-closed read-only interface. Implementations decide authorization
/// externally and must never expose mutation through this contract.
pub trait OntologyReader {
    type Error;

    /// Returns only references the caller is authorized to inspect.
    ///
    /// # Errors
    ///
    /// Returns an error when access is denied, the source is unavailable,
    /// or query processing fails.
    fn lookup(&self, query: &OntologyQuery) -> Result<Vec<KnowledgeReference>, Self::Error>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seed_identity_is_distinct_from_generation_and_body() {
        let seed = SeedReference {
            seed_id: "synthetic-seed".into(),
        };
        let generation = GenerationReference {
            generation_id: "experimental-generation".into(),
        };
        let body = BodyReference {
            body_id: "synthetic-host".into(),
        };
        assert_eq!(seed.seed_id, "synthetic-seed");
        assert_eq!(generation.generation_id, "experimental-generation");
        assert_eq!(body.body_id, "synthetic-host");
    }

    struct DenyAll;

    impl OntologyReader for DenyAll {
        type Error = &'static str;

        fn lookup(&self, _query: &OntologyQuery) -> Result<Vec<KnowledgeReference>, Self::Error> {
            Err("access denied")
        }
    }

    #[test]
    fn query_does_not_imply_access_or_mutation() {
        let denied = DenyAll.lookup(&OntologyQuery {
            concept_id: "synthetic-concept".into(),
        });
        assert_eq!(denied, Err("access denied"));
    }

    #[test]
    fn hypothesis_does_not_become_validated_automatically() {
        let reference = KnowledgeReference {
            concept_id: "synthetic-concept".into(),
            status: EpistemicStatus::Hypothesis,
            provenance: Provenance {
                source_id: "synthetic-source".into(),
                recorded_at: "2026-11-22".into(),
            },
        };
        assert_eq!(reference.status, EpistemicStatus::Hypothesis);
        assert_ne!(reference.status, EpistemicStatus::Validated);
    }
}
