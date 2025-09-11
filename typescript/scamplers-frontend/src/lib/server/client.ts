import { PUBLIC_API_PATH } from '$env/static/public';
import { API_ADDRESS_SERVER } from '$env/static/private';
import { ScamplersClient } from 'scamplers';
import { FRONTEND_TOKEN } from './secrets';

export const serverApiClient = new ScamplersClient(
	API_ADDRESS_SERVER + PUBLIC_API_PATH,
	FRONTEND_TOKEN ?? '',
	null
);
