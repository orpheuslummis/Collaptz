from datetime import datetime
from typing import List, Dict
from pydantic import BaseModel, Field


class CollatzBase(BaseModel):
    """The core data object for storing collatz-related data."""

    input_value: int = Field(
        ..., description="An input for a given collatz conjecture iteration.", example=1
    )
    output_sequence: List[int] = Field(
        ...,
        description="The sequence of numbers resulting from following the two rules of the collatz protocol.",
        example=[4, 2, 1],
    )
    proof: str = Field(
        ...,
        description="Base64-encoded bytes, representing the proof of computation.",
        example="SGksIG15IG5hbWUgaXMgcG9vcC4=",
    )


class CollatzPostRequestBody(CollatzBase):
    """The data object that the post request endpiont expects."""


class CollatzDataInDb(CollatzBase):
    id: int
    created_at: datetime
    updated_at: datetime


class CollatzSequenceResponse(CollatzDataInDb):
    """Same as database object."""


class CollatzSequencesByRangeResponse(BaseModel):
    """Returns the numbers and corresponding number of steps requested by the user."""

    data: Dict[int, List[int]]
