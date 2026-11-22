#!/usr/bin/env python3
"""A1.8: responsive experimental ARANDU window for Linux Mint.

Tk runs exclusively on the UI thread. One daemon worker owns Rust process
stdin/stdout; a bounded queue passes commands to it. No global input hooks,
session-manager changes, network access or live Seed data.
"""
from __future__ import annotations

import os
import queue
import subprocess
import sys
import threading
import time
import tkinter as tk
from pathlib import Path
from xml.etree import ElementTree as ET

from arandu_window_mint import decode_frame, draw_scene

ROOT = Path(__file__).resolve().parents[1]
BINARY = ROOT / "target" / "debug" / "arandu-cli"
COMMAND_TIMEOUT = 6.0
POLL_MS = 40
COMMANDS = {
    "1": "1", "2": "2", "3": "3", "Escape": "escape",
    "m": "m", "o": "o", "f": "f", "Left": "left",
    "Right": "right", "Up": "up", "Down": "down",
}


class RustBridge:
    """Background worker for the synthetic Rust SVG scene protocol."""

    def __init__(self) -> None:
        self.commands: queue.Queue[str] = queue.Queue(maxsize=24)
        self.events: queue.Queue[tuple[str, str]] = queue.Queue(maxsize=32)
        self.process: subprocess.Popen[str] | None = None
        self.stopping = threading.Event()
        self.thread = threading.Thread(
            target=self._run, name="arandu-rust-bridge", daemon=True
        )

    def start(self) -> None:
        self.thread.start()

    def submit(self, command: str) -> bool:
        if self.stopping.is_set():
            return False
        try:
            self.commands.put_nowait(command)
            return True
        except queue.Full:
            return False

    def _emit(self, kind: str, value: str) -> None:
        if not self.stopping.is_set():
            try:
                self.events.put_nowait((kind, value))
            except queue.Full:
                self.stopping.set()

    def _read_scene(self, process: subprocess.Popen[str]) -> str:
        assert process.stdout is not None
        raw = process.stdout.readline()
        if not raw:
            raise RuntimeError(
                f"Núcleo Rust encerrou a comunicação (código {process.poll()})"
            )
        if len(raw) > 2_000_000:
            raise RuntimeError("Cena excedeu limite de segurança do protocolo")
        decode_frame(raw)
        return raw

    def _run(self) -> None:
        process: subprocess.Popen[str] | None = None
        try:
            process = subprocess.Popen(
                [str(BINARY), "--window-protocol"],
                cwd=ROOT,
                stdin=subprocess.PIPE,
                stdout=subprocess.PIPE,
                stderr=subprocess.DEVNULL,
                text=True,
                bufsize=1,
            )
            self.process = process
            self._emit("scene", self._read_scene(process))
            while not self.stopping.is_set():
                try:
                    command = self.commands.get(timeout=0.1)
                except queue.Empty:
                    continue
                assert process.stdin is not None
                process.stdin.write(command + "\n")
                process.stdin.flush()
                self._emit("scene", self._read_scene(process))
        except (OSError, RuntimeError, ET.ParseError, ValueError) as exc:
            self._emit("error", str(exc))
        finally:
            if process is not None and process.poll() is None:
                process.terminate()

    def stop(self) -> None:
        self.stopping.set()
        process = self.process
        if process is not None and process.poll() is None:
            try:
                process.terminate()
            except ProcessLookupError:
                pass


def main() -> int:
    if not BINARY.is_file():
        print(
            "Binário Rust ausente. Execute: cargo build --locked -p arandu-cli",
            file=sys.stderr,
        )
        return 2

    try:
        root = tk.Tk()
    except tk.TclError as exc:
        print(f"Janela gráfica indisponível: {exc}", file=sys.stderr)
        return 2

    root.title("ARANDU • Confluência — Linux Mint")
    root.geometry("1180x820")
    root.minsize(650, 480)
    root.configure(bg="#09171a")

    header = tk.Frame(root, bg="#102825", height=80)
    header.pack(side="top", fill="x")
    header.pack_propagate(False)
    brand = tk.Frame(header, bg="#102825")
    brand.pack(side="left", fill="y", padx=28, pady=11)
    tk.Label(brand, text="ARANDU", fg="#e9f5ec", bg="#102825",
             font=("Sans", 20, "bold"), anchor="w").pack(anchor="w")
    tk.Label(brand, text="CONFLUÊNCIA  /  SEMENTE  /  TRÊS RIOS",
             fg="#8fbaaa", bg="#102825", font=("Sans", 9),
             anchor="w").pack(anchor="w")
    tk.Label(header, text="LINUX MINT  ·  LABORATÓRIO",
             bg="#102825", fg="#7da899", font=("Sans", 9),
             anchor="e").pack(side="right", padx=26)

    bar = tk.Frame(root, bg="#102825", pady=9, padx=19)
    bar.pack(side="top", fill="x")
    tk.Frame(root, bg="#3e7463", height=2).pack(side="top", fill="x")
    canvas = tk.Canvas(root, bg="#09171a", highlightthickness=0)
    canvas.pack(fill="both", expand=True)
    status = tk.StringVar(value="Iniciando o núcleo Rust…")
    footer = tk.Frame(root, bg="#102825", pady=9)
    footer.pack(side="bottom", fill="x")
    tk.Label(footer, textvariable=status, anchor="w", bg="#102825",
             fg="#dceee6", font=("Sans", 10)).pack(side="left", padx=20)
    tk.Label(footer, text="ESC · RECOLHER     ← → · NAVEGAR",
             bg="#102825", fg="#8fbaaa", font=("Sans", 9)).pack(
                 side="right", padx=20)
    bridge = RustBridge()
    latest: ET.Element | None = None
    pending_since = time.monotonic()
    submitted = 0
    received = 0
    closing = False
    smoke = os.environ.get("ARANDU_GUI_SMOKE") == "1"
    smoke_error = False

    def redraw(_event=None) -> None:
        if latest is not None:
            draw_scene(canvas, latest, max(1, canvas.winfo_width()),
                       max(1, canvas.winfo_height()))

    def send(command: str) -> None:
        nonlocal submitted, pending_since
        if closing:
            return
        if bridge.submit(command):
            submitted += 1
            if submitted == received:
                pending_since = time.monotonic()
            status.set(f"Processando {command}…")
        else:
            status.set("Fila de entrada cheia: comando descartado")

    for label, command in [
        ("1 · Semente", "1"), ("2 · Três Rios", "2"),
        ("3 · Fixar", "3"), ("M · Memória", "m"),
        ("O · Abertos", "o"), ("F · Frequentes", "f"),
    ]:
        button = tk.Button(
            bar, text=label, command=lambda c=command: send(c),
            bg="#1b3c34", fg="#edf7f0",
            activebackground="#457d67", activeforeground="#ffffff",
            relief="flat", bd=0, cursor="hand2",
            font=("Sans", 10), padx=15, pady=10,
        )
        button.pack(side="left", padx=4)
        button.bind("<Enter>", lambda event, widget=button: widget.configure(bg="#315e4d"))
        button.bind("<Leave>", lambda event, widget=button: widget.configure(bg="#1b3c34"))

    def on_key(event) -> None:
        command = COMMANDS.get(event.keysym)
        if command:
            send(command)

    def close() -> None:
        nonlocal closing
        if closing:
            return
        closing = True
        bridge.stop()
        root.destroy()

    def poll() -> None:
        nonlocal latest, received, smoke_error
        if closing:
            return
        try:
            while True:
                kind, payload = bridge.events.get_nowait()
                if kind == "error":
                    status.set("Falha no núcleo Rust: " + payload)
                    smoke_error = True
                    if smoke:
                        close()
                        return
                elif kind == "scene":
                    try:
                        latest = decode_frame(payload)
                    except (ET.ParseError, ValueError) as exc:
                        smoke_error = True
                        status.set(f"Cena inválida: {exc}")
                        if smoke:
                            close()
                            return
                        continue
                    received += 1
                    redraw()
                    status.set(f"Núcleo Rust ativo • quadro {received}")
                    if smoke and received == 1:
                        send("2")
                        send("3")
                        send("escape")
                    elif smoke and received >= 4:
                        root.after(100, close)
        except queue.Empty:
            pass
        outstanding = submitted - max(received - 1, 0)
        if outstanding > 0 and time.monotonic() - pending_since > COMMAND_TIMEOUT:
            status.set("Núcleo Rust sem resposta; janela permanece acessível")
            smoke_error = True
            bridge.stop()
            if smoke:
                close()
                return
        if received == 0 and time.monotonic() - started > COMMAND_TIMEOUT:
            smoke_error = True
            status.set("Falha: núcleo Rust não enviou a cena inicial")
            bridge.stop()
            if smoke:
                close()
                return
        if smoke and time.monotonic() - started > 15:
            smoke_error = True
            close()
            return
        root.after(POLL_MS, poll)

    root.bind("<KeyPress>", on_key)
    canvas.bind("<Configure>", redraw)
    root.protocol("WM_DELETE_WINDOW", close)
    started = time.monotonic()
    bridge.start()
    root.after(POLL_MS, poll)
    root.mainloop()
    if smoke:
        if smoke_error or received < 4:
            print(f"ARANDU_GUI_SMOKE_FAILED: quadros={received}", file=sys.stderr)
            return 1
        print("ARANDU_GUI_SMOKE_OK", flush=True)
    return 0


if __name__ == "__main__":
    sys.exit(main())
