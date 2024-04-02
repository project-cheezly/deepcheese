FROM python:3.12.2-slim-bookworm AS cheeseboard-cron

WORKDIR /app

RUN pip install poetry==1.8.2

RUN POETRY_NO_INTERACTION=1 \
    POETRY_VIRTUALENVS_IN_PROJECT=1 \
    POETRY_VIRTUALENVS_CREATE=1 \
    POETRY_CACHE_DIR=/tmp/poetry_cache

COPY ./tool .
COPY .env .

RUN export $(cat .env | xargs)
RUN poetry install --no-dev
RUN ln -snf /usr/share/zoneinfo/Asia/Seoul /etc/localtime

CMD ["poetry", "run", "python", "-m", "admin", "--asset_price", "--currency_value", "--update_category_history"]