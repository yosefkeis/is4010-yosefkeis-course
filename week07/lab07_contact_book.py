import json


def save_contacts_to_json(contacts, filename):
    """
    Saves a list of contacts (dictionaries) to a file in JSON format.

    Parameters
    ----------
    contacts : list
        A list of contact dictionaries.
    filename : str
        The name of the file to save the contacts to.
    """
    # Use with open(...) and json.dump() to write the contacts list
    # to the specified file. Use an indent of 4 for readability.
    with open(filename, 'w') as f:
        json.dump(contacts, f, indent=4)


def load_contacts_from_json(filename):
    """
    Loads a list of contacts from a JSON file.

    Parameters
    ----------
    filename : str
        The name of the file to load contacts from.

    Returns
    -------
    list
        A list of contact dictionaries. Returns an empty list if the
        file does not exist.
    """
    # Use a try...except block to handle the FileNotFoundError.
    # If the file exists, use with open(...) and json.load() to read
    # and return the contacts.
    # If the file does not exist, return an empty list.
    try:
        with open(filename, 'r') as f:
            return json.load(f)
    except FileNotFoundError:
        return []


if __name__ == '__main__':
    # Main execution block to test the functions
    contacts_file = 'contacts.json'

    # Try to load existing contacts
    my_contacts = load_contacts_from_json(contacts_file)
    print(f"Loaded {len(my_contacts)} contact(s).")

    # Add a new contact (as a dictionary)
    new_contact = {"name": "Charles Babbage", "email": "charles@computers.org"}
    my_contacts.append(new_contact)
    print(f"Added a new contact for {new_contact['name']}.")

    # Save the updated list of contacts
    save_contacts_to_json(my_contacts, contacts_file)
    print("Saved contacts to disk.")
