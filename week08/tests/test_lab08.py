"""Unit tests for Weather CLI application"""

import pytest
import json
import os
from pathlib import Path
from favorites import FavoritesManager


@pytest.fixture
def temp_favorites(tmp_path):
    """Fixture providing a temporary favorites file"""
    favorites_file = tmp_path / "favorites.json"
    return str(favorites_file)


def test_add_favorite(temp_favorites):
    """Test adding a favorite location"""
    manager = FavoritesManager(temp_favorites)
    success = manager.add("home", "Cincinnati, OH")

    assert success is True
    assert manager.get_location("home") == "Cincinnati, OH"


def test_add_duplicate_favorite(temp_favorites):
    """Test adding duplicate favorite returns False"""
    manager = FavoritesManager(temp_favorites)
    manager.add("home", "Cincinnati, OH")
    success = manager.add("home", "Columbus, OH")

    assert success is False
    # Original location should remain
    assert manager.get_location("home") == "Cincinnati, OH"


def test_case_insensitive_add(temp_favorites):
    """Test that adding is case-insensitive"""
    manager = FavoritesManager(temp_favorites)
    manager.add("Home", "Cincinnati, OH")
    success = manager.add("HOME", "Columbus, OH")

    assert success is False
    assert manager.get_location("home") == "Cincinnati, OH"


def test_remove_favorite(temp_favorites):
    """Test removing a favorite"""
    manager = FavoritesManager(temp_favorites)
    manager.add("home", "Cincinnati, OH")
    success = manager.remove("home")

    assert success is True
    assert manager.get_location("home") is None


def test_remove_nonexistent_favorite(temp_favorites):
    """Test removing non-existent favorite returns False"""
    manager = FavoritesManager(temp_favorites)
    success = manager.remove("nonexistent")

    assert success is False


def test_case_insensitive_remove(temp_favorites):
    """Test that removal is case-insensitive"""
    manager = FavoritesManager(temp_favorites)
    manager.add("home", "Cincinnati, OH")
    success = manager.remove("HOME")

    assert success is True
    assert manager.get_location("home") is None


def test_list_all_favorites(temp_favorites):
    """Test listing all favorites"""
    manager = FavoritesManager(temp_favorites)
    manager.add("home", "Cincinnati, OH")
    manager.add("work", "Columbus, OH")
    manager.add("vacation", "Hawaii")

    favorites = manager.list_all()

    assert len(favorites) == 3
    assert "home" in favorites
    assert "work" in favorites
    assert "vacation" in favorites


def test_list_empty_favorites(temp_favorites):
    """Test listing favorites when none exist"""
    manager = FavoritesManager(temp_favorites)
    favorites = manager.list_all()

    assert len(favorites) == 0
    assert favorites == []


def test_case_insensitive_get_location(temp_favorites):
    """Test that get_location is case-insensitive"""
    manager = FavoritesManager(temp_favorites)
    manager.add("home", "Cincinnati, OH")

    assert manager.get_location("home") == "Cincinnati, OH"
    assert manager.get_location("HOME") == "Cincinnati, OH"
    assert manager.get_location("Home") == "Cincinnati, OH"


def test_persistence_across_instances(temp_favorites):
    """Test that favorites persist across manager instances"""
    # Create first manager and add favorites
    manager1 = FavoritesManager(temp_favorites)
    manager1.add("home", "Cincinnati, OH")
    manager1.add("work", "Columbus, OH")

    # Create second manager and verify favorites loaded
    manager2 = FavoritesManager(temp_favorites)
    assert manager2.get_location("home") == "Cincinnati, OH"
    assert manager2.get_location("work") == "Columbus, OH"


def test_save_creates_proper_json(temp_favorites):
    """Test that saved file is valid JSON"""
    manager = FavoritesManager(temp_favorites)
    manager.add("home", "Cincinnati, OH")
    manager.add("work", "Columbus, OH")

    # Read and parse JSON file
    with open(temp_favorites, "r") as f:
        data = json.load(f)

    assert data["home"] == "Cincinnati, OH"
    assert data["work"] == "Columbus, OH"


def test_load_nonexistent_file(tmp_path):
    """Test loading from non-existent file returns empty dict"""
    nonexistent_file = tmp_path / "nonexistent.json"
    manager = FavoritesManager(str(nonexistent_file))

    assert manager.list_all() == []


def test_load_corrupted_json(tmp_path):
    """Test loading corrupted JSON file"""
    corrupted_file = tmp_path / "corrupted.json"

    # Write invalid JSON
    with open(corrupted_file, "w") as f:
        f.write("{ invalid json }")

    # Should handle gracefully and start fresh
    manager = FavoritesManager(str(corrupted_file))
    assert manager.list_all() == []

    # New favorites should work normally
    manager.add("home", "Cincinnati, OH")
    assert manager.get_location("home") == "Cincinnati, OH"


def test_multiple_operations(temp_favorites):
    """Test multiple add/remove operations"""
    manager = FavoritesManager(temp_favorites)

    # Add multiple favorites
    assert manager.add("home", "Cincinnati, OH") is True
    assert manager.add("work", "Columbus, OH") is True
    assert manager.add("travel", "Paris, France") is True

    assert len(manager.list_all()) == 3

    # Remove one
    assert manager.remove("work") is True
    assert len(manager.list_all()) == 2

    # Add different one
    assert manager.add("gym", "Miami, FL") is True
    assert len(manager.list_all()) == 3

    # Try to add duplicate
    assert manager.add("home", "New York") is False
    assert len(manager.list_all()) == 3
    assert manager.get_location("home") == "Cincinnati, OH"
