import { createClient } from 'redis';

const REDIS_HOST = process.env.REDIS_HOST || 'redis';

async function main() {
  console.log('📘 TypeScript Worker starting...');

  // Connect to Redis
  const client = createClient({
    socket: {
      host: REDIS_HOST,
      port: 6379
    }
  });

  client.on('error', (err) => console.error('Redis Client Error', err));

  await client.connect();
  console.log(`Connected to Redis at ${REDIS_HOST}:6379`);

  let counter = 0;
  
  // Run worker loop
  setInterval(async () => {
    counter++;
    console.log(`TypeScript Worker running... iteration ${counter}`);
    
    // Set a value in Redis
    await client.set('ts-worker-status', `alive-${counter}`);
  }, 5000);
}

main().catch(console.error);
