from abc import ABC, abstractmethod

from app.usecases.schemas.example import ExampleEntity


class IExampleManager(ABC):
    @abstractmethod
    async def example(self) -> ExampleEntity:
        """Example function."""
