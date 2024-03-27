import { getEmail } from '$lib/auth';
import { getUserSerialId } from "$lib/server/userList.js";
import sql from "$lib/server/db";

export async function load({ locals }) {
    const email = getEmail(locals);
    const accounts = await loadAccounts(email);
    const assets = await loadCurrentAssetByAccount(email);

    return {
        accounts: accounts.map(account => {
            account.assets = assets[account.id] || [];
            return account;
        })
    };
}

export const actions = {
    delete: async ({ locals, request }) => {
        const email = getEmail(locals);
        const data = await request.formData();

        const accountId = data.get('id');
        const serialId = getUserSerialId(email);

        await sql`DELETE FROM account WHERE id=${accountId} AND user_id=${serialId}`;
    }
}

async function loadAccounts(email) {
    const serialId = getUserSerialId(email);
    return sql`SELECT id, name, number FROM account WHERE user_id=${serialId}`;
}

async function loadCurrentAssetByAccount(email) {
    const serialId = getUserSerialId(email);
    const response= sql`
        SELECT asset_balance.account_id, asset.name, asset_balance.amount
        FROM asset_balance
        INNER JOIN asset
        ON asset.id = asset_balance.asset_id
        WHERE asset_balance.user_id=${serialId}`;

    return (await response).reduce((acc, row) => {
        acc[row.account_id] = acc[row.account_id] || [];
        acc[row.account_id].push(row);
        return acc;
    }, {});
}