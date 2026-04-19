"""Weather CLI application with argparse subcommands"""

import argparse
import sys

try:
    from config import WEATHER_API_KEY, WEATHER_API_BASE_URL
except ImportError:
    print("Error: config.py not found!")
    print("Please create config.py from config.example.py")
    sys.exit(1)

from weather_api import WeatherAPI, format_current_weather, format_forecast
from favorites import FavoritesManager


def resolve_location(location: str, favorites: FavoritesManager) -> str:
    """
    Resolve location, checking if it's a favorite first.

    Parameters
    ----------
    location : str
        Location name or favorite name
    favorites : FavoritesManager
        Favorites manager instance

    Returns
    -------
    str
        Resolved location string
    """
    resolved = favorites.get_location(location)
    return resolved if resolved else location


def handle_current(args, api: WeatherAPI, favorites: FavoritesManager) -> None:
    """Handle 'current' subcommand"""
    location = resolve_location(args.location, favorites)
    data = api.get_current_weather(location)

    if data:
        print(format_current_weather(data))
    else:
        print(f"Failed to fetch weather for {location}")


def handle_forecast(args, api: WeatherAPI, favorites: FavoritesManager) -> None:
    """Handle 'forecast' subcommand"""
    location = resolve_location(args.location, favorites)
    days = getattr(args, "days", 3)

    data = api.get_forecast(location, days=days)

    if data:
        print(format_forecast(data))
    else:
        print(f"Failed to fetch forecast for {location}")


def handle_favorites_add(args, favorites: FavoritesManager) -> None:
    """Handle 'favorites add' subcommand"""
    success = favorites.add(args.name, args.location)

    if success:
        print(f"Added favorite: {args.name} -> {args.location}")
    else:
        print(f"Favorite '{args.name}' already exists")


def handle_favorites_list(args, favorites: FavoritesManager) -> None:
    """Handle 'favorites list' subcommand"""
    all_favorites = favorites.list_all()

    if not all_favorites:
        print("No favorites saved yet")
        return

    print("\nSaved Favorites:")
    print("-" * 40)
    for name in sorted(all_favorites):
        location = favorites.get_location(name)
        print(f"  {name}: {location}")
    print("-" * 40)


def handle_favorites_remove(args, favorites: FavoritesManager) -> None:
    """Handle 'favorites remove' subcommand"""
    success = favorites.remove(args.name)

    if success:
        print(f"Removed favorite: {args.name}")
    else:
        print(f"Favorite '{args.name}' not found")


def main():
    """Main entry point for CLI application"""
    parser = argparse.ArgumentParser(
        description="Weather CLI - Get weather forecasts and manage favorites"
    )

    subparsers = parser.add_subparsers(dest="command", help="Available commands")

    # Current weather command
    current_parser = subparsers.add_parser("current", help="Get current weather")
    current_parser.add_argument("location", help="City name or favorite name")

    # Forecast command
    forecast_parser = subparsers.add_parser("forecast", help="Get weather forecast")
    forecast_parser.add_argument("location", help="City name or favorite name")
    forecast_parser.add_argument(
        "--days", type=int, default=3, help="Number of days (1-3)"
    )

    # Favorites subcommand
    favorites_parser = subparsers.add_parser(
        "favorites", help="Manage favorite locations"
    )
    favorites_subparsers = favorites_parser.add_subparsers(
        dest="favorites_command", help="Favorites subcommands"
    )

    # Favorites add
    add_parser = favorites_subparsers.add_parser("add", help="Add a favorite")
    add_parser.add_argument("name", help="Name for the favorite")
    add_parser.add_argument("location", help="Location string")

    # Favorites list
    favorites_subparsers.add_parser("list", help="List all favorites")

    # Favorites remove
    remove_parser = favorites_subparsers.add_parser("remove", help="Remove a favorite")
    remove_parser.add_argument("name", help="Name of favorite to remove")

    args = parser.parse_args()

    # Initialize API and favorites
    api = WeatherAPI(WEATHER_API_KEY, WEATHER_API_BASE_URL)
    favorites = FavoritesManager()

    # Handle commands
    if args.command == "current":
        handle_current(args, api, favorites)
    elif args.command == "forecast":
        handle_forecast(args, api, favorites)
    elif args.command == "favorites":
        if args.favorites_command == "add":
            handle_favorites_add(args, favorites)
        elif args.favorites_command == "list":
            handle_favorites_list(args, favorites)
        elif args.favorites_command == "remove":
            handle_favorites_remove(args, favorites)
        else:
            favorites_parser.print_help()
    else:
        parser.print_help()


if __name__ == "__main__":
    main()
