# Lab 02: Prompt Engineering Solutions

## Problem 1: Debugging

**My Prompt:**
```
You are a senior Python developer. I have a function that's supposed to sum all even numbers in a list, but it's giving wrong results. It seems to be summing the odd numbers instead. Can you identify the bug and provide the corrected version?

Here's the code:
```python
def sum_of_evens(numbers):
    """Calculate the sum of all even numbers in a list.

    Parameters
    ----------
    numbers : list of int
        A list of integers.

    Returns
    -------
    int
        The sum of all even numbers in the list.
    """
    total = 0
    for num in numbers:
        if num % 2 == 1:  # This line has a bug!
            total += num
    return total
```
```

**AI's Corrected Code:**
```python
def sum_of_evens(numbers):
    """Calculate the sum of all even numbers in a list.

    Parameters
    ----------
    numbers : list of int
        A list of integers.

    Returns
    -------
    int
        The sum of all even numbers in the list.
    """
    total = 0
    for num in numbers:
        if num % 2 == 0:  # Fix: Changed from 1 to 0 to correctly check for even numbers.
            total += num
    return total
```

**What I Learned:**
The bug was a simple logical error in the conditional statement. The code was checking `num % 2 == 1`, which is true for odd numbers. The AI correctly identified that it should be `num % 2 == 0` to check for even numbers. This highlights the importance of carefully checking the logic of conditional statements.

---

## Problem 2: Refactoring

**My Prompt:**
```
You are an expert Python developer who specializes in writing clean, Pythonic code. I have a function that works, but it's written in a clunky, non-idiomatic way. Can you please refactor it to be more concise and readable? Please use a list comprehension.

Here's the code:
```python
def get_names_of_adults(users):
    """Given a list of user dictionaries, returns a list of names of users
    who are 18 or older.

    Parameters
    ----------
    users : list of dict
        List of user dictionaries with 'name' and 'age' keys.

    Returns
    -------
    list of str
        Names of users who are 18 or older.
    """
    results = []
    for i in range(len(users)):
        if users[i]['age'] >= 18:
            results.append(users[i]['name'])
    return results
```
```

**AI's Corrected Code:**
```python
def get_names_of_adults(users):
    """Given a list of user dictionaries, returns a list of names of users
    who are 18 or older.

    Parameters
    ----------
    users : list of dict
        List of user dictionaries with 'name' and 'age' keys.

    Returns
    -------
    list of str
        Names of users who are 18 or older.
    """
    return [user['name'] for user in users if user['age'] >= 18]
```

**What I Learned:**
List comprehensions can significantly simplify code that iterates over a list to build a new list. The refactored code is much more readable and Pythonic. It expresses the logic in a single, declarative line instead of a multi-line loop.

---

## Problem 3: Documenting

**My Prompt:**
```
You are a helpful programming assistant who writes excellent documentation. I have a Python function that is missing a docstring. Can you please write a professional, NumPy-style docstring for it? Make sure to document the parameters, what it returns, and any exceptions it might raise.

Here's the code:
```python
def calculate_area(length, width):
    if length <= 0 or width <= 0:
        raise ValueError("Length and width must be positive numbers.")
    return length * width
```
```

**AI's Corrected Code:**
```python
def calculate_area(length, width):
    """Calculate the area of a rectangle.

    Parameters
    ----------
    length : float or int
        The length of the rectangle. Must be a positive number.
    width : float or int
        The width of the rectangle. Must be a positive number.

    Returns
    -------
    float or int
        The calculated area of the rectangle.

    Raises
    ------
    ValueError
        If either length or width is not a positive number.
    """
    if length <= 0 or width <= 0:
        raise ValueError("Length and width must be positive numbers.")
    return length * width
```

**What I Learned:**
A good docstring is crucial for making code understandable. The NumPy style is very structured and includes sections for `Parameters`, `Returns`, and `Raises`, which makes it very clear how to use the function and what to expect. It's important to document not just the happy path, but also the error conditions.
