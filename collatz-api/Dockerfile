FROM python:3.8

WORKDIR /app

COPY ./requirements.txt /app/requirements.txt

RUN pip install alembic
RUN pip install --no-cache-dir --upgrade -r /app/requirements.txt

COPY . /app

# Database Environment Variables
# FIXME do not hardcode
ENV DB_URL=postgres://postgres:postgres@localhost:5432/collatz-dev

# Server Environment Variables
ENV OPENAPI_URL="/openapi.json"
ENV SERVER_PORT=8000

ENV ENVIRONMENT=development

EXPOSE $SERVER_PORT

CMD ["python", "-m", "app"]
