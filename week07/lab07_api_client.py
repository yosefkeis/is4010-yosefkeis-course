import requests


def get_api_data(url):
    """
    Fetches and parses JSON data from a given API url.

    Parameters
    ----------
    url : str
        The URL of the API endpoint.

    Returns
    -------
    dict or None
        A dictionary containing the parsed JSON data, or None if
        the request fails or the response is not valid JSON.
    """
    # Use a try...except block to handle potential requests.exceptions.RequestException
    try:
        # Make a GET request to the URL
        response = requests.get(url)
        
        # Raise an HTTPError if the response was an error
        response.raise_for_status()

        # Parse and return the JSON data
        return response.json()

    except requests.exceptions.RequestException as e:
        print(f"Error making request: {e}")
        return None
    except requests.exceptions.JSONDecodeError:
        print("Error: Failed to decode JSON from response.")
        return None


if __name__ == '__main__':
    # Example using the Pokémon API
    pokemon_url = "https://pokeapi.co/api/v2/pokemon/snorlax"
    
    # Get the data from the API
    pokemon_data = get_api_data(pokemon_url)

    # If data was successfully fetched, display some of it
    if pokemon_data:
        print(f"Successfully fetched data for: {pokemon_data['name'].title()}")
        print(f"Weight: {pokemon_data['weight']} hectograms")
        print("Abilities:")
        for ability in pokemon_data['abilities']:
            print(f"  - {ability['ability']['name']}")
