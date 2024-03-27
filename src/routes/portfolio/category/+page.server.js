import { getEmail } from '$lib/auth';
import { getUserSerialId } from "$lib/server/userList";
import sql from '$lib/server/db';

export async function load({ locals }) {
    const email = getEmail(locals);
    const categories = await loadCategories(email);
    const assets = await loadCurrentAssetByCategory(email);

    return {
        categories: categories.map(category => {
            category.assets = assets[category.id] || [];
            return category;
        })
    }
}

export const actions = {
    delete: async ({ locals, request }) => {
        const email = getEmail(locals);
        const data = await request.formData();

        const categoryId = data.get('id');
        const serialId = getUserSerialId(email);

        await sql`DELETE FROM category WHERE id=${categoryId} AND user_id=${serialId}`;
    }
}

async function loadCategories(email) {
    const serialId = getUserSerialId(email);
    return sql`SELECT id, name FROM category WHERE user_id=${serialId}`;
}

async function loadCurrentAssetByCategory(email) {
    const serialId = getUserSerialId(email);
    const response = sql`
        SELECT asset_balance.category_id, account.name as account_name, asset.name, asset_balance.amount
        FROM asset_balance
        INNER JOIN asset ON asset.id = asset_balance.asset_id
        INNER JOIN account ON account.id = asset_balance.account_id
        WHERE asset_balance.user_id=${serialId}`;

    return (await response).reduce((acc, row) => {
        acc[row.category_id] = acc[row.category_id] || [];
        acc[row.category_id].push(row);
        return acc;
    }, {});
}