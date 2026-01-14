import Redis from 'ioredis';
import { WorkQueue, KeyPrefix, Item } from '@mevitae/redis-work-queue';

const redisHost = process.env.REDIS_HOST || 'localhost';
const redisPort = parseInt(process.env.REDIS_PORT || '6379');

// Main Redis connection for work queue operations
export const redis = new Redis({
	host: redisHost,
	port: redisPort,
	lazyConnect: true
});

// Separate connection for subscribing to results
export function createSubscriber() {
	return new Redis({
		host: redisHost,
		port: redisPort
	});
}

// Work queues for each language
export const workQueues = {
	rust: new WorkQueue(new KeyPrefix('rust_jobs')),
	typescript: new WorkQueue(new KeyPrefix('typescript_jobs')),
	python: new WorkQueue(new KeyPrefix('python_jobs'))
} as const;

export type WorkerLanguage = keyof typeof workQueues;

// Result channels for each language
export const resultChannels = {
	rust: 'results:rust',
	typescript: 'results:typescript',
	python: 'results:python'
} as const;

export interface TaskData {
	expression: string;
	clientId: string;
	timestamp: number;
}

export interface TaskResult {
	taskId: string;
	clientId: string;
	expression: string;
	result?: string;
	error?: string;
	language: string;
	timestamp: number;
}

export async function sendTask(
	language: WorkerLanguage,
	expression: string,
	clientId: string,
	taskId: string
): Promise<void> {
	const taskData: TaskData = {
		expression,
		clientId,
		timestamp: Date.now()
	};

	const item = new Item(JSON.stringify(taskData), taskId);
	await workQueues[language].addItem(redis, item);

	console.log(`Task ${taskId} sent to ${language} queue: ${expression}`);
}

export { Item };
