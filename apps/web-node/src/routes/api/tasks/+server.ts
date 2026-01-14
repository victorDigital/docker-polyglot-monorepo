import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import { sendTask, type WorkerLanguage } from '$lib/server/redis';

export const POST: RequestHandler = async ({ request }) => {
	try {
		const body = await request.json();
		const { expression, language, clientId, taskId } = body;

		if (!expression || typeof expression !== 'string') {
			return json({ error: 'Expression is required' }, { status: 400 });
		}

		if (!language || !['rust', 'typescript', 'python'].includes(language)) {
			return json({ error: 'Invalid language. Must be rust, typescript, or python' }, { status: 400 });
		}

		if (!clientId || typeof clientId !== 'string') {
			return json({ error: 'clientId is required' }, { status: 400 });
		}

		if (!taskId || typeof taskId !== 'string') {
			return json({ error: 'taskId is required' }, { status: 400 });
		}

		await sendTask(language as WorkerLanguage, expression, clientId, taskId);

		return json({ 
			success: true, 
			taskId,
			message: `Task sent to ${language} worker`
		});
	} catch (error) {
		console.error('Error sending task:', error);
		return json({ error: 'Failed to send task' }, { status: 500 });
	}
};
