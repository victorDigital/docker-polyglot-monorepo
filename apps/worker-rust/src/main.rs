use redis::{Client, AsyncCommands};
use redis_work_queue::{Item, KeyPrefix, WorkQueue};
use serde::{Deserialize, Serialize};
use std::env;
use std::time::Duration;

#[derive(Debug, Deserialize)]
struct TaskData {
    number: u64,
    #[serde(rename = "clientId")]
    client_id: String,
    timestamp: u64,
}

#[derive(Debug, Serialize)]
struct ResultData {
    #[serde(rename = "taskId")]
    task_id: String,
    #[serde(rename = "clientId")]
    client_id: String,
    number: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
    language: String,
    timestamp: u64,
}

async fn process_task(
    work_queue: &WorkQueue,
    db: &mut redis::aio::MultiplexedConnection,
    publisher: &mut redis::aio::MultiplexedConnection,
    item: &Item,
) -> Result<(), Box<dyn std::error::Error>> {
    let data: TaskData = serde_json::from_slice(&item.data)?;
    
    println!("Processing task {}: count primes up to {}", item.id, data.number);
    
    // Compute the number of primes <= number
    let result_value = count_primes(data.number);
    
    let (result, error) = match result_value {
        Ok(val) => {
            println!("Result: {}", val);
            (Some(val.to_string()), None)
        }
        Err(e) => {
            println!("Error computing primes: {}", e);
            (None, Some(e))
        }
    };
    
    // Publish result back via Redis pub/sub
    let result_data = ResultData {
        task_id: item.id.to_string(),
        client_id: data.client_id,
        number: data.number,
        result,
        error,
        language: "rust".to_string(),
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_millis() as u64,
    };
    
    let result_json = serde_json::to_string(&result_data)?;
    publisher.publish::<_, _, ()>("results:rust", result_json).await?;
    println!("Published result for task {}", item.id);
    
    // Mark task as complete
    work_queue.complete(db, item).await?;
    
    Ok(())
}

fn count_primes(n: u64) -> Result<u64, String> {
    if n < 2 {
        return Ok(0);
    }
    
    let mut is_prime = vec![true; (n + 1) as usize];
    is_prime[0] = false;
    is_prime[1] = false;
    
    let sqrt_n = (n as f64).sqrt() as u64;
    for i in 2..=sqrt_n {
        if is_prime[i as usize] {
            let mut j = i * i;
            while j <= n {
                is_prime[j as usize] = false;
                j += i;
            }
        }
    }
    
    let count = is_prime.iter().filter(|&&p| p).count() as u64;
    Ok(count)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let redis_host = env::var("REDIS_HOST").unwrap_or_else(|_| "localhost".to_string());
    let redis_port = env::var("REDIS_PORT").unwrap_or_else(|_| "6379".to_string());
    let redis_url = format!("redis://{}:{}", redis_host, redis_port);
    
    println!("Rust Worker starting...");
    println!("Connected to Redis at {}:{}", redis_host, redis_port);
    
    let client = Client::open(redis_url)?;
    let mut db = client.get_multiplexed_async_connection().await?;
    let mut publisher = client.get_multiplexed_async_connection().await?;
    
    let work_queue = WorkQueue::new(KeyPrefix::from("rust_jobs"));
    
    println!("Worker Rust is running and waiting for tasks...");
    
    loop {
        match work_queue.lease(&mut db, None, Duration::from_secs(30)).await {
            Ok(Some(item)) => {
                if let Err(e) = process_task(&work_queue, &mut db, &mut publisher, &item).await {
                    eprintln!("Error processing task: {}", e);
                }
            }
            Ok(None) => {
                // No task available, wait a bit
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
            Err(e) => {
                eprintln!("Error leasing task: {}", e);
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        }
    }
}
