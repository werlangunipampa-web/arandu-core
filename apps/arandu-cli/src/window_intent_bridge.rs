//! A8.4: stdin -> planos de janelas, sem executar comandos de sistema.
//!
//! Protocolo estrito de linhas, recebido SOMENTE sob clique humano na Opy:
//! A84|1|<generation>|<target>|<recolher|retomar>
//! W|<token>|<app-id>|<monitor>|<workspace>|<min>|<max>|<fullscreen>|<canmin>|<canmax>|<canclose>|<cantile>
//! ...
//! Resposta: A84|PLAN|<minimize|unminimize,activate|activate>|<token>|<generation>
//! ou A84|NOOP|<token>|<generation>. Nenhuma janela é modificada aqui.

use std::io::{self, Read, Write};

use arandu_core::espacos_vivos::{
    plan_window_action, ConfirmedGesture, PlanError, PlanOutcome, WindowEffect, WindowIntent,
    WindowObservation, WindowRef, WindowSnapshot,
};
use arandu_model::ApplicationId;

const MAX_INPUT: u64 = 24_576;

fn invalid(reason: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, reason)
}

fn parse_u64(value: &str) -> io::Result<u64> {
    value.parse::<u64>().map_err(|_| invalid("inteiro inválido"))
}

fn parse_u32(value: &str) -> io::Result<u32> {
    value.parse::<u32>().map_err(|_| invalid("índice inválido"))
}

fn flag(value: &str) -> io::Result<bool> {
    match value {
        "0" => Ok(false),
        "1" => Ok(true),
        _ => Err(invalid("capacidade inválida")),
    }
}

fn valid_app_id(value: &str) -> bool {
    (1..=192).contains(&value.len())
        && value.as_bytes()[0].is_ascii_alphanumeric()
        && value
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || b"._-".contains(&c))
}

fn parse_window(line: &str, generation: u64) -> io::Result<WindowObservation> {
    let parts: Vec<&str> = line.split('|').collect();
    if parts.len() != 12 || parts[0] != "W" || !valid_app_id(parts[2]) {
        return Err(invalid("linha de janela inválida"));
    }
    let reference = WindowRef::new(parts[1], generation)
        .map_err(|_| invalid("referência de janela inválida"))?;
    Ok(WindowObservation {
        reference,
        application: ApplicationId::new(parts[2]),
        monitor: parse_u32(parts[3])?,
        workspace: parse_u32(parts[4])?,
        minimized: flag(parts[5])?,
        maximized: flag(parts[6])?,
        fullscreen: flag(parts[7])?,
        can_minimize: flag(parts[8])?,
        can_maximize: flag(parts[9])?,
        can_close: flag(parts[10])?,
        can_tile: flag(parts[11])?,
    })
}

fn translate_error(err: PlanError) -> io::Error {
    match err {
        PlanError::WindowMissing | PlanError::StaleReference => invalid("janela não disponível"),
        _ => invalid("operação de janela recusada"),
    }
}

fn plan(input: &str) -> io::Result<String> {
    if input.contains('\r') || input.contains('\0') {
        return Err(invalid("caractere proibido"));
    }
    let mut lines = input.lines();
    let header = lines.next().ok_or_else(|| invalid("cabeçalho ausente"))?;
    let fields: Vec<&str> = header.split('|').collect();
    if fields.len() != 5 || fields[0] != "A84" || fields[1] != "1" {
        return Err(invalid("protocolo desconhecido"));
    }
    let generation = parse_u64(fields[2])?;
    let target = WindowRef::new(fields[3], generation)
        .map_err(|_| invalid("alvo inválido"))?;
    let intent = match fields[4] {
        "recolher" => WindowIntent::RecolherAoRio,
        "retomar" => WindowIntent::TrazerAoFoco,
        _ => return Err(invalid("ação não permitida neste estágio")),
    };
    let windows = lines
        .map(|line| parse_window(line, generation))
        .collect::<io::Result<Vec<_>>>()?;
    let snapshot = WindowSnapshot::new(generation, windows)
        .map_err(|_| invalid("snapshot inválido"))?;
    let result = plan_window_action(&snapshot, &target, &intent, ConfirmedGesture::Click)
        .map_err(translate_error)?;
    let output = match result {
        PlanOutcome::NoChange => format!("A84|NOOP|{}|{generation}\n", target.token()),
        PlanOutcome::Ready(plan) => {
            if plan.needs_confirmation {
                return Err(invalid("confirmação não disponível"));
            }
            let mut ops = Vec::new();
            for step in plan.steps {
                match step {
                    WindowEffect::Minimize(ref id) if id == &target => ops.push("minimize"),
                    WindowEffect::Unminimize(ref id) if id == &target => ops.push("unminimize"),
                    WindowEffect::Activate(ref id) if id == &target => ops.push("activate"),
                    _ => return Err(invalid("plano inesperado")),
                }
            }
            let actions = ops.join(",");
            if !["minimize", "unminimize,activate", "activate"].contains(&actions.as_str()) {
                return Err(invalid("sequência inesperada"));
            }
            format!("A84|PLAN|{actions}|{}|{generation}\n", target.token())
        }
    };
    Ok(output)
}

/// Executa somente validação e serialização. Sem D-Bus nem GNOME.
///
/// # Errors
///
/// Rejeita comandos, snapshots e referências incorretos.
pub fn run(mut input: impl Read, mut output: impl Write) -> io::Result<()> {
    let mut data = String::new();
    input.by_ref().take(MAX_INPUT + 1).read_to_string(&mut data)?;
    if data.len() as u64 > MAX_INPUT {
        return Err(invalid("entrada acima do limite"));
    }
    let response = plan(&data)?;
    output.write_all(response.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASE: &str =
        "A84|1|55|w11|recolher\nW|w11|org.editor.desktop|0|0|0|0|0|1|1|1|1\nW|w12|org.editor.desktop|0|0|1|0|0|1|1|1|1\n";

    fn invoke(input: &str) -> io::Result<String> {
        let mut output = Vec::new();
        run(input.as_bytes(), &mut output)?;
        String::from_utf8(output).map_err(|_| invalid("saída não UTF-8"))
    }

    #[test]
    fn plans_minimize_without_touching_other_window_from_same_app() {
        assert_eq!(invoke(BASE).unwrap(), "A84|PLAN|minimize|w11|55\n");
    }

    #[test]
    fn retomar_preserves_unminimize_then_activate_order() {
        let data = BASE.replace("|w11|recolher", "|w12|retomar");
        assert_eq!(
            invoke(&data).unwrap(),
            "A84|PLAN|unminimize,activate|w12|55\n"
        );
    }

    #[test]
    fn no_change_when_recolher_already_minimized() {
        let data = BASE.replace("|w11|recolher", "|w12|recolher");
        assert_eq!(invoke(&data).unwrap(), "A84|NOOP|w12|55\n");
    }

    #[test]
    fn rejects_close_confluence_invalid_tokens_and_titles() {
        for input in [
            BASE.replace("recolher", "encerrar"),
            BASE.replace("recolher", "confluencia"),
            BASE.replace("w11", "../private"),
            BASE.replace("org.editor.desktop", "meu arquivo aberto"),
            BASE.replace("A84|1|55", "A84|2|55"),
            BASE.replace("W|w12", "W|w11"),
            BASE.replace("|0|0|0|0|0|1|1|1|1", "|0|0|0|0|0|yes|1|1|1"),
        ] {
            assert!(invoke(&input).is_err());
        }
    }

    #[test]
    fn enforces_maximum_input_size() {
        let data = "a".repeat(MAX_INPUT as usize + 1);
        assert!(invoke(&data).is_err());
    }

    #[test]
    fn refuses_stale_or_missing_target() {
        assert!(invoke(&BASE.replace("A84|1|55|w11", "A84|1|55|unknown")).is_err());
        assert!(invoke(&BASE.replace("A84|1|55|w11", "A84|1|0|w11")).is_err());
    }
}
