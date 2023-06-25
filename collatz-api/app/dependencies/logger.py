import logging

from app.settings import settings

logging.basicConfig(
    format="%(asctime)s|%(name)s|%(levelname)-5.5s|%(message)s",
    datefmt="%Y-%m-%dT%H:%M:%S",
)

logger = logging.getLogger(settings.application_name)
logger.setLevel(logging.INFO)

__all__ = ["logger"]
