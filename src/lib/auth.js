import { SvelteKitAuth } from "@auth/sveltekit";
import Github from '@auth/core/providers/github';
import { GITHUB_ID, GITHUB_SECRET } from '$env/static/private';

export const { handle, signIn, signOut } = SvelteKitAuth({
    providers: [
        Github({
            clientId: GITHUB_ID,
            clientSecret: GITHUB_SECRET,
        })
    ]
});
