"""Tests for project inventory module."""

import os

from src.adoption.inventory import (
    Inventory,
    InventoryItem,
    detect_format,
    inventory_project,
)


class TestDetectFormat:
    def test_markdown(self):
        assert detect_format("README.md") == "markdown"
        assert detect_format("notes.txt") == "markdown"

    def test_code(self):
        assert detect_format("main.py") == "code"
        assert detect_format("app.ts") == "code"
        assert detect_format("lib.go") == "code"

    def test_config(self):
        assert detect_format("config.json") == "config"
        assert detect_format("settings.yaml") == "config"
        assert detect_format("pyproject.toml") == "config"

    def test_data(self):
        assert detect_format("data.csv") == "data"

    def test_image(self):
        assert detect_format("logo.png") == "image"
        assert detect_format("photo.jpg") == "image"

    def test_pdf(self):
        assert detect_format("document.pdf") == "pdf"

    def test_unknown(self):
        assert detect_format("file.xyz") == "other"


class TestInventory:
    def test_by_format(self):
        inv = Inventory(".", items=[
            InventoryItem("a.py", "code", 100),
            InventoryItem("b.py", "code", 200),
            InventoryItem("c.md", "markdown", 50),
        ])
        by_fmt = inv.by_format
        assert len(by_fmt["code"]) == 2
        assert len(by_fmt["markdown"]) == 1

    def test_total_size(self):
        inv = Inventory(".", items=[
            InventoryItem("a.py", "code", 100),
            InventoryItem("b.py", "code", 200),
        ])
        assert inv.total_size == 300

    def test_to_markdown(self):
        inv = Inventory(".", items=[
            InventoryItem("src/main.py", "code", 1024),
        ])
        md = inv.to_markdown()
        assert "# Project Inventory" in md
        assert "src/main.py" in md
        assert "1,024b" in md


class TestInventoryProject:
    def test_inventories_files(self, tmp_path):
        (tmp_path / "src").mkdir()
        (tmp_path / "src" / "main.py").write_text("print('hello')")
        (tmp_path / "README.md").write_text("# Project")
        (tmp_path / "config.json").write_text("{}")

        inv = inventory_project(str(tmp_path))
        assert len(inv.items) == 3
        paths = [i.path for i in inv.items]
        assert any("main.py" in p for p in paths)

    def test_skips_hidden(self, tmp_path):
        (tmp_path / ".git").mkdir()
        (tmp_path / ".gitignore").write_text("*.pyc")
        (tmp_path / "visible.py").write_text("code")

        inv = inventory_project(str(tmp_path))
        paths = [i.path for i in inv.items]
        assert not any(".git" in p for p in paths)
        assert any("visible.py" in p for p in paths)

    def test_skips_node_modules(self, tmp_path):
        (tmp_path / "node_modules" / "pkg").mkdir(parents=True)
        (tmp_path / "node_modules" / "pkg" / "index.js").write_text("module.exports = {}")
        (tmp_path / "app.js").write_text("const x = 1")

        inv = inventory_project(str(tmp_path))
        paths = [i.path for i in inv.items]
        assert not any("node_modules" in p for p in paths)

    def test_nonexistent_dir(self):
        inv = inventory_project("/nonexistent/path")
        assert len(inv.items) == 0

    def test_tracks_size_and_mtime(self, tmp_path):
        (tmp_path / "file.txt").write_text("content")
        inv = inventory_project(str(tmp_path))
        assert inv.items[0].size_bytes > 0
        assert inv.items[0].last_modified > 0

    def test_handles_oserror_on_stat(self, tmp_path, monkeypatch):
        (tmp_path / "file.txt").write_text("content")
        original_stat = os.stat

        def failing_stat(path, *args, **kwargs):
            if "file.txt" in str(path):
                raise OSError("Permission denied")
            return original_stat(path, *args, **kwargs)

        monkeypatch.setattr(os, "stat", failing_stat)
        inv = inventory_project(str(tmp_path))
        assert len(inv.items) == 1
        assert inv.items[0].size_bytes == 0
        assert inv.items[0].last_modified == 0.0
