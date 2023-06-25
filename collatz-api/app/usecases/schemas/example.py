from pydantic import BaseModel


class ExampleEntity(BaseModel):
    height: int
    weight: int
