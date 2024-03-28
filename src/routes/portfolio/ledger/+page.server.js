import sql from '$lib/server/db';
import { getUserSerialId } from "$lib/server/userList";
import {getEmail} from "$lib/auth";

export async function load({ locals, url }){
    const page = parseInt(url.searchParams.get('page')) || 1;
    const email = getEmail(locals);

    const ledger = await loadLedger(email, page);
    const categories = await loadCategories(email);
    const accounts = await loadAccounts(email);
    const assets = await loadAssets();
    const maxPage = await calculateMaxPage(email);

    return {
        ledger: ledger,
        pageCnt: page,
        categories: categories,
        assets: assets,
        accounts: accounts,
        maxPage: maxPage
    }
}

async function loadLedger(email, page) {
    const serialId = getUserSerialId(email);
    return sql`
        SELECT * 
        FROM ledger
        WHERE user_id=${serialId}
        ORDER BY record_date DESC
        LIMIT 10 OFFSET ${(page - 1) * 10}`;
}

async function loadCategories(email) {
    const serialId = getUserSerialId(email);
    const response= await sql`SELECT id, name FROM category WHERE user_id=${serialId}`;
    return response.reduce((acc, {id, name}) => {
        acc[id] = name;
        return acc;
    }, {});
}

async function loadAccounts(email) {
    const serialId = getUserSerialId(email);
    const response = await sql`SELECT id, name FROM account WHERE user_id=${serialId}`;
    return response.reduce((acc, {id, name}) => {
        acc[id] = name;
        return acc;
    }, {});
}

async function loadAssets() {
    const response = await sql`SELECT id, name FROM asset`;
    return response.reduce((acc, {id, name}) => {
        acc[id] = name;
        return acc;
    }, {});
}

async function calculateMaxPage(email) {
    const serialId = getUserSerialId(email);
    const response = await sql`SELECT COUNT(*) FROM ledger WHERE user_id=${serialId}`;
    return Math.ceil(response[0].count);
}