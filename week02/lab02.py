def factorial(n):
    """Calculate the factorial of a non-negative integer.

    Parameters
    ----------
    n : int
        The non-negative integer to calculate the factorial of.

    Returns
    -------
    int
        The factorial of n. Returns 1 for n = 0.

    Examples
    --------
    >>> factorial(5)
    120
    >>> factorial(0)
    1
    """
    if n == 0:
        return 1
    else:
        return n * factorial(n-1)


def is_prime(number):
    """Check if a number is a prime number.

    A prime number is a natural number greater than 1 that has no
    positive divisors other than 1 and itself.

    Parameters
    ----------
    number : int
        The integer to check.

    Returns
    -------
    bool
        True if the number is prime, False otherwise.

    Examples
    --------
    >>> is_prime(17)
    True
    >>> is_prime(4)
    False
    >>> is_prime(1)
    False
    """
    if number <= 1:
        return False
    for i in range(2, int(number**0.5) + 1):
        if number % i == 0:
            return False
    return True


def reverse_string(s):
    """Reverse a given string.

    Parameters
    ----------
    s : str
        The string to be reversed.

    Returns
    -------
    str
        The reversed string.

    Examples
    --------
    >>> reverse_string("hello")
    'olleh'
    >>> reverse_string("Python")
    'nohtyP'
    """
    return s[::-1]