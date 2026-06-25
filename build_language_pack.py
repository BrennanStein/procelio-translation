#!/usr/bin/env python3
"""
Build a Unity-ready language pack by merging:
1) translated text from files/<LANG>/language.json
2) shared baseline styles from files/shared_style.json
3) optional per-language overrides from files/<LANG>/style.json

The script writes a generated output folder
"""

from __future__ import annotations

import argparse
import copy
import json
import shutil
from pathlib import Path
from typing import Any


REPO_ROOT = Path(__file__).resolve().parent
FILES_DIR = REPO_ROOT / "files"
DEFAULT_SHARED_STYLE = FILES_DIR / "shared_style.json"
DEFAULT_SOURCE_LANGUAGE = "ENGLISH"


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Build a merged language pack folder for Unity/proceliotool."
    )
    parser.add_argument(
        "language_dir",
        nargs="?",
        help="Language folder inside files/, for example GERMAN.",
    )
    parser.add_argument(
        "--shared-style",
        default=str(DEFAULT_SHARED_STYLE),
        help=f"Shared style baseline file (default: {DEFAULT_SHARED_STYLE}).",
    )
    parser.add_argument(
        "--source-language",
        default=DEFAULT_SOURCE_LANGUAGE,
        help=f"Canonical source language folder used to fill missing entries (default: {DEFAULT_SOURCE_LANGUAGE}).",
    )
    parser.add_argument(
        "--output-dir",
        help="Output directory. Defaults to build/<LANGUAGE>/.",
    )
    return parser.parse_args()


def read_json(path: Path) -> Any:
    with path.open(encoding="utf-8") as handle:
        return json.load(handle)


def write_json(path: Path, payload: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w", encoding="utf-8") as handle:
        json.dump(payload, handle, ensure_ascii=False, indent=2)
        handle.write("\n")


def merge_language_pack(args: argparse.Namespace) -> None:
    source_dir = FILES_DIR / args.source_language
    source_json = source_dir / "language.json"
    language_dir = FILES_DIR / args.language_dir
    language_json = language_dir / "language.json"
    style_json = language_dir / "style.json"
    image_png = language_dir / "image.png"
    shared_style_json = Path(args.shared_style)

    if not source_json.exists():
        raise SystemExit(f"Source language file does not exist: {source_json}")
    if not language_json.exists():
        raise SystemExit(f"Language file does not exist: {language_json}")
    if not image_png.exists():
        raise SystemExit(f"Image file does not exist: {image_png}")
    if not shared_style_json.exists():
        raise SystemExit(f"Shared style file does not exist: {shared_style_json}")

    source_data = read_json(source_json)
    language_data = read_json(language_json)
    shared_style_data = read_json(shared_style_json)
    shared_by_key = shared_style_data.get("by_key", {})

    language_style_data = read_json(style_json) if style_json.exists() else {"by_key": {}}
    lang_by_key = language_style_data.get("by_key", {})

    output_dir = (
        Path(args.output_dir)
        if args.output_dir
        else REPO_ROOT / "build" / args.language_dir
    )
    output_json = output_dir / "language.json"
    output_image = output_dir / "image.png"

    source_by_name = {
        entry["name"]: entry for entry in source_data.get("language_elements", [])
    }
    language_by_name = {
        entry["name"]: entry for entry in language_data.get("language_elements", [])
    }

    merged = copy.deepcopy(language_data)
    merged_elements = []

    for source_entry in source_data.get("language_elements", []):
        name = source_entry["name"]
        entry = language_by_name.get(name, source_entry)
        merged_entry = {"name": name, "value": entry.get("value", source_entry.get("value", ""))}

        # shared baseline from English
        shared_style = shared_by_key.get(name, {})
        for key, value in shared_style.items():
            merged_entry[key] = value

        # language-specific style.json overrides
        style_override = lang_by_key.get(name, {})
        for key, value in style_override.items():
            merged_entry[key] = value

        merged_elements.append(merged_entry)

    merged["language_elements"] = merged_elements

    output_dir.mkdir(parents=True, exist_ok=True)
    write_json(output_json, merged)
    shutil.copy2(image_png, output_image)

    print(f"Built merged pack for {args.language_dir}")
    print(f"language.json: {output_json}")
    print(f"image.png:      {output_image}")
    print(f"shared style:   {shared_style_json}")
    print(f"style override: {style_json}")


def main() -> None:
    args = parse_args()
    if args.language_dir:
        merge_language_pack(args)
        return

    language_dirs = sorted(
        path.name
        for path in FILES_DIR.iterdir()
        if path.is_dir() and (path / "language.json").exists() and (path / "image.png").exists()
    )
    if not language_dirs:
        raise SystemExit("No language folders with language.json and image.png were found.")

    for language_dir in language_dirs:
        child_args = argparse.Namespace(**vars(args))
        child_args.language_dir = language_dir
        if child_args.output_dir:
            child_args.output_dir = str(Path(child_args.output_dir) / language_dir)
        merge_language_pack(child_args)


if __name__ == "__main__":
    main()
