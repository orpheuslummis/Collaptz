import click
import uvicorn
from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware

from app.dependencies import get_client_session, get_event_loop
from app.infrastructure.db.core import get_or_create_database
from app.infrastructure.web.endpoints.metrics import health
from app.infrastructure.web.endpoints.public import collatz
from app.settings import settings


def setup_app():
    app = FastAPI(
        title="Collatz API",
        description="The following are endpoints for the Collatz frontend to utilize.",
        openapi_url=settings.openapi_url,
    )
    app.include_router(health.health_router, prefix="/metrics/health")
    app.include_router(collatz.collatz_router, prefix="/public/data")

    # CORS (Cross-Origin Resource Sharing)
    origins = ["*"]
    app.add_middleware(
        CORSMiddleware,
        allow_origins=origins,
        allow_credentials=True,
        allow_methods=["*"],
        allow_headers=["*"],
    )

    return app


fastapi_app = setup_app()


@fastapi_app.on_event("startup")
async def startup_event():
    await get_event_loop()
    await get_client_session()
    await get_or_create_database()


@fastapi_app.on_event("shutdown")
async def shutdown_event():
    # Close client session
    client_session = await get_client_session()
    await client_session.close()
    # Close database connection once db exists
    DATABASE = await get_or_create_database()
    if DATABASE.is_connected:
        await DATABASE.disconnect()


@click.command()
@click.option("--reload", is_flag=True)
def main(reload=False):
    kwargs = {"reload": reload}
    uvicorn.run(
        "app.infrastructure.web.setup:fastapi_app",
        loop="uvloop",
        host=settings.server_host,
        port=settings.server_port,
        **kwargs,
    )
