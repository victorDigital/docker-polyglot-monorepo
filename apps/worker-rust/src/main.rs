use redis::{Client, AsyncCommands};
use redis_work_queue::{Item, KeyPrefix, WorkQueue};
use serde::{Deserialize, Serialize};
use std::env;
use std::time::Duration;

#[derive(Debug, Deserialize)]
struct TaskData {
    expression: String,
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
    expression: String,
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
    let data: TaskData = serde_json::from_slice(item.data())?;
    
    println!("Processing task {}: {}", item.id(), data.expression);
    
    // Evaluate the expression using evalexpr or a simple parser
    // For now, we'll use a simple eval approach (limited support)
    let result_value = evaluate_expression(&data.expression);
    
    let (result, error) = match result_value {
        Ok(val) => {
            println!("Result: {}", val);
            (Some(val.to_string()), None)
        }
        Err(e) => {
            println!("Error evaluating expression: {}", e);
            (None, Some(e))
        }
    };
    
    // Publish result back via Redis pub/sub
    let result_data = ResultData {
        task_id: item.id().to_string(),
        client_id: data.client_id,
        expression: data.expression,
        result,
        error,
        language: "rust".to_string(),
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_millis() as u64,
    };
    
    let result_json = serde_json::to_string(&result_data)?;
    publisher.publish::<_, _, ()>("results:rust", result_json).await?;
    println!("Published result for task {}", item.id());
    
    // Mark task as complete
    work_queue.complete(db, item).await?;
    
    Ok(())
}

fn evaluate_expression(expr: &str) -> Result<f64, String> {
    // Simple expression evaluator for basic arithmetic
    // In production, you'd want to use a proper parser library like evalexpr
    
    // Remove whitespace
    let expr = expr.trim().replace(" ", "");
    
    // Try to parse simple operations
    if let Some(pos) = expr.find('+') {
        let left = &expr[..pos];
        let right = &expr[pos+1..];
        let left_val: f64 = left.parse().map_err(|e| format!("Parse error: {}", e))?;
        let right_val: f64 = right.parse().map_err(|e| format!("Parse error: {}", e))?;
        return Ok(left_val + right_val);
    } else if let Some(pos) = expr.find('-') {
        if pos > 0 {  // Not a negative number
            let left = &expr[..pos];
            let right = &expr[pos+1..];
            let left_val: f64 = left.parse().map_err(|e| format!("Parse error: {}", e))?;
            let right_val: f64 = right.parse().map_err(|e| format!("Parse error: {}", e))?;
            return Ok(left_val - right_val);
        }
    } else if let Some(pos) = expr.find('*') {
        let left = &expr[..pos];
        let right = &expr[pos+1..];
        let left_val: f64 = left.parse().map_err(|e| format!("Parse error: {}", e))?;
        let right_val: f64 = right.parse().map_err(|e| format!("Parse error: {}", e))?;
        return Ok(left_val * right_val);
    } else if let Some(pos) = expr.find('/') {
        let left = &expr[..pos];
        let right = &expr[pos+1..];
        let left_val: f64 = left.parse().map_err(|e| format!("Parse error: {}", e))?;
        let right_val: f64 = right.parse().map_err(|e| format!("Parse error: {}", e))?;
        if right_val == 0.0 {
            return Err("Division by zero".to_string());
        }
        return Ok(left_val / right_val);
    }
    
    // If no operator found, try to parse as a number
    expr.parse::<f64>().map_err(|e| format!("Parse error: {}", e))
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
