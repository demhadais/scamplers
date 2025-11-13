<script lang="ts">
  import { authClient } from "$lib/auth-client";
</script>

<div>
  <button
    onclick={async () => {
      await authClient.signIn.social({
        provider: "microsoft",
      });
    }}
  >
    Sign in with Microsoft
  </button>
</div>
