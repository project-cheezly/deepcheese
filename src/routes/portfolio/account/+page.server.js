import { getEmail } from '$lib/auth';
import {getUserSerialId} from "$lib/server/userList.js";
import sql from "$lib/server/db";

export async function load({ locals, data }) {
    const email = getEmail(locals);
    const accounts = await loadAccounts(email);
    const assets = await loadCurrentAssetBalance(email);

    return {
        accounts: accounts.map(account => {
            account.assets = assets[account.id] || [];
            return account;
        })
    };
}

async function loadAccounts(email) {
    const serialId = getUserSerialId(email);
    return sql`SELECT id, name, number FROM account WHERE user_id=${serialId}`;
}

async function loadCurrentAssetBalance(email) {
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