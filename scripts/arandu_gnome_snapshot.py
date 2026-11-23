#!/usr/bin/env python3
"""A2: consulta GNOME Confluência sob demanda e entrega apenas IDs ao Rust.

Não altera o GNOME, não monitora continuamente, não ativa microfone/câmera
e não inicia modelos. Requer a extensão Confluência GNOME já ativa.
"""
from __future__ import annotations

import json
import os
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
BINARY = ROOT / "target" / "debug" / "arandu-cli"
DBUS_NAME = "org.gnome.Shell"
DBUS_PATH = "/org/arandulab/Tape/Windows"
DBUS_IFACE = "org.arandulab.Tape.Windows1"
MAX_WINDOWS = 128
VALID_APP = re.compile(r"[A-Za-z0-9][A-Za-z0-9._-]{0,191}\Z", re.ASCII)


def extract_app_ids(payload: str) -> tuple[str, ...]:
    """Descarta títulos, arquivos, histórico e qualquer dado não necessário."""
    if len(payload.encode("utf-8")) > 256_000:
        raise ValueError("Snapshot GNOME excede o limite permitido")
    data = json.loads(payload)
    if not isinstance(data, dict) or not isinstance(data.get("windows"), list):
        raise ValueError("Snapshot GNOME inválido")
    windows = data["windows"]
    if len(windows) > MAX_WINDOWS:
        raise ValueError("Número de janelas excede o limite")
    ids: list[str] = []
    seen: set[str] = set()
    for item in windows:
        if not isinstance(item, dict):
            raise ValueError("Janela GNOME inválida")
        value = item.get("app")
        if not isinstance(value, str) or not VALID_APP.fullmatch(value):
            raise ValueError("ID GNOME inválido")
        if value not in seen:
            seen.add(value)
            ids.append(value)
    return tuple(ids)


def request_snapshot() -> str:
    """Uma chamada D-Bus local; a extensão conserva o controle da enumeração."""
    try:
        import gi

        from gi.repository import Gio, GLib
    except (ImportError, ValueError) as exc:
        raise RuntimeError("PyGObject/Gio indisponível") from exc
    connection = Gio.bus_get_sync(Gio.BusType.SESSION, None)
    reply = connection.call_sync(
        DBUS_NAME,
        DBUS_PATH,
        DBUS_IFACE,
        "Snapshot",
        None,
        GLib.VariantType("(s)"),
        Gio.DBusCallFlags.NONE,
        3000,
        None,
    )
    if reply is None:
        raise RuntimeError("A extensão não retornou Snapshot")
    return reply.unpack()[0]


def main() -> int:
    if "GNOME" not in os.environ.get("XDG_CURRENT_DESKTOP", "").upper():
        print("A2: execute dentro de uma sessão GNOME.", file=sys.stderr)
        return 2
    if not BINARY.is_file():
        print("A2: compile primeiro: cargo build --locked -p arandu-cli",
              file=sys.stderr)
        return 2
    try:
        ids = extract_app_ids(request_snapshot())
        result = subprocess.run(
            [str(BINARY), "--gnome-apps-stdin"],
            input="".join(f"{app_id}\n" for app_id in ids),
            text=True,
            capture_output=True,
            timeout=5,
            check=True,
        )
    except (OSError, ValueError, RuntimeError, subprocess.SubprocessError) as exc:
        # Nunca registra conteúdo de Snapshot ou títulos de janelas em erros.
        print("A2: leitura local indisponível. Verifique se a extensão "
              "Confluência está ativa e se o Rust foi compilado.",
              file=sys.stderr)
        return 1
    print(result.stdout, end="")
    return 0


if __name__ == "__main__":
    sys.exit(main())
