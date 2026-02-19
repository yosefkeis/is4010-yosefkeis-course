users = [
    {"name": "alice", "age": 30, "is_active": True, "email": "alice@example.com"},
    {"name": "bob", "age": 25, "is_active": False},
    {"name": "charlie", "age": 35, "is_active": True, "email": "charlie@example.com"},
    {"name": "david", "age": "unknown", "is_active": False}
]


def calculate_average_age(users):
    """Calculate the average age from a list of user dictionaries.

    Parameters
    ----------
    users : list of dict
        List of user dictionaries which may contain an integer `age` key.

    Returns
    -------
    float
        The average of all integer ages found in `users`. Returns `0.0`
        if no valid integer ages are present or on handled errors.

    Examples
    --------
    >>> calculate_average_age(users)
    30.0
    """
    try:
        total_age = 0
        count = 0
        for user in users:
            age = user.get("age")
            if isinstance(age, int):
                total_age += age
                count += 1

        if count == 0:
            print("error: cannot calculate average age of an empty or invalid list.")
            return 0.0

        return total_age / count
    except Exception as exc:
        print(f"error: unexpected error calculating average age: {exc}")
        return 0.0


def get_active_user_emails(users):
    """Return a list of emails for users marked as active.

    Parameters
    ----------
    users : list of dict
        List of user dictionaries which may contain `is_active` and `email` keys.

    Returns
    -------
    list
        A list of email strings for users where `is_active` is truthy and an
        `email` value is present. Returns an empty list on handled errors.

    Examples
    --------
    >>> get_active_user_emails(users)
    ['alice@example.com', 'charlie@example.com']
    """
    try:
        emails = []
        for user in users:
            # Use .get() to avoid KeyError for missing keys
            if user.get("is_active") and user.get("email"):
                emails.append(user["email"])
        return emails
    except Exception as exc:
        print(f"error: unexpected error getting active user emails: {exc}")
        return []


if __name__ == '__main__':
    avg_age = calculate_average_age(users)
    print(f"average user age: {avg_age:.2f}")

    active_emails = get_active_user_emails(users)
    print(f"active user emails: {active_emails}")
