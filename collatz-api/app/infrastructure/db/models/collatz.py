import sqlalchemy as sa

from app.infrastructure.db.metadata import METADATA
from sqlalchemy.dialects.postgresql import ARRAY

COLLATZ_DATA = sa.Table(
    "collatz_data",
    METADATA,
    sa.Column("id", sa.BigInteger, primary_key=True, autoincrement=True),
    sa.Column(
        "input_value",
        sa.BigInteger,
        index=True,
    ),
    sa.Column(
        "output_sequence",
        ARRAY(sa.BigInteger),
    ),
    sa.Column("proof", sa.String, nullable=False),
    sa.Column("created_at", sa.DateTime, nullable=False, server_default=sa.func.now()),
    sa.Column(
        "updated_at",
        sa.DateTime,
        nullable=False,
        server_default=sa.func.now(),
        onupdate=sa.func.now(),
    ),
)
