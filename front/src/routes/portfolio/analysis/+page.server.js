import sql from '$lib/server/db';
import { getUserSerialId } from "$lib/server/userList";
import { getEmailFromLocals } from "$lib/auth";

export async function load({ locals, depends }) {
    depends("data:categoryValue");

    const serialId = getUserSerialId(await getEmailFromLocals(locals));
    const categoryValueHistory = await loadCategoryValueHistory(serialId);
    const realtimeCategoryValueHistory = await loadRealtimeCategoryValueHistory(serialId);

    return {
        categoryValueHistory: categoryValueHistory,
        realtimeCategoryValueHistory: realtimeCategoryValueHistory
    }
}

async function loadCategoryValueHistory(serialId) {
    return sql`
        SELECT
            category_history.tr_date as timestamp,
            category.name as category_name,
            category_history.value
        FROM category
        INNER JOIN category_history
            ON category.id = category_history.category_id
        WHERE category.user_id = ${serialId}
        ORDER BY category_history.tr_date DESC`;
}

async function loadRealtimeCategoryValueHistory(serialId) {
    return sql`
        SELECT
            realtime_category_history.tr_timestamp as timestamp,
            category.name as category_name,
            realtime_category_history.value
        FROM category
        INNER JOIN realtime_category_history
            ON category.id = realtime_category_history.category_id
        WHERE category.user_id = ${serialId}
        ORDER BY realtime_category_history.tr_timestamp DESC
        LIMIT 1440`;
}