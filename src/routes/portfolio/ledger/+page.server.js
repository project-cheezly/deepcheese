import sql from '$lib/server/db';
import { getUserSerialId } from "$lib/server/userList";
import {getEmail} from "$lib/auth";

export async function load({ locals, url }){
    const page = parseInt(url.searchParams.get('page')) || 1;
    const email = getEmail(await locals.auth());

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

export const actions = {
    create: async ({ locals, request }) => {
        let data = await request.formData();

        const user_id = getUserSerialId(getEmail(await locals.auth()));

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

        await sql.begin(async (sql) => {
            await sql`
                INSERT INTO ledger
                (user_id, record_date, category_id, account_id, asset_id, type, amount, value, fee)
                VALUES (${user_id}, ${record_date}, ${category_id}, ${account_id}, ${asset_id}, ${trade_type},
                ${amount}, ${value}, ${fee})`;

            if (trade_type === 'BUY' || trade_type === 'SELL') {
                const adjustedAmount = trade_type === 'BUY' ? amount : -amount;
                await sql`
                    INSERT INTO asset_balance (user_id, category_id, asset_id, account_id, amount)
                    VALUES (${user_id}, ${category_id}, ${asset_id}, ${account_id}, ${adjustedAmount})
                    ON CONFLICT (user_id, category_id, asset_id, account_id)
                    DO UPDATE SET amount = asset_balance.amount + ${adjustedAmount}`;
            }

            const adjustedValue = trade_type === 'BUY' ? -value : value;
            await sql`
                INSERT INTO money_balance (user_id, currency_id, account_id, category_id, value)
                VALUES (
                    ${user_id},
                    (SELECT currency_id FROM asset WHERE id=${asset_id}),
                    ${account_id},
                    ${category_id},
                    ${adjustedValue}::NUMERIC(12, 2) * ${amount}::INTEGER - ${fee}::NUMERIC(12, 2)
                ) ON CONFLICT (user_id, currency_id, account_id, category_id)
                DO UPDATE SET value = money_balance.value 
                + ${adjustedValue}::NUMERIC(12, 2) 
                * ${amount}::INTEGER 
                - ${fee}::NUMERIC(12, 2)`;
        });
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