class Book:
    """A simple Book class representing a published book.

    Parameters
    ----------
    title : str
        The book title.
    author : str
        The book author.
    year : int
        The publication year.
    """

    def __init__(self, title: str, author: str, year: int):
        self.title = title
        self.author = author
        self.year = year

    def __str__(self) -> str:
        return f'"{self.title}" by {self.author} ({self.year})'

    def get_age(self) -> int:
        """Return the age of the book assuming current year is 2025."""
        current_year = 2025
        return current_year - self.year


class EBook(Book):
    """An EBook extends Book with a file size in megabytes."""

    def __init__(self, title: str, author: str, year: int, file_size: int):
        super().__init__(title, author, year)
        self.file_size = file_size

    def __str__(self) -> str:
        parent = super().__str__()
        return f"{parent} ({self.file_size} MB)"


if __name__ == '__main__':
    # Quick manual test
    b = Book("The Hobbit", "J.R.R. Tolkien", 1937)
    print(b)
    print("Age:", b.get_age())

    eb = EBook("Dune", "Frank Herbert", 1965, 5)
    print(eb)
    print("EBook age:", eb.get_age())
