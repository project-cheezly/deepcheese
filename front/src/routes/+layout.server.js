import sql from '$lib/server/db.js';

export async function load() {
    const response = await sql`SELECT updated_timestamp FROM update_log WHERE target = 'cheeseboard-cron';`;

    return {
        updatedAt: response[0].updated_timestamp
    }
}