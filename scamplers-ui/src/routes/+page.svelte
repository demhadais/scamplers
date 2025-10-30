<script lang="ts">
    import { authClient } from "$lib/auth-client";
    const session = authClient.useSession();
</script>

<!-- If the user isn't signed in, they will just be redirected to the '/auth/sign-in' -->
{#if $session.data}
    <div>
        <button
            onclick={async () => {
                await authClient.signOut();
            }}
        >
            Sign Out
        </button>
    </div>
{/if}
