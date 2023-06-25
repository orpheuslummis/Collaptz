from app.infrastructure.db.core import get_or_create_database

from app.usecases.interfaces.repos.collatz import ICollatzRepo
from app.infrastructure.db.repos.collatz import CollatzRepo


async def get_example_repo() -> ICollatzRepo:
    return CollatzRepo(db=await get_or_create_database())
