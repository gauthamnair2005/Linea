#!/usr/bin/env python3
"""Linea third-party package installer for .ln libraries."""

from __future__ import annotations

import argparse
import json
import shutil
from dataclasses import dataclass
from pathlib import Path
from typing import Dict, List, Set
import xml.etree.ElementTree as ET


@dataclass(frozen=True)
class Package:
    name: str
    version: str
    description: str
    developer: str
    entrypoint: str
    dependencies: List[str]


def safe_join(base: Path, rel: str) -> Path:
    candidate = (base / rel).resolve()
    if base.resolve() not in candidate.parents and candidate != base.resolve():
        raise ValueError(f"Unsafe path detected: {rel}")
    return candidate


def load_package(registry: Path, name: str) -> Package:
    meta_path = safe_join(registry / "metadata", f"{name}.xml")
    if not meta_path.exists():
        raise FileNotFoundError(f"Missing metadata file: {meta_path}")

    root = ET.parse(meta_path).getroot()
    pkg_name = root.findtext("name", "").strip() or name
    version = root.findtext("version", "").strip()
    developer = root.findtext("developer", "").strip()
    description = root.findtext("description", "").strip()
    entrypoint = root.findtext("entrypoint", "").strip() or f"libs/{pkg_name}.ln"
    dependencies = []
    deps = root.find("dependencies")
    if deps is not None:
        for dep in deps.findall("dependency"):
            dep_name = (dep.get("name") or dep.text or "").strip()
            if dep_name:
                dependencies.append(dep_name)

    if not version:
        raise ValueError(f"Package {pkg_name} is missing <version> in metadata.")

    return Package(
        name=pkg_name,
        version=version,
        description=description,
        developer=developer,
        entrypoint=entrypoint,
        dependencies=dependencies,
    )


def resolve_install_order(registry: Path, targets: List[str]) -> List[Package]:
    visiting: Set[str] = set()
    visited: Set[str] = set()
    order: List[Package] = []
    package_cache: Dict[str, Package] = {}

    def dfs(pkg_name: str) -> None:
        if pkg_name in visited:
            return
        if pkg_name in visiting:
            raise ValueError(f"Cyclic dependency detected at package: {pkg_name}")

        visiting.add(pkg_name)
        package = package_cache.get(pkg_name) or load_package(registry, pkg_name)
        package_cache[pkg_name] = package

        for dep in package.dependencies:
            dfs(dep)

        visiting.remove(pkg_name)
        visited.add(pkg_name)
        order.append(package)

    for target in targets:
        dfs(target)

    return order


def write_lockfile(lock_path: Path, installed: List[Package], registry: Path) -> None:
    lock_data = {
        "registry": str(registry.resolve()),
        "packages": [
            {
                "name": pkg.name,
                "version": pkg.version,
                "developer": pkg.developer,
                "entrypoint": pkg.entrypoint,
            }
            for pkg in installed
        ],
    }
    lock_path.parent.mkdir(parents=True, exist_ok=True)
    lock_path.write_text(json.dumps(lock_data, indent=2) + "\n", encoding="utf-8")


def install_packages(registry: Path, libs_dir: Path, targets: List[str]) -> None:
    install_order = resolve_install_order(registry, targets)
    libs_dir.mkdir(parents=True, exist_ok=True)
    installed: List[Package] = []

    for pkg in install_order:
        src = safe_join(registry, pkg.entrypoint)
        if not src.exists():
            raise FileNotFoundError(
                f"Package '{pkg.name}' metadata points to missing entrypoint: {src}"
            )

        dest = libs_dir / f"{pkg.name}.ln"
        shutil.copy2(src, dest)
        installed.append(pkg)
        print(f"[installed] {pkg.name}@{pkg.version} -> {dest}")

    write_lockfile(libs_dir / ".linea-packages.lock.json", installed, registry)
    print(f"[ok] Installed {len(installed)} package(s) with dependency resolution.")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Install third-party Linea libraries from an XML registry."
    )
    parser.add_argument(
        "packages",
        nargs="+",
        help="One or more package names to install from registry/metadata/<name>.xml",
    )
    parser.add_argument(
        "--registry",
        required=True,
        help="Path to third-party registry repository",
    )
    parser.add_argument(
        "--libs-dir",
        default="libs",
        help="Destination Linea libs directory (default: ./libs)",
    )
    return parser.parse_args()


def main() -> None:
    args = parse_args()
    registry = Path(args.registry).expanduser().resolve()
    libs_dir = Path(args.libs_dir).expanduser().resolve()

    if not registry.exists():
        raise FileNotFoundError(f"Registry path does not exist: {registry}")
    if not (registry / "metadata").exists() or not (registry / "libs").exists():
        raise FileNotFoundError(
            f"Registry must contain 'metadata/' and 'libs/' folders: {registry}"
        )

    install_packages(registry, libs_dir, args.packages)


if __name__ == "__main__":
    main()
