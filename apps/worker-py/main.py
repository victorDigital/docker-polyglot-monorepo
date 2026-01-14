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
        expression = data['expression']
        client_id = data['clientId']
        
        print(f"Processing task {item.id()}: {expression}")
        
        # Evaluate the expression
        result = None
        error = None
        
        try:
            result = eval(expression)
            print(f"Result: {result}")
        except Exception as e:
            error = str(e)
            print(f"Error evaluating expression: {error}")
        
        # Publish result back via Redis pub/sub
        result_data = {
            'taskId': item.id(),
            'clientId': client_id,
            'expression': expression,
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

