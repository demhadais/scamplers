<script lang="ts">
    import { authClient } from "$lib/auth-client";
    const session = authClient.useSession();
</script>

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
