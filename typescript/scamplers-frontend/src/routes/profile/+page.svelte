<script lang="ts">
	let { data } = $props();
	let apiKey = $state('');

	async function toggleApiKey() {
		if (apiKey) {
			apiKey = '';
			return;
		}

		const { session } = data;
		if (session && session.user.apiKey) {
			apiKey = session.user.apiKey;
		}
	}

	const { session } = data;

	const { name, email } = session!.user;
</script>

<div>
	<ul>
		<li><strong>{name}</strong></li>
		<li>{email}</li>
	</ul>
	<button onclick={toggleApiKey}
		>{#if !apiKey}View API Key{:else}{apiKey} - Hide API Key{/if}</button
	>
</div>
