
use crate::config::{TestConfig, TestStep};
use reqwest::Client;
use std::time::Instant;

pub async fn run_test(config: TestConfig) {
    let client = Client::new();
    let mut previous_response: Option<String> = None;

    for step in config.steps {
        println!("Running step: {}", step.name);

        let start_time = Instant::now();
        let mut handles = vec![];

        for _ in 0..step.concurrency {
            let client = client.clone();
            let step = step.clone();
            let previous_response = previous_response.clone();

            let handle = tokio::spawn(async move {
                let mut last_response = None;
                for _ in 0..step.iterations {
                    let response = execute_request(&client, &step, &previous_response).await;
                    last_response = Some(response.unwrap_or_default());
                }
                last_response
            });

            handles.push(handle);
        }

        for handle in handles {
            previous_response = handle.await.unwrap();
        }

        let duration = start_time.elapsed();
        println!("Step '{}' finished in {:?}", step.name, duration);
    }
}

async fn execute_request(
    client: &Client,
    step: &TestStep,
    previous_response: &Option<String>,
) -> Result<String, reqwest::Error> {
    let mut request_builder = match step.request.method.to_uppercase().as_str() {
        "GET" => client.get(&step.request.url),
        "POST" => client.post(&step.request.url),
        // Add other methods as needed
        _ => panic!("Unsupported HTTP method: {}", step.request.method),
    };

    for (key, value) in &step.request.headers {
        request_builder = request_builder.header(key, value);
    }

    if !step.request.body.is_empty() {
        let mut body = step.request.body.clone();
        if let Some(prev_resp) = previous_response {
            // Very simple templating: replace {{response}} with the previous response.
            body = body.replace("{{response}}", prev_resp);
        }
        request_builder = request_builder.body(body);
    }

    let response = request_builder.send().await?;
    let response_body = response.text().await?;

    Ok(response_body)
}
