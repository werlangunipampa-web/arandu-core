//! Experimental A0.5 policy boundary for ontology lookup.
//!
//! This is an in-memory reference implementation with synthetic data.
//! It does not authenticate the requesting process, enforce OS isolation,
//! or provide a capability to write into a live Seed.
use crate::ontology_contract::{KnowledgeReference, OntologyQuery, OntologyReader};

/// Purpose must be declared by the caller rather than inferred from the query.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueryPurpose {
    UserNavigation,
    LearningResearch,
}

/// Explicitly approved lookup scope.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadGrant {
    pub subject_id: String,
    pub purpose: QueryPurpose,
    pub permitted_concepts: Vec<String>,
}

/// A request identifies the principal and why they are asking.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizedQuery {
    pub subject_id: String,
    pub purpose: QueryPurpose,
    pub query: OntologyQuery,
}

/// Authorization is evaluated on every lookup and does not mutate a grant.
pub trait ReadPolicy {
    fn permits(&self, request: &AuthorizedQuery) -> bool;
}

/// An empty grant set denies every request.
#[derive(Debug, Clone, Default)]
pub struct ExplicitGrantPolicy {
    grants: Vec<ReadGrant>,
}

impl ExplicitGrantPolicy {
    #[must_use]
    pub fn new(grants: Vec<ReadGrant>) -> Self {
        Self { grants }
    }
}

impl ReadPolicy for ExplicitGrantPolicy {
    fn permits(&self, request: &AuthorizedQuery) -> bool {
        if request.subject_id.is_empty() || request.query.concept_id.is_empty() {
            return false;
        }

        self.grants.iter().any(|grant| {
            grant.subject_id == request.subject_id
                && grant.purpose == request.purpose
                && grant
                    .permitted_concepts
                    .iter()
                    .any(|concept| concept == &request.query.concept_id)
        })
    }
}

/// Access-denied results deliberately reveal no information about a concept.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GuardedLookupError<E> {
    Denied,
    Backend(E),
}

/// Only use this guarded facade at the application boundary.
/// Do not expose the underlying `OntologyReader` to untrusted callers.
pub struct GuardedOntologyReader<R, P> {
    reader: R,
    policy: P,
}

impl<R, P> GuardedOntologyReader<R, P>
where
    R: OntologyReader,
    P: ReadPolicy,
{
    #[must_use]
    pub fn new(reader: R, policy: P) -> Self {
        Self { reader, policy }
    }

    /// # Errors
    ///
    /// Denies a missing grant or returns a backend lookup error.
    pub fn lookup(
        &self,
        request: &AuthorizedQuery,
    ) -> Result<Vec<KnowledgeReference>, GuardedLookupError<R::Error>> {
        if !self.policy.permits(request) {
            return Err(GuardedLookupError::Denied);
        }

        self.reader
            .lookup(&request.query)
            .map_err(GuardedLookupError::Backend)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ontology_contract::{EpistemicStatus, Provenance};
    use std::cell::Cell;

    struct SyntheticReader {
        calls: Cell<u32>,
    }

    impl OntologyReader for SyntheticReader {
        type Error = &'static str;

        fn lookup(&self, query: &OntologyQuery) -> Result<Vec<KnowledgeReference>, Self::Error> {
            self.calls.set(self.calls.get() + 1);
            Ok(vec![KnowledgeReference {
                concept_id: query.concept_id.clone(),
                status: EpistemicStatus::Hypothesis,
                provenance: Provenance {
                    source_id: "synthetic-test-fixture".into(),
                    recorded_at: "synthetic-timestamp".into(),
                },
            }])
        }
    }

    fn request(subject: &str, purpose: QueryPurpose, concept: &str) -> AuthorizedQuery {
        AuthorizedQuery {
            subject_id: subject.into(),
            purpose,
            query: OntologyQuery {
                concept_id: concept.into(),
            },
        }
    }

    fn grant() -> ReadGrant {
        ReadGrant {
            subject_id: "synthetic-user".into(),
            purpose: QueryPurpose::LearningResearch,
            permitted_concepts: vec!["concept-a".into()],
        }
    }

    #[test]
    fn default_policy_denies_without_consulting_backend() {
        let guarded = GuardedOntologyReader::new(
            SyntheticReader {
                calls: Cell::new(0),
            },
            ExplicitGrantPolicy::default(),
        );
        let result = guarded.lookup(&request(
            "synthetic-user",
            QueryPurpose::LearningResearch,
            "concept-a",
        ));
        assert_eq!(result, Err(GuardedLookupError::Denied));
        assert_eq!(guarded.reader.calls.get(), 0);
    }

    #[test]
    fn explicit_grant_allows_only_matching_subject_purpose_and_concept() {
        let guarded = GuardedOntologyReader::new(
            SyntheticReader {
                calls: Cell::new(0),
            },
            ExplicitGrantPolicy::new(vec![grant()]),
        );

        for (subject, purpose, concept) in [
            ("other-user", QueryPurpose::LearningResearch, "concept-a"),
            ("synthetic-user", QueryPurpose::UserNavigation, "concept-a"),
            (
                "synthetic-user",
                QueryPurpose::LearningResearch,
                "concept-b",
            ),
            ("", QueryPurpose::LearningResearch, "concept-a"),
            ("synthetic-user", QueryPurpose::LearningResearch, ""),
        ] {
            assert_eq!(
                guarded.lookup(&request(subject, purpose, concept)),
                Err(GuardedLookupError::Denied)
            );
        }
        assert_eq!(guarded.reader.calls.get(), 0);

        let result = guarded
            .lookup(&request(
                "synthetic-user",
                QueryPurpose::LearningResearch,
                "concept-a",
            ))
            .expect("synthetic grant should allow matching lookup");
        assert_eq!(result[0].status, EpistemicStatus::Hypothesis);
        assert_eq!(guarded.reader.calls.get(), 1);
    }

    struct FailingReader;

    impl OntologyReader for FailingReader {
        type Error = &'static str;

        fn lookup(&self, _query: &OntologyQuery) -> Result<Vec<KnowledgeReference>, Self::Error> {
            Err("synthetic-backend-error")
        }
    }

    #[test]
    fn backend_error_is_not_converted_into_a_success() {
        let guarded =
            GuardedOntologyReader::new(FailingReader, ExplicitGrantPolicy::new(vec![grant()]));
        assert_eq!(
            guarded.lookup(&request(
                "synthetic-user",
                QueryPurpose::LearningResearch,
                "concept-a",
            )),
            Err(GuardedLookupError::Backend("synthetic-backend-error"))
        );
    }
}
