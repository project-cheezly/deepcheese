import postgres from 'postgres';
import { env } from '$env/dynamic/private';
import { building } from '$app/environment';

const url = `postgres://${env.DB_USER}:${env.DB_PASSWORD}@${env.DB_HOST}:${env.DB_PORT}`;

let sql;
if (!building) {
    sql = postgres(url);
}

export default sql;
