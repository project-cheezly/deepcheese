import sql from '$lib/server/db';
import { getUserSerialId } from "$lib/server/userList";
import { getEmail } from "$lib/auth";

export async function load({ locals, url }) {
    const page = parseInt(url.searchParams.get('page')) || 1;
    const user_id = getUserSerialId(getEmail(locals));

    return {
        moneyFlow: await loadMoneyFlow(user_id, page),
        pageCnt: page,
        maxPage: await calculateMaxPage(user_id),
        categories: await loadCategories(user_id),
        accounts: await loadAccounts(user_id)
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