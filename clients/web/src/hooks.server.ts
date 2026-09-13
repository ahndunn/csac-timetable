import type { Handle } from '@sveltejs/kit';

export const handle: Handle = async ({ event, resolve }) => {
	// Intercept and reverse proxy all /api/* requests to the Rust API Gateway
	if (event.url.pathname.startsWith('/api/')) {
		const gatewayUrl =
			process.env.PUBLIC_GATEWAY_URL ||
			process.env.GATEWAY_URL ||
			'http://localhost:8080';

		const targetUrl = new URL(
			`${event.url.pathname}${event.url.search}`,
			gatewayUrl
		).toString();

		const headers = new Headers(event.request.headers);
		headers.delete('host');

		const init: RequestInit = {
			method: event.request.method,
			headers,
			// @ts-ignore
			duplex: 'half'
		};

		if (event.request.method !== 'GET' && event.request.method !== 'HEAD') {
			init.body = await event.request.arrayBuffer();
		}

		try {
			const res = await fetch(targetUrl, init);
			return res;
		} catch (err) {
			console.error('API Gateway Proxy Error:', err);
			return new Response(
				JSON.stringify({ error: 'Gateway service unavailable' }),
				{
					status: 502,
					headers: { 'Content-Type': 'application/json' }
				}
			);
		}
	}

	return resolve(event, {
		filterSerializedResponseHeaders: (name) =>
			name.toLowerCase() === 'content-type' || name.toLowerCase().startsWith('x-')
	});
};
