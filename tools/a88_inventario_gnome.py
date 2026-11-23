#!/usr/bin/env python3
"""ARANDU A8.8 — inventário local e consentido do GNOME, somente leitura.

Somente registra presença/versões de pacotes, IDs de extensões e esquemas.
Não lê títulos de janelas, documentos, usuários, SSIDs, processo nem arquivos pessoais.
Não modifica configurações, aplicativos ou sessão GNOME.
"""
from __future__ import annotations

import argparse
import json
import os
import platform
import re
import shutil
import subprocess
from datetime import datetime, timezone
from pathlib import Path

CATEGORIES = {
    "desktop": ["gnome-shell", "gnome-extensions", "gsettings", "gnome-control-center", "gnome-session", "mutter"],
    "network": ["nmcli", "bluetoothctl", "resolvectl"],
    "audio": ["wpctl", "pactl", "pw-cli"],
    "power": ["loginctl", "powerprofilesctl", "upower"],
    "media": ["gnome-screenshot", "grim", "flatpak"],
    "files": ["nautilus", "gio", "udisksctl", "restic", "borg", "tracker3"],
    "hardware": ["xrandr", "wlr-randr", "loginctl", "fc-list"],
    "services": ["systemctl", "gdbus", "busctl", "pkexec"],
    "accessibility": ["orca", "ibus", "xinput"],
}
PACKAGES = [
    "gnome-shell", "mutter", "gnome-session", "gnome-settings-daemon",
    "gnome-control-center", "gnome-software", "gnome-shell-extensions",
    "nautilus", "gnome-calendar", "gnome-system-monitor",
    "gnome-terminal", "gnome-text-editor", "evince", "gnome-screenshot",
    "network-manager", "bluez", "pipewire", "wireplumber",
    "xdg-desktop-portal", "xdg-desktop-portal-gnome",
    "upower", "power-profiles-daemon", "orca", "at-spi2-core",
]

def query(args: list[str], *, timeout: int = 4, max_chars: int = 2400) -> dict:
    if not shutil.which(args[0]):
        return {"ok": False, "reason": "command_missing"}
    try:
        result = subprocess.run(args, capture_output=True, text=True, timeout=timeout, check=False)
        return {"ok": result.returncode == 0, "stdout": (result.stdout or "").strip()[:max_chars],
                "exit_code": result.returncode}
    except (OSError, subprocess.TimeoutExpired):
        return {"ok": False, "reason": "not_available"}

def versions():
    result = {}
    for name in PACKAGES:
        info = query(["dpkg-query", "-W", "-f=${Version}", name])
        if info.get("ok") and info.get("stdout"):
            result[name] = info["stdout"][:64]
    return result

def inventory():
    return {
        "schema": "arandu-a88-local-inventory-v1",
        "generated_utc": datetime.now(timezone.utc).isoformat(),
        "privacy": "No titles, files, paths, account, SSID, clipboard, window or process data collected",
        "platform": {"os": platform.system(), "arch": platform.machine(),
                     "session_type": os.environ.get("XDG_SESSION_TYPE", "unknown"),
                     "desktop": os.environ.get("XDG_CURRENT_DESKTOP", "unknown"),
                     "arandu_session": os.environ.get("ARANDU_SESSION") == "1"},
        "gnome_version": query(["gnome-shell", "--version"]),
        "commands": {category: {cmd: bool(shutil.which(cmd)) for cmd in cmds}
                     for category, cmds in CATEGORIES.items()},
        "package_versions": versions(),
        "gnome_extensions_ids": [],
        "gsettings_schemas_count": None,
    }

def main():
    parser = argparse.ArgumentParser(description="Inventário ARANDU sem mudanças de sistema")
    parser.add_argument("--output", default="", help="Destino opcional do JSON")
    args = parser.parse_args()
    data = inventory()
    ext = query(["gnome-extensions", "list"], timeout=6, max_chars=16000)
    if ext.get("ok"):
        data["gnome_extensions_ids"] = sorted(
            value.strip()[:100] for value in ext.get("stdout", "").splitlines()
            if re.fullmatch(r"[A-Za-z0-9._@-]{2,100}", value.strip()))[:100]
    schemas = query(["gsettings", "list-schemas"], timeout=6, max_chars=128000)
    if schemas.get("ok"):
        data["gsettings_schemas_count"] = len(schemas.get("stdout", "").splitlines())
    output = Path(args.output).expanduser() if args.output else Path.home() / "Downloads" / "arandu_a88_inventario_local.json"
    output.parent.mkdir(parents=True, exist_ok=True)
    old_mask = os.umask(0o077)
    try:
        output.write_text(json.dumps(data, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
        output.chmod(0o600)
    finally:
        os.umask(old_mask)
    print("ARANDU_A88_READ_ONLY_OK")
    print(f"Arquivo: {output}")
    print("Nenhuma configuração, sessão ou aplicativo foi modificado.")
    print("Revise o JSON antes de compartilhá-lo, se desejar.")

if __name__ == "__main__":
    main()
