#!/usr/bin/env python3
"""ARANDU A1.5: isolated Linux Mint window for Rust-generated SVG scenes.

Temporary Tk adapter, not a Wayland compositor. Rust owns interaction state,
navigation and scene geometry. Python only presents native window controls.
Requires Python 3 with tkinter plus Cargo/Rust in the checkout.
"""
from __future__ import annotations

import subprocess
import sys
import tkinter as tk
from pathlib import Path
from xml.etree import ElementTree as ET

ROOT = Path(__file__).resolve().parents[1]
SVG_NS = "{http://www.w3.org/2000/svg}"
VIEW_WIDTH = 1100
VIEW_HEIGHT = 720


def decode_frame(line: str) -> ET.Element:
    if not line.startswith("SCENE <svg "):
        raise ValueError("Invalid Rust scene response")
    root = ET.fromstring(line.removeprefix("SCENE "))
    if root.tag != SVG_NS + "svg":
        raise ValueError("Unexpected scene root")
    return root


def draw_scene(canvas: tk.Canvas, scene: ET.Element, width: int, height: int) -> None:
    canvas.delete("all")
    scale = min(width / VIEW_WIDTH, height / VIEW_HEIGHT)
    offset_x = (width - VIEW_WIDTH * scale) / 2
    offset_y = (height - VIEW_HEIGHT * scale) / 2

    def x(value: str) -> float:
        return offset_x + float(value) * scale

    def y(value: str) -> float:
        return offset_y + float(value) * scale

    for element in scene:
        tag = element.tag.removeprefix(SVG_NS)
        color = element.get("fill", "#dceee6")
        if tag == "rect":
            canvas.create_rectangle(0, 0, width, height, fill=color, outline="")
        elif tag == "line":
            canvas.create_line(
                x(element.attrib["x1"]), y(element.attrib["y1"]),
                x(element.attrib["x2"]), y(element.attrib["y2"]),
                fill=element.get("stroke", "#60aa91"),
                width=max(1, float(element.get("stroke-width", "1")) * scale),
            )
        elif tag == "circle":
            cx = x(element.attrib["cx"])
            cy = y(element.attrib["cy"])
            radius = float(element.attrib["r"]) * scale
            canvas.create_oval(
                cx - radius, cy - radius, cx + radius, cy + radius,
                fill=color, outline=element.get("stroke", ""),
                width=max(1, float(element.get("stroke-width", "1")) * scale),
            )
        elif tag == "text":
            anchor = {"middle": "center", "start": "w", "end": "e"}.get(
                element.get("text-anchor", "start"), "w"
            )
            font_size = max(8, round(float(element.get("font-size", "13")) * scale))
            canvas.create_text(
                x(element.attrib["x"]), y(element.attrib["y"]),
                text=element.text or "", fill=color, anchor=anchor,
                font=("Sans", font_size),
            )


def main() -> int:
    try:
        process = subprocess.Popen(
            ["cargo", "run", "--locked", "--quiet", "-p", "arandu-cli",
             "--", "--window-protocol"],
            cwd=ROOT, stdin=subprocess.PIPE, stdout=subprocess.PIPE,
            text=True, bufsize=1,
        )
    except OSError as exc:
        print(f"Unable to start Rust core: {exc}", file=sys.stderr)
        return 1

    assert process.stdin is not None
    assert process.stdout is not None
    try:
        initial = decode_frame(process.stdout.readline())
    except (ValueError, ET.ParseError) as exc:
        print(f"Rust scene unavailable: {exc}", file=sys.stderr)
        process.terminate()
        return 1

    root = tk.Tk()
    root.title("ARANDU A1.5 — Linux Mint (laboratório)")
    root.geometry("1100x780")
    root.minsize(550, 420)
    root.configure(bg="#09171a")

    toolbar = tk.Frame(root, bg="#102e2d")
    toolbar.pack(side="top", fill="x")
    canvas = tk.Canvas(root, bg="#09171a", highlightthickness=0)
    canvas.pack(fill="both", expand=True)
    scene = initial

    def redraw(_event=None) -> None:
        draw_scene(canvas, scene, max(canvas.winfo_width(), 1),
                   max(canvas.winfo_height(), 1))

    def send(command: str) -> None:
        nonlocal scene
        if process.poll() is not None:
            root.title("ARANDU — Núcleo Rust encerrado")
            return
        try:
            process.stdin.write(command + "\n")
            process.stdin.flush()
            scene = decode_frame(process.stdout.readline())
            redraw()
        except (OSError, ValueError, ET.ParseError) as exc:
            root.title(f"ARANDU — erro de protocolo: {exc}")

    for label, command in [
        ("1 · Semente", "1"),
        ("2 · Três Rios", "2"),
        ("3 · Fixar", "3"),
        ("M · Memória", "m"),
        ("O · Abertos", "o"),
        ("F · Frequentes", "f"),
    ]:
        tk.Button(toolbar, text=label, command=lambda c=command: send(c),
                  bg="#15372e", fg="#f1f8f5",
                  activebackground="#386957", activeforeground="#ffffff",
                  relief="flat", padx=9, pady=8).pack(side="left", padx=2)

    keys = {
        "1": "1", "2": "2", "3": "3", "Escape": "escape",
        "m": "m", "o": "o", "f": "f",
        "Left": "left", "Right": "right",
        "Up": "up", "Down": "down",
    }

    def on_key(event) -> None:
        command = keys.get(event.keysym)
        if command:
            send(command)

    def close() -> None:
        try:
            if process.poll() is None:
                process.stdin.write("quit\n")
                process.stdin.flush()
                process.wait(timeout=1)
        except (OSError, subprocess.TimeoutExpired):
            process.terminate()
        root.destroy()

    root.bind("<KeyPress>", on_key)
    canvas.bind("<Configure>", redraw)
    root.protocol("WM_DELETE_WINDOW", close)
    redraw()
    root.mainloop()
    return 0


if __name__ == "__main__":
    sys.exit(main())
