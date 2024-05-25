import sql from '$lib/server/db';
import { getUserSerialId } from "$lib/server/userList";
import { getEmailFromLocals } from "$lib/server/auth";

export async function load({ locals, url }) {
    const page = parseInt(url.searchParams.get('page')) || 1;
    const user_id = getUserSerialId(await getEmailFromLocals(locals));

    return {
        moneyFlow: await loadMoneyFlow(user_id, page),
        pageCnt: page,
        maxPage: await calculateMaxPage(user_id),
        categories: await loadCategories(user_id),
        accounts: await loadAccounts(user_id)
    }
}

export const actions = {
    create: async ({ locals, request }) => {
        let user_id = getUserSerialId(await getEmailFromLocals(locals));

        let data = await request.formData();
        let record_date = data.get('record_date');
        let account_id = data.get('account_id');
        let category_id = data.get('category_id');
        let transaction_type = data.get('transaction_type');
        let currency_id = data.get('currency_id');
        let amount = data.get('value');

        await sql`CALL insert_bank(
            ${record_date}::DATE,
            ${category_id}::INTEGER,
            ${account_id}::INTEGER,
            ${currency_id}::INTEGER,
            ${transaction_type}::flow_type,
            ${amount}::NUMERIC(12, 2)
        )`;
    }
}

async function loadMoneyFlow(user_id, page) {
    return sql`
        SELECT account_id, category.id as category_id, currency_id, flow, bank_record.id, record_date, value
        FROM bank_record
        INNER JOIN category ON bank_record.category_id = category.id
        WHERE category.user_id = ${user_id}
        AND bank_record.origin = true
        ORDER BY bank_record.record_date DESC
        LIMIT 10 OFFSET ${(page - 1) * 10}`;
}

async function loadCategories(user_id) {
    const response= await sql`SELECT id, name FROM category WHERE user_id=${user_id}`;
    return response.reduce((acc, {id, name}) => {
        acc[id] = name;
        return acc;
    }, {});
}

async function loadAccounts(user_id) {
    const response = await sql`
        SELECT id, name FROM account WHERE user_id=${user_id}`;
    return response.reduce((acc, {id, name}) => {
        acc[id] = name;
        return acc;
    }, {});
}


async function calculateMaxPage(user_id) {
    const response = await sql`
        SELECT COUNT(*) 
        FROM bank_record 
        INNER JOIN category ON bank_record.category_id = category.id
        WHERE category.user_id = ${user_id}`;
    return Math.ceil(response[0].count / 10);
}