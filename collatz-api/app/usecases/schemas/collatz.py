import json
from datetime import datetime
from typing import List, Dict
from pydantic import BaseModel, Field, validator


class CollatzBase(BaseModel):
    """The core data object for storing collatz-related data."""

    output_sequence: List[int] = Field(
        ...,
        description="The sequence of numbers resulting from following the two rules of the collatz protocol.",
        example=[4, 2, 1],
    )
    proof: List[int] = Field(
        ...,
        description="Bytearray, representing the proof of computation.",
        example=[1, 2, 3, 4, 5],
    )

    @property
    def input_value(self):
        return self.output_sequence[0]

    @validator("proof", pre=True)
    def validate_proof(cls, proof):
        if isinstance(proof, str):
            proof = json.loads(proof)
        return proof


class CollatzPostRequestBody(CollatzBase):
    """The data object that the post request endpiont expects."""

    image_id: List[int] = Field(
        description="A binary representing the hash of the program that ran the collatz code.",
        example=[837, 12, 37827],
    )


class CollatzDataInDb(CollatzBase):
    id: int
    created_at: datetime
    updated_at: datetime


class CollatzSequenceResponse(CollatzDataInDb):
    """Same as database object."""


class CollatzSequencesByRangeResponse(BaseModel):
    """Returns the numbers and corresponding number of steps requested by the user."""

    data: Dict[int, List[int]]
