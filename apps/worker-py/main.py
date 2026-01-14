import time
import redis
import os

def main():
    print("🐍 Python Worker starting...")
    
    # Connect to Redis
    redis_host = os.getenv('REDIS_HOST', 'redis')
    r = redis.Redis(host=redis_host, port=6379, decode_responses=True)
    
    print(f"Connected to Redis at {redis_host}:6379")
    
    counter = 0
    while True:
        counter += 1
        print(f"Python Worker running... iteration {counter}")
        
        # Set a value in Redis
        r.set('python-worker-status', f'alive-{counter}')
        
        time.sleep(5)

if __name__ == "__main__":
    main()
