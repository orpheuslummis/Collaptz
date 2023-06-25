from typing import List
from abc import ABC, abstractmethod

from app.usecases.schemas.collatz import CollatzBase, CollatzDataInDb


class ICollatzRepo(ABC):
    @abstractmethod
    async def create(self, data: CollatzBase) -> CollatzDataInDb:
        """Insert Collatz data into the database."""

    @abstractmethod
    async def retrieve(self, id: int) -> CollatzDataInDb:
        """Retrieve Collatz data from database by ID."""

    @abstractmethod
    async def retrieve_by_input_value(self, input_value: int) -> CollatzDataInDb:
        """Retrieve Collatz data from database by one input value."""

    @abstractmethod
    async def retrieve_by_range(self, min: int, max: int) -> List[CollatzDataInDb]:
        """Retrieve Collatz data from database by max and min input values."""
