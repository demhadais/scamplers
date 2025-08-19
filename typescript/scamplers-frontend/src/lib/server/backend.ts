import { env } from '$env/dynamic/private';
import { ScamplersClient } from 'scamplers';
import { FRONTEND_TOKEN } from './secrets';

const BACKEND_HOST = env.SCAMPLERS_BACKEND_HOST ?? env.BACKEND_HOST;
const BACKEND_PORT = env.SCAMPLERS_BACKEND_PORT ?? env.BACKEND_PORT;

export const BACKEND_URL = `http://${BACKEND_HOST}:${BACKEND_PORT}`;

export const scamplersClient = new ScamplersClient(BACKEND_URL, FRONTEND_TOKEN ?? '', null, false);
