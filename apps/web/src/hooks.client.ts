import { initSentry } from '$lib/analytics/sentry';
import type { HandleClientError } from '@sveltejs/kit';

initSentry();

export const handleError: HandleClientError = ({ error }) => {
	console.error(error);
};
