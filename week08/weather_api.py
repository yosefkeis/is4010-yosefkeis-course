"""Weather API client for fetching weather data from WeatherAPI.com"""

import requests
from typing import Optional, Dict, Any


class WeatherAPI:
    """Client for interacting with WeatherAPI.com"""

    def __init__(self, api_key: str, base_url: str = "http://api.weatherapi.com/v1"):
        """
        Initialize WeatherAPI client.

        Parameters
        ----------
        api_key : str
            WeatherAPI.com API key
        base_url : str
            Base URL for API endpoints
        """
        self.api_key = api_key
        self.base_url = base_url

    def get_current_weather(self, location: str) -> Optional[Dict[str, Any]]:
        """
        Fetch current weather for a location.

        Parameters
        ----------
        location : str
            Location name (e.g., "London" or "New York, USA")

        Returns
        -------
        dict or None
            Weather data or None if request fails
        """
        url = f"{self.base_url}/current.json"
        params = {"key": self.api_key, "q": location}

        try:
            response = requests.get(url, params=params)
            response.raise_for_status()
            return response.json()
        except requests.exceptions.RequestException as e:
            print(f"Error fetching current weather: {e}")
            return None

    def get_forecast(
        self, location: str, days: int = 3
    ) -> Optional[Dict[str, Any]]:
        """
        Fetch weather forecast for a location.

        Parameters
        ----------
        location : str
            Location name
        days : int
            Number of days (1-3)

        Returns
        -------
        dict or None
            Forecast data or None if request fails
        """
        url = f"{self.base_url}/forecast.json"
        params = {"key": self.api_key, "q": location, "days": days}

        try:
            response = requests.get(url, params=params)
            response.raise_for_status()
            return response.json()
        except requests.exceptions.RequestException as e:
            print(f"Error fetching forecast: {e}")
            return None


def format_current_weather(data: Dict[str, Any]) -> str:
    """
    Format current weather data for display.

    Parameters
    ----------
    data : dict
        Weather data from API

    Returns
    -------
    str
        Formatted weather string
    """
    if not data or "current" not in data:
        return "Invalid weather data"

    current = data["current"]
    location = data["location"]

    output = "\n" + "=" * 50 + "\n"
    output += f"Current Weather for {location['name']}, {location['country']}\n"
    output += "=" * 50 + "\n"
    output += f"Condition: {current['condition']['text']}\n"
    output += f"Temperature: {current['temp_f']}°F ({current['temp_c']}°C)\n"
    output += f"Feels Like: {current['feelslike_f']}°F ({current['feelslike_c']}°C)\n"
    output += f"Humidity: {current['humidity']}%\n"
    output += f"Wind: {current['wind_mph']} mph {current['wind_dir']}\n"
    output += f"Last Updated: {current['last_updated']}\n"
    output += "=" * 50 + "\n"

    return output


def format_forecast(data: Dict[str, Any]) -> str:
    """
    Format forecast data for display.

    Parameters
    ----------
    data : dict
        Forecast data from API

    Returns
    -------
    str
        Formatted forecast string
    """
    if not data or "forecast" not in data:
        return "Invalid forecast data"

    location = data["location"]
    output = "\n" + "=" * 50 + "\n"
    output += f"Forecast for {location['name']}, {location['country']}\n"
    output += "=" * 50 + "\n"

    for day in data["forecast"]["forecastday"]:
        output += f"\n{day['date']}\n"
        output += f"  Condition: {day['day']['condition']['text']}\n"
        output += f"  High: {day['day']['maxtemp_f']}°F ({day['day']['maxtemp_c']}°C)\n"
        output += f"  Low: {day['day']['mintemp_f']}°F ({day['day']['mintemp_c']}°C)\n"
        output += f"  Chance of Rain: {day['day']['daily_chance_of_rain']}%\n"

    output += "=" * 50 + "\n"
    return output
