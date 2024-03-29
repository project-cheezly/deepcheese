import { getEmailFromLocals } from '$lib/auth';
import { getUserSerialId } from "$lib/server/userList.js";
import sql from "$lib/server/db";

export async function load({ locals }) {
    const serialId = getUserSerialId(await getEmailFromLocals(locals));
    const accounts = await loadAccounts(serialId);
    const assets = await loadCurrentAssetByAccount(serialId);

    return {
        accounts: accounts.map(account => {
            account.assets = assets[account.id] || [];
            return account;
        })
    };
}

export const actions = {
    delete: async ({ locals, request }) => {
        const serialId = getUserSerialId(await getEmailFromLocals(locals));

        const data = await request.formData();
        const accountId = data.get('id');

        await sql`DELETE FROM account WHERE id=${accountId} AND user_id=${serialId}`;
    },

    update: async ({ locals, request }) => {
        const userId = getUserSerialId(await getEmailFromLocals(locals));
        const data = await request.formData();

        const accountId = data.get('id');
        const name = data.get('name');
        const number = data.get('number');

        await sql`
            UPDATE account 
            SET name=${name}, number=${number}
            WHERE id=${accountId} AND user_id=${userId}`;
    },

    create: async ({ locals, request }) => {
        const userId = getUserSerialId(await getEmailFromLocals(locals));
        const data = await request.formData();

        const name = data.get('name');
        const number = data.get('number');

        await sql`
            INSERT INTO account (user_id, name, number)
            VALUES (${userId}, ${name}, ${number})`;
    }
}

async function loadAccounts(serialId) {
    return sql`SELECT id, name, number FROM account WHERE user_id=${serialId}`;
}

async function loadCurrentAssetByAccount(serialId) {
    const response= sql`
        SELECT asset_balance.account_id, category.name as category_name, asset.name, asset_balance.amount
        FROM asset_balance
        INNER JOIN asset ON asset.id = asset_balance.asset_id
        INNER JOIN category ON category.id = asset_balance.category_id
        WHERE asset_balance.user_id=${serialId}`;

    return (await response).reduce((acc, row) => {
        acc[row.account_id] = acc[row.account_id] || [];
        acc[row.account_id].push(row);
        return acc;
    }, {});
}