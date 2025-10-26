import { ScamplersClient } from 'scamplers';
import type { LayoutLoad } from './$types';
import { PUBLIC_API_PATH } from '$env/static/public';
import { PUBLIC_API_ADDRESS_CLIENT } from '$env/static/public';

export const load: LayoutLoad = async (event) => {
	const session = event.data.session;
	const apiClient =
		session === null
			? null
			: new ScamplersClient(PUBLIC_API_ADDRESS_CLIENT + PUBLIC_API_PATH, null, session.user.apiKey);

	let person = null;
	if (apiClient !== null) {
		person = await apiClient?.fetch_person(session!.user.id);
	}

	return {
		session,
		apiClient,
		person
	};
};
