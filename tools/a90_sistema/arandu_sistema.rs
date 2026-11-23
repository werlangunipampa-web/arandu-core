//! ARANDU A9.0: comandos estritamente permitidos para controles locais.
//! Camada de integração Rust/Linux: nenhuma dependência de GNOME Shell.
//! Não lê SSIDs, nomes de usuários, documentos, títulos ou redes salvas.
use std::env;
use std::process::{Command, Output};

const LIMIT: usize = 3072;

fn execute(tool: &str, args: &[&str]) -> Result<String, String> {
    let output: Output = Command::new(tool)
        .args(args)
        .env("LC_ALL", "C")
        .output()
        .map_err(|_| format!("{tool} indisponível"))?;
    if !output.status.success() {
        return Err(format!("{tool} recusou a operação"));
    }
    if output.stdout.len() > LIMIT {
        return Err(format!("{tool} retornou dados excessivos"));
    }
    String::from_utf8(output.stdout)
        .map(|value| value.trim().to_owned())
        .map_err(|_| "resposta inválida".to_string())
}

fn network_status(text: &str) -> &'static str {
    let value = text.to_ascii_lowercase();
    if value.starts_with("connected") { "connected" }
    else if value.starts_with("connecting") { "connecting" }
    else if value.starts_with("disconnected") { "disconnected" }
    else { "unknown" }
}

fn wifi_status(text: &str) -> &'static str {
    match text.trim() {
        "enabled" => "enabled",
        "disabled" => "disabled",
        _ => "unknown",
    }
}

fn parse_wpctl(text: &str) -> Option<(u8, bool)> {
    let lower = text.to_ascii_lowercase();
    let value = lower.strip_prefix("volume:")?.split_whitespace().next()?;
    let fraction: f64 = value.parse().ok()?;
    if !fraction.is_finite() || fraction < 0.0 { return None; }
    let percentage = (fraction * 100.0).round().clamp(0.0, 100.0) as u8;
    Some((percentage, lower.contains("[muted]")))
}

fn parse_pactl(text: &str) -> Option<u8> {
    let index = text.find('%')?;
    let before = text[..index].trim_end();
    let number = before.split_whitespace().last()?.parse::<u16>().ok()?;
    Some(number.min(100) as u8)
}

fn status() {
    let network = execute("nmcli", &["-t", "-f", "STATE", "general"])
        .map_or("unknown", |s| network_status(&s));
    let wifi = execute("nmcli", &["radio", "wifi"])
        .map_or("unknown", |s| wifi_status(&s));
    let audio = execute("wpctl", &["get-volume", "@DEFAULT_AUDIO_SINK@"])
        .ok().and_then(|s| parse_wpctl(&s)).or_else(|| {
            let vol = execute("pactl", &["get-sink-volume", "@DEFAULT_SINK@"])
                .ok().and_then(|s| parse_pactl(&s))?;
            let muted = execute("pactl", &["get-sink-mute", "@DEFAULT_SINK@"])
                .ok()?.to_ascii_lowercase().contains("yes");
            Some((vol, muted))
        });
    let (volume, mute) = audio.map_or(("unknown".to_string(), "unknown"),
        |(vol, muted)| (vol.to_string(), if muted {"yes"} else {"no"}));
    println!("A90|1");
    println!("network|{network}");
    println!("wifi|{wifi}");
    println!("volume|{volume}");
    println!("mute|{mute}");
}

fn action(name: &str) -> Result<(), String> {
    match name {
        "wifi-on" => { execute("nmcli", &["radio", "wifi", "on"])?; }
        "wifi-off" => { execute("nmcli", &["radio", "wifi", "off"])?; }
        "vol-up" => {
            if execute("wpctl", &["set-volume", "-l", "1.0", "@DEFAULT_AUDIO_SINK@", "5%+"]).is_err() {
                let now = execute("pactl", &["get-sink-volume", "@DEFAULT_SINK@"])
                    .ok().and_then(|s| parse_pactl(&s)).ok_or("áudio indisponível")?;
                let next = now.saturating_add(5).min(100).to_string() + "%";
                execute("pactl", &["set-sink-volume", "@DEFAULT_SINK@", &next])?;
            }
        }
        "vol-down" => {
            if execute("wpctl", &["set-volume", "-l", "1.0", "@DEFAULT_AUDIO_SINK@", "5%-"]).is_err() {
                let now = execute("pactl", &["get-sink-volume", "@DEFAULT_SINK@"])
                    .ok().and_then(|s| parse_pactl(&s)).ok_or("áudio indisponível")?;
                let next = now.saturating_sub(5).to_string() + "%";
                execute("pactl", &["set-sink-volume", "@DEFAULT_SINK@", &next])?;
            }
        }
        "mute-toggle" => {
            if execute("wpctl", &["set-mute", "@DEFAULT_AUDIO_SINK@", "toggle"]).is_err() {
                execute("pactl", &["set-sink-mute", "@DEFAULT_SINK@", "toggle"])?;
            }
        }
        _ => return Err("ação não autorizada".to_string()),
    }
    println!("A90|OK|{name}");
    Ok(())
}

fn main() {
    let args: Vec<_> = env::args().skip(1).collect();
    let outcome = match args.as_slice() {
        [command] if command == "status" => { status(); Ok(()) },
        [command] if matches!(command.as_str(), "wifi-on" | "wifi-off" | "vol-up" | "vol-down" | "mute-toggle") => action(command),
        _ => Err("uso: arandu-sistema [status|wifi-on|wifi-off|vol-up|vol-down|mute-toggle]".to_string()),
    };
    if let Err(message) = outcome {
        eprintln!("ARANDU_A90_ERROR: {message}");
        std::process::exit(2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wifi_parse_does_not_leak_network_names() {
        assert_eq!(wifi_status("enabled"), "enabled");
        assert_eq!(wifi_status("MyWifi"), "unknown");
        assert_eq!(network_status("connected (global)"), "connected");
        assert_eq!(network_status("disconnected"), "disconnected");
    }

    #[test]
    fn wpctl_and_pactl_parse_with_clamps() {
        assert_eq!(parse_wpctl("Volume: 0.42"), Some((42, false)));
        assert_eq!(parse_wpctl("Volume: 0.15 [MUTED]"), Some((15, true)));
        assert_eq!(parse_wpctl("Volume: 1.20"), Some((100, false)));
        assert_eq!(parse_wpctl("Volume: NaN"), None);
        assert_eq!(parse_pactl("Volume: front-left: 32768 / 50% / -18.06 dB"), Some(50));
        assert_eq!(parse_pactl("Volume: front-left: 65536 / 100% / 0.00 dB"), Some(100));
    }

    #[test]
    fn invalid_action_refused() {
        assert!(action("shutdown").is_err());
        assert!(action("exec").is_err());
    }
}
