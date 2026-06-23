#!/usr/bin/env python3
"""
Bootstrap and run automated Procelio translation generation.

The script creates or reuses a temporary virtual environment, installs the
required translation package, and translates a selected language pack from the
English source.
"""

from __future__ import annotations

import argparse
import copy
import importlib.util
import json
import os
import re
import shutil
import subprocess
import sys
import tempfile
import time
from pathlib import Path
from typing import Any


REPO_ROOT = Path(__file__).resolve().parent
FILES_DIR = REPO_ROOT / "files"
TEMPLATES_DIR = REPO_ROOT / "templates"
DEFAULT_SOURCE_DIR = "ENGLISH"
DEFAULT_PACKAGE = "deep-translator==1.11.4"
CHILD_ENVVAR = "PROCELIO_TRANSLATE_CHILD"
PLACEHOLDER_RE = re.compile(r"(<[^>]+>|%[A-Z_]+%|%s|%d|\$\{[^}]+\})")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Create or update a translated language pack from ENGLISH."
    )
    parser.add_argument(
        "language_dir",
        help="Target folder inside files/, for example GERMAN or FRENCH.",
    )
    parser.add_argument(
        "--target-code",
        help="Translator target code, for example de, fr, es, it.",
    )
    parser.add_argument(
        "--source-dir",
        default=DEFAULT_SOURCE_DIR,
        help=f"Source folder inside files/ (default: {DEFAULT_SOURCE_DIR}).",
    )
    parser.add_argument(
        "--anglicized-name",
        help="Language name in English ASCII, for example German.",
    )
    parser.add_argument(
        "--native-name",
        help="Language name in the target language, for example Deutsch.",
    )
    parser.add_argument(
        "--authors",
        help="Author attribution to write into language.json.",
    )
    parser.add_argument(
        "--venv-dir",
        default=str(Path(tempfile.gettempdir()) / "procelio-translate-venv"),
        help="Temporary virtual environment directory.",
    )
    parser.add_argument(
        "--package",
        default=DEFAULT_PACKAGE,
        help=f"Translation package to install in the temp venv (default: {DEFAULT_PACKAGE}).",
    )
    parser.add_argument(
        "--glossary-path",
        help="Optional glossary file path. Defaults to files/<LANG>/glossary.json.",
    )
    parser.add_argument(
        "--cache-path",
        help="Optional cache file path. Defaults to files/<LANG>/translation_cache.json.",
    )
    parser.add_argument(
        "--force-retranslate",
        action="store_true",
        help="Ignore the stored translation cache and retranslate non-glossary values.",
    )
    parser.add_argument(
        "--no-reuse-existing",
        action="store_true",
        help="Do not seed the cache from an existing target language.json.",
    )
    parser.add_argument(
        "--no-install",
        action="store_true",
        help="Fail instead of installing the translation package when it is missing.",
    )
    parser.add_argument(
        "--write-glossary-template-only",
        action="store_true",
        help="Create the glossary file if missing and exit without translating.",
    )
    parser.add_argument(
        "--rebuild-cache-only",
        action="store_true",
        help="Refresh translation_cache.json from the current target language.json and exit.",
    )
    return parser.parse_args()


def venv_python(venv_dir: Path) -> Path:
    if os.name == "nt":
        return venv_dir / "Scripts" / "python.exe"
    return venv_dir / "bin" / "python"


def run(cmd: list[str], *, env: dict[str, str] | None = None) -> None:
    subprocess.run(cmd, check=True, env=env)


def ensure_bootstrap_environment(args: argparse.Namespace) -> None:
    if os.environ.get(CHILD_ENVVAR) == "1":
        return
    if args.write_glossary_template_only or args.rebuild_cache_only:
        return

    venv_dir = Path(args.venv_dir)
    python_in_venv = venv_python(venv_dir)

    if not python_in_venv.exists():
        print(f"[bootstrap] Creating temporary virtual environment at {venv_dir}")
        run([sys.executable, "-m", "venv", str(venv_dir)])

    package_name = args.package.split("==", 1)[0].replace("-", "_")
    check_cmd = [
        str(python_in_venv),
        "-c",
        (
            "import importlib.util, sys; "
            f"sys.exit(0 if importlib.util.find_spec('{package_name}') else 1)"
        ),
    ]
    package_missing = subprocess.run(check_cmd).returncode != 0

    if package_missing:
        if args.no_install:
            raise SystemExit(
                f"Required package {args.package!r} is missing in {venv_dir}. "
                "Re-run without --no-install."
            )
        print(f"[bootstrap] Installing {args.package} into {venv_dir}")
        run([str(python_in_venv), "-m", "pip", "install", args.package])

    child_env = dict(os.environ)
    child_env[CHILD_ENVVAR] = "1"
    child_cmd = [str(python_in_venv), str(Path(__file__).resolve()), *sys.argv[1:]]
    run(child_cmd, env=child_env)
    raise SystemExit(0)


def default_glossary_path(language_dir: str) -> Path:
    return FILES_DIR / language_dir / "glossary.json"


def default_cache_path(language_dir: str) -> Path:
    return FILES_DIR / language_dir / "translation_cache.json"


def default_style_path(language_dir: str) -> Path:
    return FILES_DIR / language_dir / "style.json"


def read_json(path: Path) -> Any:
    with path.open(encoding="utf-8") as handle:
        return json.load(handle)


def write_json(path: Path, payload: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w", encoding="utf-8") as handle:
        json.dump(payload, handle, ensure_ascii=False, indent=2)
        handle.write("\n")


def copy_template_if_missing(path: Path, template_name: str) -> dict[str, Any]:
    if not path.exists():
        template_path = TEMPLATES_DIR / template_name
        if not template_path.exists():
            raise SystemExit(f"Template file does not exist: {template_path}")
        path.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(template_path, path)
    return read_json(path)


def ensure_glossary(path: Path) -> dict[str, Any]:
    payload = copy_template_if_missing(path, "glossary.json")

    payload.setdefault("meta", {})
    payload.setdefault("by_key", {})
    payload.setdefault("by_source_text", {})
    payload.setdefault("post_replace", [])
    return payload


def protect_text(text: str) -> tuple[str, list[tuple[str, str]]]:
    replacements: list[tuple[str, str]] = []

    def replace(match: re.Match[str]) -> str:
        token = f"__PROCELIO_TOKEN_{len(replacements)}__"
        replacements.append((token, match.group(0)))
        return token

    return PLACEHOLDER_RE.sub(replace, text), replacements


def unprotect_text(text: str, replacements: list[tuple[str, str]]) -> str:
    for token, original in replacements:
        text = text.replace(token, original)
    return text


def apply_post_replace(text: str, glossary: dict[str, Any]) -> str:
    for pair in glossary.get("post_replace", []):
        if not isinstance(pair, list) or len(pair) != 2:
            continue
        before, after = pair
        text = text.replace(before, after)
    return text


def build_existing_translation_map(
    source_data: dict[str, Any],
    target_data: dict[str, Any] | None,
) -> dict[str, str]:
    if not target_data:
        return {}

    target_by_name = {
        entry["name"]: entry
        for entry in target_data.get("language_elements", [])
        if "name" in entry
    }
    mapping: dict[str, str] = {}
    for source_entry in source_data.get("language_elements", []):
        target_entry = target_by_name.get(source_entry["name"])
        if not target_entry:
            continue
        source_value = source_entry.get("value", "")
        target_value = target_entry.get("value", "")
        if source_value and target_value and source_value != target_value:
            mapping.setdefault(source_value, target_value)
    return mapping


def rebuild_cache_from_existing_translation(
    source_data: dict[str, Any],
    target_data: dict[str, Any] | None,
    existing_cache: dict[str, str] | None = None,
) -> dict[str, str]:
    refreshed_cache = dict(existing_cache or {})
    refreshed_cache.update(build_existing_translation_map(source_data, target_data))
    return refreshed_cache


def import_translator() -> Any:
    if importlib.util.find_spec("deep_translator") is None:
        raise SystemExit(
            "deep-translator is not available in the current environment. "
            "Re-run the script without --no-install so it can bootstrap itself."
        )
    from deep_translator import GoogleTranslator  # type: ignore

    return GoogleTranslator


def translate_value(
    translator: Any,
    text: str,
    glossary: dict[str, Any],
    cache: dict[str, str],
    *,
    key_name: str,
    force_retranslate: bool,
) -> str:
    if key_name in glossary["by_key"]:
        return glossary["by_key"][key_name]
    if text in glossary["by_source_text"]:
        return glossary["by_source_text"][text]
    if not text:
        return text
    if not force_retranslate and text in cache:
        return apply_post_replace(cache[text], glossary)

    protected_text, replacements = protect_text(text)

    for attempt in range(5):
        try:
            translated = translator.translate(protected_text)
            break
        except Exception as exc:  # pragma: no cover - network dependent
            if attempt == 4:
                raise SystemExit(f"Translation failed for {key_name!r}: {exc}") from exc
            time.sleep(1.5 * (attempt + 1))
    translated = unprotect_text(translated, replacements)
    translated = apply_post_replace(translated, glossary)
    cache[text] = translated
    return translated


def build_target_entry(
    source_entry: dict[str, Any],
    translated_value: str,
) -> dict[str, Any]:
    return {"name": source_entry["name"], "value": translated_value}


def translate_language(args: argparse.Namespace) -> None:
    source_dir = FILES_DIR / args.source_dir
    target_dir = FILES_DIR / args.language_dir
    source_json = source_dir / "language.json"
    source_image = source_dir / "image.png"
    target_json = target_dir / "language.json"
    target_image = target_dir / "image.png"

    if not source_json.exists():
        raise SystemExit(f"Source language file does not exist: {source_json}")
    if not source_image.exists():
        raise SystemExit(f"Source image file does not exist: {source_image}")

    glossary_path = (
        Path(args.glossary_path)
        if args.glossary_path
        else default_glossary_path(args.language_dir)
    )
    style_path = default_style_path(args.language_dir)
    cache_path = (
        Path(args.cache_path) if args.cache_path else default_cache_path(args.language_dir)
    )

    glossary = ensure_glossary(glossary_path)
    copy_template_if_missing(style_path, "style.json")
    if args.write_glossary_template_only:
        print(f"Wrote glossary template to {glossary_path}")
        print(f"Ensured style template at {style_path}")
        return

    target_dir.mkdir(parents=True, exist_ok=True)
    if not target_image.exists():
        shutil.copy2(source_image, target_image)

    source_data = read_json(source_json)
    existing_target = read_json(target_json) if target_json.exists() else None
    cache = read_json(cache_path) if cache_path.exists() else {}

    if not args.no_reuse_existing:
        cache = rebuild_cache_from_existing_translation(
            source_data,
            existing_target,
            cache,
        )

    if args.rebuild_cache_only:
        write_json(cache_path, cache)
        print(f"Rebuilt cache for {args.language_dir} from current language.json")
        print(f"cache: {cache_path}")
        return

    if not args.target_code:
        raise SystemExit("--target-code is required unless --rebuild-cache-only is used.")

    GoogleTranslator = import_translator()
    translator = GoogleTranslator(source="en", target=args.target_code)

    existing_by_name = (
        {
            entry["name"]: entry
            for entry in existing_target.get("language_elements", [])
            if "name" in entry
        }
        if existing_target
        else {}
    )

    translated_elements = []
    unique_translated = 0
    for source_entry in source_data.get("language_elements", []):
        source_value = source_entry.get("value", "")
        translated_value = translate_value(
            translator,
            source_value,
            glossary,
            cache,
            key_name=source_entry["name"],
            force_retranslate=args.force_retranslate,
        )
        if source_value and source_value not in glossary["by_source_text"]:
            unique_translated += int(source_value in cache)
        translated_elements.append(
            build_target_entry(
                source_entry,
                translated_value,
            )
        )

    output = copy.deepcopy(source_data)
    output["language_elements"] = translated_elements
    output["anglicized_name"] = (
        args.anglicized_name
        or (existing_target or {}).get("anglicized_name")
        or args.language_dir.title()
    )
    output["native_name"] = (
        args.native_name
        or (existing_target or {}).get("native_name")
        or args.language_dir.title()
    )
    output["authors"] = (
        args.authors
        or (existing_target or {}).get("authors")
        or "Unknown"
    )
    if existing_target and "version" in existing_target:
        output["version"] = existing_target["version"]

    write_json(target_json, output)
    write_json(cache_path, cache)

    print(f"Translated {args.language_dir} from {args.source_dir}")
    print(f"language.json: {target_json}")
    print(f"image.png:      {target_image}")
    print(f"glossary:       {glossary_path}")
    print(f"cache:          {cache_path}")


def main() -> None:
    args = parse_args()
    ensure_bootstrap_environment(args)
    translate_language(args)


if __name__ == "__main__":
    main()
