from app.usecases.interfaces.services.example_manager import IExampleManager
from app.usecases.services.example_manager import ExampleManager


async def get_example_service() -> IExampleManager:
    """Instantiates and returns the Signature Manger Service."""

    return ExampleManager()
