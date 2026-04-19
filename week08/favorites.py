"""Favorites manager for storing favorite weather locations"""

import json
import os
from typing import List, Optional, Dict


class FavoritesManager:
    """Manages favorite locations stored in JSON file"""

    def __init__(self, filename: str = "favorites.json"):
        """
        Initialize FavoritesManager.

        Parameters
        ----------
        filename : str
            Path to favorites JSON file
        """
        self.filename = filename
        self.favorites = self._load_favorites()

    def _load_favorites(self) -> Dict[str, str]:
        """
        Load favorites from JSON file.

        Returns
        -------
        dict
            Dictionary of {name: location} pairs
        """
        try:
            with open(self.filename, "r") as f:
                return json.load(f)
        except FileNotFoundError:
            return {}
        except json.JSONDecodeError:
            print(f"Warning: {self.filename} is corrupted, starting fresh")
            return {}

    def _save_favorites(self) -> None:
        """Save favorites to JSON file"""
        with open(self.filename, "w") as f:
            json.dump(self.favorites, f, indent=4)

    def add(self, name: str, location: str) -> bool:
        """
        Add a favorite location.

        Parameters
        ----------
        name : str
            Name for the favorite
        location : str
            Location string (e.g., "London" or "New York, USA")

        Returns
        -------
        bool
            True if added, False if already exists
        """
        name_lower = name.lower()
        if name_lower in self.favorites:
            return False

        self.favorites[name_lower] = location
        self._save_favorites()
        return True

    def remove(self, name: str) -> bool:
        """
        Remove a favorite location.

        Parameters
        ----------
        name : str
            Name of favorite to remove

        Returns
        -------
        bool
            True if removed, False if not found
        """
        name_lower = name.lower()
        if name_lower not in self.favorites:
            return False

        del self.favorites[name_lower]
        self._save_favorites()
        return True

    def list_all(self) -> List[str]:
        """
        Get all favorite names.

        Returns
        -------
        list
            List of favorite names
        """
        return list(self.favorites.keys())

    def get_location(self, name: str) -> Optional[str]:
        """
        Get location for a favorite name.

        Parameters
        ----------
        name : str
            Name of favorite

        Returns
        -------
        str or None
            Location string if found, None otherwise
        """
        return self.favorites.get(name.lower())
