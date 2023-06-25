from os import path

from pydantic import BaseSettings

DOTENV_FILE = ".env" if path.isfile(".env") else None


class Settings(BaseSettings):
    """Application Settings and Environment Variables"""

    # Application Settings
    application_name: str = "collatz-api"
    environment: str = "development"
    log_level: str = "info"
    server_host: str = "0.0.0.0"
    server_port: int = 8000
    server_prefix: str = ""
    openapi_url: str = "/openapi.json"

    # Database Settings
    db_url: str

    class Config:
        env_file = DOTENV_FILE


settings: Settings = Settings()
