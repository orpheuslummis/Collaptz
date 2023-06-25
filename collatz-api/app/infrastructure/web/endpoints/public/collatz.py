import os

from fastapi import APIRouter, Body, Depends, Path, HTTPException, Query


from app.dependencies import get_example_service, get_example_repo

from app.usecases.schemas.collatz import (
    CollatzPostRequestBody,
    CollatzSequenceRepsonse,
    CollatzSequencesByRangeResponse,
)

from app.usecases.interfaces.repos.collatz import ICollatzRepo
from app.usecases.interfaces.services.example_manager import IExampleManager

collatz_router = APIRouter(tags=["Collatz Data"])


@collatz_router.post(
    "/actions/create",
    status_code=201,
    response_model=None,
)
async def add_sequence(
    body: CollatzPostRequestBody = Body(...),
    collatz_repo: ICollatzRepo = Depends(get_example_repo),
    example_service: IExampleManager = Depends(get_example_service),
) -> None:
    """Issues new challenge."""

    fname = f'{body.input_value}_receipt.dat'
    with open(fname) as f:
        f.write(body.proof)

    # 1. Check that the proof is valid
    out = os.system(f"cargo run -- {body.image_id} {fname}")
    if out == 0:
        raise HTTPException(status_code=400, detail="Invalid proof.")

    # 2. If the proof is valid, insert the data into the database
    stored_data = await collatz_repo.create(data=body)


@collatz_router.get(
    "/{input_value}",
    status_code=200,
    response_model=CollatzSequenceRepsonse,
)
async def get_sequence(
    input_value: int = Path(...),
    collatz_repo: ICollatzRepo = Depends(get_example_repo),
) -> CollatzSequenceRepsonse:
    """Retrieve stuff from the database."""

    retrieved_data = await collatz_repo.retrieve_by_input_value(input_value=input_value)

    if not retrieved_data:
        raise HTTPException(status_code=404, detail="Resource not found")


    return CollatzSequenceRepsonse(**retrieved_data.dict())


@collatz_router.get(
    "/",
    status_code=200,
    response_model=CollatzSequencesByRangeResponse,
)
async def get_sequences_by_range(
    min: int = Query(...),
    max: int = Query(...),
    collatz_repo: ICollatzRepo = Depends(get_example_repo),
) -> CollatzSequencesByRangeResponse:
    """Retrieve stuff from the database."""

    if not min < max:
        raise HTTPException(status_code=400, detail="Min must be less than max.")

    retrieved_data = await collatz_repo.retrieve_by_range(min=min, max=max)

    return_dict = {}
    for collatz_bundle in retrieved_data:
        return_dict[collatz_bundle.input_value] = collatz_bundle.output_sequence

    return CollatzSequencesByRangeResponse(data=return_dict)
