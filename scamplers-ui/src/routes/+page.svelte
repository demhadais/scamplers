<script lang="ts">
    import { authClient } from "$lib/auth-client";
    const session = authClient.useSession();
</script>

{#if $session.data}
    {JSON.stringify($session.data.user)}
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
