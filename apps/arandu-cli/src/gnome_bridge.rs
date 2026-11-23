//! A2: one-shot GNOME → Rust bridge (read-only, no background polling).
//!
//! The GNOME adapter owns host discovery. This process only accepts bounded
//! GNOME application IDs via stdin. It cannot launch or focus applications,
//! contact a model, access user documents, or change the desktop.

use std::io::{self, Read, Write};

use arandu_core::AranduCore;
use arandu_model::RiverKind;
use arandu_platform_api::gnome_snapshot::GnomeSnapshotAdapter;

const MAX_INPUT_BYTES: usize = 32_768;

/// Synchronizes an explicitly requested app-ID snapshot with the real core.
///
/// # Errors
///
/// Returns an error when the input exceeds the bound, has invalid IDs, or
/// cannot be processed/output.
pub fn run<R: Read, W: Write>(input: R, mut output: W) -> io::Result<()> {
    let mut raw = String::new();
    let bound = u64::try_from(MAX_INPUT_BYTES + 1).unwrap_or(u64::MAX);
    input.take(bound).read_to_string(&mut raw)?;
    if raw.len() > MAX_INPUT_BYTES {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "GNOME: entrada excede limite de segurança",
        ));
    }

    let adapter = GnomeSnapshotAdapter::from_app_ids(raw.lines())
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "GNOME: IDs inválidos"))?;

    let mut core = AranduCore::default();
    core.synchronize_platform(&adapter)
        .map_err(|_| io::Error::other("GNOME: sincronização indisponível"))?;

    let open = &core.river(RiverKind::Open).nodes;
    writeln!(output, "ARANDU A2 | Ponte GNOME → Rust (somente leitura)")?;
    writeln!(output, "ABERTOS: {} aplicativo(s)", open.len())?;
    for id in open {
        writeln!(output, "• {}", id.as_str())?;
    }
    writeln!(output, "MEMÓRIA e FREQUENTES: não inferidos a partir de um único snapshot")?;
    writeln!(output, "FOCO: não fornecido por este contrato")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn gnome_snapshot_populates_rust_open_river() {
        let data = b"org.mozilla.firefox.desktop\norg.gnome.Nautilus.desktop\norg.mozilla.firefox.desktop\n";
        let mut out = Vec::new();
        run(Cursor::new(data), &mut out).expect("bridge should accept IDs");
        let view = String::from_utf8(out).expect("UTF-8 output");
        assert!(view.contains("ABERTOS: 2 aplicativo(s)"));
        assert!(view.contains("org.gnome.Nautilus.desktop"));
        assert!(!view.contains("FOCO: org"));
    }

    #[test]
    fn rejects_invalid_and_oversized_inputs() {
        assert!(run(Cursor::new(b"../home/secrets\n"), Vec::new()).is_err());
        let too_big = vec![b'a'; (MAX_INPUT_BYTES + 1) as usize];
        assert!(run(Cursor::new(too_big), Vec::new()).is_err());
    }

    #[test]
    fn empty_snapshot_is_a_valid_request() {
        let mut out = Vec::new();
        run(Cursor::new(b""), &mut out).expect("empty snapshot");
        assert!(String::from_utf8(out)
            .expect("UTF-8 output")
            .contains("ABERTOS: 0 aplicativo(s)"));
    }
}
