import { error, redirect } from '@sveltejs/kit';
import { sequence } from '@sveltejs/kit/hooks';
import { dev } from '$app/environment';
import { handle as authenticationHandle, getEmail } from '$lib/auth';
import { getUserSerialId } from '$lib/server/userList';

async function authorizationHandle({ event, resolve }) {
    if (dev) {
        console.log("dev mode: skipping authorization");
        return resolve(event);
    }

    const session = await event.locals.auth();
    if (!session) throw redirect(303, '/auth/signin');
    if (getUserSerialId(session.user.email) === 0) error(403);

    return resolve(event);
}

export const handle = sequence(authenticationHandle, authorizationHandle);
