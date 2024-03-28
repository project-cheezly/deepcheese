import sql from '$lib/server/db';
import { getUserSerialId } from "$lib/server/userList";
import { getEmail } from "$lib/auth";

export async function load({ locals, url }) {
    const page = parseInt(url.searchParams.get('page')) || 1;
    const user_id = getUserSerialId(getEmail(await locals.auth()));

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
        let user_id = getUserSerialId(getEmail(await locals.auth()));

        let data = await request.formData();
        let record_date = data.get('record_date');
        let account_id = data.get('account_id');
        let category_id = data.get('category_id');
        let transaction_type = data.get('transaction_type');
        let currency_id = data.get('currency_id');
        let amount = data.get('amount');

        await sql.begin(async (sql) => {
            await sql`
                INSERT INTO money_flow
                (record_date, user_id, account_id, category_id, transaction_type, currency_id, value)
                VALUES (${record_date}, ${user_id}, ${account_id}, ${category_id}, ${transaction_type}, ${currency_id}, ${amount}::NUMERIC(12, 2))`;

            const adjustedValue = transaction_type === 'INFLOW' ? amount : -amount;
            await sql`
                INSERT INTO money_balance
                (user_id, account_id, category_id, currency_id, value)
                VALUES (
                    ${user_id},
                    ${account_id},
                    ${category_id},
                    ${currency_id},
                    ${adjustedValue}
                ) ON CONFLICT (user_id, currency_id, account_id, category_id)
                DO UPDATE SET value = money_balance.value + ${adjustedValue}::NUMERIC(12, 2)`;
        })
    }
}

async function loadMoneyFlow(user_id, page) {
    return sql`
        SELECT *
        FROM money_flow
        WHERE user_id = ${user_id}
        ORDER BY record_date DESC
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
    const response = await sql`SELECT id, name FROM account WHERE user_id=${user_id}`;
    return response.reduce((acc, {id, name}) => {
        acc[id] = name;
        return acc;
    }, {});
}


async function calculateMaxPage(user_id) {
    const response = await sql`SELECT COUNT(*) FROM money_flow WHERE user_id=${user_id}`;
    return Math.ceil(response[0].count / 10);
}