import { getEmailFromLocals } from '$lib/server/auth';
import { getUserSerialId } from "$lib/server/userList";
import sql from '$lib/server/db';

export async function load({ locals }) {
    const serialId = getUserSerialId(await getEmailFromLocals(locals));
    const categories = await loadCategories(serialId);
    const assets = await loadCurrentAssetByCategory(serialId);
    const balance = await loadBalanceByCategory(serialId);

    return {
        categories: categories.map(category => {
            category.assets = assets[category.id] || [];
            category.balance = balance[category.id] || [];
            return category;
        })
    }
}

export const actions = {
    delete: async ({ locals, request }) => {
        const data = await request.formData();

        const categoryId = data.get('id');
        const serialId = getUserSerialId(await getEmailFromLocals(locals));

        await sql`DELETE FROM category WHERE id=${categoryId} AND user_id=${serialId}`;
    },

    update: async ({ locals, request }) => {
        const data = await request.formData();
        const categoryId = data.get('id');
        const serialId = getUserSerialId(await getEmailFromLocals(locals));
        const name = data.get('name');

        await sql`UPDATE category SET name=${name} WHERE id=${categoryId} AND user_id=${serialId}`;
    },

    create: async({ locals, request }) => {
        const userId = getUserSerialId(await getEmailFromLocals(locals));
        const data = await request.formData();

        const name = data.get('name');
        await sql`INSERT INTO category (name, user_id) VALUES (${name}, ${userId})`;
    }
}

async function loadCategories(serialId) {
    return sql`SELECT id, name FROM category WHERE user_id=${serialId}`;
}

async function loadCurrentAssetByCategory(serialId) {
    const response = sql`
        SELECT asset_balance.category_id, account.name as account_name, asset.name, asset_balance.amount
        FROM asset_balance
        INNER JOIN asset ON asset.id = asset_balance.asset_id
        INNER JOIN account ON account.id = asset_balance.account_id
        WHERE account.user_id=${serialId}`;

    return (await response).reduce((acc, row) => {
        acc[row.category_id] = acc[row.category_id] || [];
        acc[row.category_id].push(row);
        return acc;
    }, {});
}

async function loadBalanceByCategory(serialId) {
    const response = sql`
        SELECT bank_balance.category_id, bank_balance.currency_id, bank_balance.value, account.name as account_name
        FROM bank_balance
        INNER JOIN account ON account.id = bank_balance.account_id
        WHERE account.user_id=${serialId}`;

    return (await response).reduce((acc, row) => {
        acc[row.category_id] = acc[row.category_id] || [];
        acc[row.category_id].push(row);
        return acc;
    }, {});
}