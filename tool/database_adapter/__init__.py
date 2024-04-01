import os
import psycopg


class PostgreSQL:
    def __init__(self):
        db_user = os.environ.get("DB_USER")
        db_password = os.environ.get("DB_PASSWORD")
        db_host = os.environ.get("DB_HOST")
        db_port = os.environ.get("DB_PORT")

        print(db_user, db_password, db_host, db_port)

        self.conn = psycopg.connect(
            user=db_user,
            password=db_password,
            host=db_host,
            port=db_port
        )
