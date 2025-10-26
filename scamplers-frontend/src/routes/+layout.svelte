<script lang="ts">
    import { signOut } from "@auth/sveltekit/client";

    let { data, children } = $props();
    const { session } = data;
</script>

<svelte:head></svelte:head>

<div>
    {#if session}
        <button
            onclick={() => {
                signOut();
            }}>Sign Out</button
        >
        {session.user?.name}
    {/if}
</div>

{@render children?.()}
