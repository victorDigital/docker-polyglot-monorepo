import Redis from 'ioredis';
import { WorkQueue, KeyPrefix, Item } from '@mevitae/redis-work-queue';

const redisHost = process.env.REDIS_HOST || 'localhost';
const redisPort = parseInt(process.env.REDIS_PORT || '6379');

const redis = new Redis({
	host: redisHost,
	port: redisPort
});

const redisPublisher = new Redis({
	host: redisHost,
	port: redisPort
});

const workQueue = new WorkQueue(new KeyPrefix('typescript_jobs'));

console.log('TypeScript Worker starting...');
console.log(`Connected to Redis at ${redisHost}:${redisPort}`);

async function processTask(item: Item) {
	try {
		const data = item.dataJson();
		const expression = data.expression;
		const clientId = data.clientId;

		console.log(`Processing task ${item.id}: ${expression}`);

		// Evaluate the expression using eval (be careful in production!)
		let result: any;
		let error: string | null = null;

		try {
			result = eval(expression);
			console.log(`Result: ${result}`);
		} catch (e: any) {
			error = e.message;
			console.error(`Error evaluating expression: ${error}`);
		}

		// Publish result back via Redis pub/sub
		const resultData = {
			taskId: item.id,
			clientId,
			expression,
			result: error ? undefined : String(result),
			error,
			language: 'typescript',
			timestamp: Date.now()
		};

		await redisPublisher.publish('results:typescript', JSON.stringify(resultData));
		console.log(`Published result for task ${item.id}`);

		// Mark task as complete
		const completed = await workQueue.complete(redis, item);
		console.log(`Task ${item.id} completed: ${completed}`);
	} catch (error) {
		console.error('Error processing task:', error);
		// Don't mark as complete so it can be retried
	}
}

async function main() {
	console.log('Worker TS is running and waiting for tasks...');

	while (true) {
		try {
			// Lease a task with 30 second timeout, block for up to 5 seconds
			const item = await workQueue.lease(redis, 30, true, 5);

			if (item) {
				await processTask(item);
			}
		} catch (error) {
			console.error('Error in worker loop:', error);
			// Wait a bit before retrying
			await new Promise(resolve => setTimeout(resolve, 1000));
		}
	}
}

main().catch(console.error);
