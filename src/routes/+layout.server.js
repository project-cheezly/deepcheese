import { getEmail } from '$lib/auth';

export async function load(event) {
    return {
        session: getEmail(event.locals)
    }
}