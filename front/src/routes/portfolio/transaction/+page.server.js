import sql from '$lib/server/db';
import { getUserSerialId } from "$lib/server/userList";
import { getEmailFromLocals } from "$lib/auth";

export async function load({ locals, url }){
    const page = parseInt(url.searchParams.get('page')) || 1;
    const serialId = getUserSerialId(await getEmailFromLocals(locals))

    const transactionRecord = await loadTransactionRecord(serialId, page);
    const categories = await loadCategories(serialId);
    const accounts = await loadAccounts(serialId);
    const maxPage = await calculateMaxPage(serialId);

    return {
        ledger: transactionRecord,
        pageCnt: page,
        categories: categories,
        accounts: accounts,
        maxPage: maxPage
    }
}

export const actions = {
    create: async ({ request }) => {
        let data = await request.formData();

        const record_date = data.get('record_date');
        const category_id = data.get('category_id');
        const account_id = data.get('account_id');
        const asset_id = data.get('asset_id');
        const trade_type = data.get('trade_type');
        const amount = data.get('amount');
        const value = data.get('value');
        const fee = data.get('fee');

        // null check
        if (!record_date || !category_id || !account_id || !asset_id || !trade_type || !amount || !value || !fee) {
            return {
                status: 400,
                body: { message: 'Missing required fields' }
            };
        }

        await sql`CALL insert_transaction(
            ${record_date},
            ${category_id},
            ${account_id},
            ${asset_id},
            ${trade_type},
            ${amount},
            ${value},
            ${fee}
        );`;
    }
}

async function loadTransactionRecord(serialId, page) {
    return sql`
        SELECT transaction_record.*, market.currency_id, asset.name AS asset_name
        FROM transaction_record
        INNER JOIN asset ON transaction_record.asset_id = asset.id
        INNER JOIN market ON asset.market_id = market.id
        INNER JOIN category ON transaction_record.category_id = category.id
        WHERE category.user_id = ${serialId}
        ORDER BY record_date DESC
        LIMIT 10 OFFSET ${(page - 1) * 10}`;
}

async function loadCategories(serialId) {
    const response= await sql`SELECT id, name FROM category WHERE user_id=${serialId}`;
    return response.reduce((acc, {id, name}) => {
        acc[id] = name;
        return acc;
    }, {});
}

async function loadAccounts(serialId) {
    const response = await sql`SELECT id, name FROM account WHERE user_id=${serialId}`;
    return response.reduce((acc, {id, name}) => {
        acc[id] = name;
        return acc;
    }, {});
}

async function calculateMaxPage(serialId) {
    const response = await sql`
        SELECT COUNT(*)
        FROM transaction_record
        INNER JOIN category ON transaction_record.category_id = category.id
        WHERE category.user_id=${serialId}`;
    return Math.ceil(response[0].count);
}