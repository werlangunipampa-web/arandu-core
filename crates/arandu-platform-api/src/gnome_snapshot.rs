//! A2: read-only GNOME Confluência snapshot adapter.
//!
//! This is a platform boundary, not a second desktop. GNOME/Mutter retain
//! ownership of windows and monitors; the ARANDU Rust core owns the rivers.
//! The existing GNOME extension supplies app IDs via its opt-in D-Bus
//! Snapshot method. Window titles and documents never cross this boundary.

use std::collections::HashSet;

use arandu_model::{ApplicationId, ApplicationNode};

use crate::{PlatformAdapter, PlatformError};

/// Maximum number of observed windows in one explicitly requested snapshot.
pub const MAX_SNAPSHOT_WINDOWS: usize = 128;
const MAX_APP_ID_BYTES: usize = 192;

#[derive(Debug, Clone, Default)]
pub struct GnomeSnapshotAdapter {
    open: Vec<ApplicationNode>,
}

impl GnomeSnapshotAdapter {
    /// Builds the adapter from GNOME desktop application IDs only.
    ///
    /// Repeated windows of the same application map to one node. No titles,
    /// paths, timestamps, monitor data, focus inference or personal history
    /// are retained.
    ///
    /// # Errors
    ///
    /// Rejects malformed IDs and snapshots over the per-request bound.
    pub fn from_app_ids<I, S>(ids: I) -> Result<Self, PlatformError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let mut open = Vec::new();
        let mut seen = HashSet::new();
        let mut windows = 0usize;

        for item in ids {
            windows += 1;
            if windows > MAX_SNAPSHOT_WINDOWS {
                return Err(PlatformError::new("GNOME: limite de janelas excedido"));
            }
            let id = item.as_ref();
            if !valid_app_id(id) {
                return Err(PlatformError::new("GNOME: identificador de aplicativo inválido"));
            }
            if seen.insert(id.to_owned()) {
                open.push(ApplicationNode::new(
                    ApplicationId::new(id),
                    id,
                    id,
                ));
            }
        }
        Ok(Self { open })
    }
}

fn valid_app_id(id: &str) -> bool {
    !id.is_empty()
        && id.len() <= MAX_APP_ID_BYTES
        && id.bytes().next().is_some_and(|b| b.is_ascii_alphanumeric())
        && id
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'.' | b'_' | b'-'))
}

impl PlatformAdapter for GnomeSnapshotAdapter {
    fn list_open_applications(&self) -> Result<Vec<ApplicationNode>, PlatformError> {
        Ok(self.open.clone())
    }

    fn focused_application(&self) -> Result<Option<ApplicationNode>, PlatformError> {
        // The historical Snapshot contract does not expose real-time focus.
        // Never manufacture focus from the order of windows or recent history.
        Ok(None)
    }

    fn installed_applications(&self) -> Result<Vec<ApplicationNode>, PlatformError> {
        Err(PlatformError::new("GNOME: catálogo instalado não fornecido pelo Snapshot"))
    }

    fn activate_application(&self, _app: &ApplicationNode) -> Result<(), PlatformError> {
        Err(PlatformError::new("GNOME: ativação desabilitada no adaptador de leitura"))
    }

    fn launch_application(&self, _app: &ApplicationNode) -> Result<(), PlatformError> {
        Err(PlatformError::new("GNOME: lançamento desabilitado no adaptador de leitura"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deduplicates_windows_but_preserves_first_seen_order() {
        let adapter = GnomeSnapshotAdapter::from_app_ids([
            "org.mozilla.firefox.desktop",
            "org.gnome.Nautilus.desktop",
            "org.mozilla.firefox.desktop",
        ])
        .expect("valid GNOME snapshot");
        let open = adapter.list_open_applications().expect("open applications");
        assert_eq!(open.len(), 2);
        assert_eq!(open[0].id.as_str(), "org.mozilla.firefox.desktop");
        assert_eq!(open[1].id.as_str(), "org.gnome.Nautilus.desktop");
    }

    #[test]
    fn refuses_titles_control_characters_and_oversized_inputs() {
        for bad in ["", "a\tprivate", "a\nprivate", "/home/user", "a b", "../private"] {
            assert!(GnomeSnapshotAdapter::from_app_ids([bad]).is_err());
        }
        let ids = vec!["app.desktop"; MAX_SNAPSHOT_WINDOWS + 1];
        assert!(GnomeSnapshotAdapter::from_app_ids(ids).is_err());
        assert!(GnomeSnapshotAdapter::from_app_ids(["x".repeat(193)]).is_err());
    }

    #[test]
    fn snapshot_does_not_infer_focus_or_allow_mutation() {
        let adapter = GnomeSnapshotAdapter::from_app_ids(["org.gnome.Nautilus.desktop"])
            .expect("valid GNOME snapshot");
        assert!(adapter.focused_application().expect("focus query").is_none());
        assert!(adapter.installed_applications().is_err());
        let app = &adapter.list_open_applications().expect("open")[0];
        assert!(adapter.activate_application(app).is_err());
        assert!(adapter.launch_application(app).is_err());
    }
}
