<script lang="ts">
	import { resolve } from '$app/paths';
	import '../app.css';
	import { signOut } from '@auth/sveltekit/client';
	import * as NavigationMenu from '$lib/components/ui/navigation-menu/index.js';

	let { data, children } = $props();
	// const { session, person } = data;
	// const { user } = session!;
</script>

<NavigationMenu.Root>
	<NavigationMenu.List>
		<NavigationMenu.Item>
			<NavigationMenu.Trigger>Item One</NavigationMenu.Trigger>
			<NavigationMenu.Content>
				<NavigationMenu.Link>Link</NavigationMenu.Link>
			</NavigationMenu.Content>
		</NavigationMenu.Item>
	</NavigationMenu.List>
</NavigationMenu.Root>

{@render children()}
