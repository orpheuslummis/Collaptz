from typing import List

from databases import Database
from sqlalchemy import select, and_

from app.usecases.interfaces.repos.collatz import ICollatzRepo
from app.usecases.schemas.collatz import CollatzBase, CollatzDataInDb

from app.infrastructure.db.models.collatz import COLLATZ_DATA


class CollatzRepo(ICollatzRepo):
    def __init__(self, db: Database):
        self.db = db

    async def create(self, data: CollatzBase) -> CollatzDataInDb:
        """Insert Collatz data into the database."""
        insert_statement = COLLATZ_DATA.insert().values(
            input_value=data.input_value,
            output_sequence=data.output_sequence,
            proof=str(data.proof),
        )

        id = await self.db.execute(insert_statement)

        return await self.retrieve(id=id)

    async def retrieve(self, id: int) -> CollatzDataInDb:
        """Retrieve Collatz data from database by ID."""

        query = select(COLLATZ_DATA).where(COLLATZ_DATA.c.id == id)

        result = await self.db.fetch_one(query)

        return CollatzDataInDb(**result) if result else None

    async def retrieve_by_input_value(self, input_value: int) -> CollatzDataInDb:
        """Retrieve Collatz data from database by one input value."""

        query = select(COLLATZ_DATA).where(COLLATZ_DATA.c.input_value == input_value)

        result = await self.db.fetch_one(query)

        return CollatzDataInDb(**result) if result else None

    async def retrieve_by_range(self, min: int, max: int) -> List[CollatzDataInDb]:
        """Retrieve Collatz data from database by max and min input values."""

        query = select(COLLATZ_DATA).where(
            and_(
                COLLATZ_DATA.c.input_value >= min,
                COLLATZ_DATA.c.input_value <= max,
            )
        )

        results = await self.db.fetch_all(query)

        return [CollatzDataInDb(**result) for result in results]
