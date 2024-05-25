import { SvelteKitAuth } from "@auth/sveltekit";
import Github from '@auth/core/providers/github';
import { env } from '$env/dynamic/private';
import { dev } from "$app/environment";
import { building } from "$app/environment";

if (!building) {

}

const { handle, signIn, signOut } = SvelteKitAuth({
    providers: [
        Github({
            clientId: env.GITHUB_ID,
            clientSecret: env.GITHUB_SECRET,
        })
    ]
});

export { handle, signIn, signOut };

/**
 *  현재 세션에 접속한 유저의 이메일을 조회합니다.
 * @param locals
 * @returns {Promise<string|null>} 이메일을 반환합니다. 세션이 존재하지 않으면 null을 반환합니다.
 */
export async function getEmailFromLocals(locals) {
    if (!dev) {
        const session = await locals.auth();
        return session?.user.email ?? null;
    } else {
        return null;
    }
}