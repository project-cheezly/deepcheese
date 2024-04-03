import sql from '$lib/server/db.js';

export async function load({ depends }) {
    depends("data:categoryValue");
    const response = await sql`SELECT updated_timestamp FROM update_log WHERE target = 'cheeseboard-cron';`;

    return {
        updatedAt: response[0].updated_timestamp
    }
}