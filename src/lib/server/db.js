import postgres from 'postgres';
import { env } from '$env/dynamic/private';

const url = `postgres://${env.DB_USER}:${env.DB_PASSWORD}@${env.DB_HOST}:${env.DB_PORT}`;

const sql = postgres(url);
export default sql;