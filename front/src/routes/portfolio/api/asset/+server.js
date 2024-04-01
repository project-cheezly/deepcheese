import sql from '$lib/server/db.js';

export async function GET({ url }) {
    if (!url.searchParams.has('asset_name')) {
        return {
            status: 400,
            body: { error: 'asset_name is required' }
        };
    }

    const name = url.searchParams.get('asset_name');

    const response = await sql`
        SELECT id, name
        FROM asset
        WHERE UPPER(name) LIKE ${ name.toUpperCase() + '%' }
        LIMIT 10`;

    return new Response(JSON.stringify(response));
}