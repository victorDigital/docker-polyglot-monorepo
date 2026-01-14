import os
import json
import time
import redis
from redis_work_queue import WorkQueue, KeyPrefix, Item

redis_host = os.environ.get('REDIS_HOST', 'localhost')
redis_port = int(os.environ.get('REDIS_PORT', '6379'))

# Connect to Redis
db = redis.Redis(host=redis_host, port=redis_port, decode_responses=False)
publisher = redis.Redis(host=redis_host, port=redis_port)

# Create work queue
work_queue = WorkQueue(KeyPrefix("python_jobs"))

print("Python Worker starting...")
print(f"Connected to Redis at {redis_host}:{redis_port}")

def process_task(item: Item):
    """Process a single task"""
    try:
        data = item.data_json()
        number = data['number']
        client_id = data['clientId']
        
        print(f"Processing task {item.id()}: count primes up to {number}")
        
        # Compute the number of primes <= number
        result = None
        error = None
        
        try:
            result = count_primes(number)
            print(f"Result: {result}")
        except Exception as e:
            error = str(e)
            print(f"Error computing primes: {error}")
        
        # Publish result back via Redis pub/sub
        result_data = {
            'taskId': item.id(),
            'clientId': client_id,
            'number': number,
            'result': str(result) if error is None else None,
            'error': error,
            'language': 'python',
            'timestamp': int(time.time() * 1000)
        }
        
        publisher.publish('results:python', json.dumps(result_data))
        print(f"Published result for task {item.id()}")
        
        # Mark task as complete
        work_queue.complete(db, item)
        
    except Exception as error:
        print(f"Error processing task: {error}")
        # Don't mark as complete so it can be retried

def count_primes(n: int) -> int:
    """Count the number of prime numbers less than or equal to n"""
    if n < 2:
        return 0
    
    # Sieve of Eratosthenes
    is_prime = [True] * (n + 1)
    is_prime[0] = is_prime[1] = False
    
    for i in range(2, int(n**0.5) + 1):
        if is_prime[i]:
            for j in range(i*i, n+1, i):
                is_prime[j] = False
    
    return sum(is_prime)

def main():
    print("Worker PY is running and waiting for tasks...")
    
    while True:
        try:
            # Lease a task with 30 second timeout, block for up to 5 seconds
            item = work_queue.lease(db, lease_secs=30, block=True, timeout=5)
            
            if item:
                process_task(item)
                
        except Exception as error:
            print(f"Error in worker loop: {error}")
            # Wait a bit before retrying
            time.sleep(1)

if __name__ == "__main__":
    main()

