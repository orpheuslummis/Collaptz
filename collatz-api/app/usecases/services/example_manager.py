from app.usecases.interfaces.services.example_manager import IExampleManager
from app.usecases.schemas.example import ExampleEntity
from app.settings import settings


class ExampleManager(IExampleManager):
    def __init__(self):
        pass

    async def example(self) -> ExampleEntity:
        """Does something."""

        return ExampleEntity(height=199, weight=120)
