from datetime import datetime
from typing import List, Dict
from pydantic import BaseModel, Field


class CollatzBase(BaseModel):
    """The core data object for storing collatz-related data."""

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
    image_id: List[int] = Field(
        ...,
        description="A binary representing the hash of the program that ran the collatz code.",
        example=[837, 12, 37827],
    )

    @property
    def input_value(self):
        return self.output_sequence[0]


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
