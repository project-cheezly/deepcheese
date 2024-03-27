import sql from '$lib/server/db';
import { dev } from '$app/environment';

async function getUserList() {
    const users = await sql`SELECT * FROM sheet_user`;
    return users.reduce((acc, user) => {
        acc[user.email] = user.serial_id;
        return acc;
    }, {});
}

const userList = await getUserList();

export function getUserSerialId(email) {
    if (userList[email]) {
        return userList[email];
    } else if (dev) {
        return 1;
    } else {
        return 0;
    }
}
