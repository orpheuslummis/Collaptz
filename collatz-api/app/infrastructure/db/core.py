from databases import Database

from app.dependencies import logger
from app.settings import settings

DATABASE = None


async def get_or_create_database():
    global DATABASE
    if DATABASE is not None:
        return DATABASE
    DATABASE = Database(settings.db_url, min_size=5)
    await DATABASE.connect()
    logger.info("Database connection established.")
    return DATABASE
