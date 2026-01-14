import type { RequestHandler } from './$types';
import { createSubscriber, resultChannels, type TaskResult } from '$lib/server/redis';

export const GET: RequestHandler = async ({ url }) => {
	const clientId = url.searchParams.get('clientId');

	if (!clientId) {
		return new Response('clientId query parameter is required', { status: 400 });
	}

	const subscriber = createSubscriber();

	const stream = new ReadableStream({
		async start(controller) {
			const encoder = new TextEncoder();

			// Send initial connection message
			controller.enqueue(encoder.encode(`data: ${JSON.stringify({ type: 'connected', clientId })}\n\n`));

			// Subscribe to all result channels
			const channels = Object.values(resultChannels);
			
			subscriber.subscribe(...channels, (err) => {
				if (err) {
					console.error('Failed to subscribe:', err);
					controller.error(err);
				}
			});

			subscriber.on('message', (channel, message) => {
				try {
					const result: TaskResult = JSON.parse(message);
					
					// Only send results for this client
					if (result.clientId === clientId) {
						const eventData = {
							type: 'result',
							...result
						};
						controller.enqueue(encoder.encode(`data: ${JSON.stringify(eventData)}\n\n`));
					}
				} catch (error) {
					console.error('Error parsing result message:', error);
				}
			});

			// Send periodic heartbeat to keep connection alive
			const heartbeatInterval = setInterval(() => {
				try {
					controller.enqueue(encoder.encode(`: heartbeat\n\n`));
				} catch {
					clearInterval(heartbeatInterval);
				}
			}, 30000);

			// Cleanup function
			return () => {
				clearInterval(heartbeatInterval);
				subscriber.unsubscribe();
				subscriber.quit();
			};
		},
		cancel() {
			subscriber.unsubscribe();
			subscriber.quit();
		}
	});

	return new Response(stream, {
		headers: {
			'Content-Type': 'text/event-stream',
			'Cache-Control': 'no-cache',
			'Connection': 'keep-alive',
			'Access-Control-Allow-Origin': '*'
		}
	});
};
