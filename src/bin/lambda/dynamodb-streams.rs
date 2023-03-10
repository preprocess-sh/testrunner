use lambda_runtime::{service_fn, LambdaEvent};
use testrunner::{
    entrypoints::lambda::dynamodb::{model::DynamoDBEvent, parse_events},
    utils::{get_event_bus, setup_tracing},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    // Initialize logger
    setup_tracing();

    // Initialize event bus
    let event_bus = get_event_bus().await;

    // Run the Lambda function
    lambda_runtime::run(service_fn(|event: LambdaEvent<DynamoDBEvent>| {
        let (event, ctx) = event.into_parts();
        parse_events(&event_bus, event, ctx)
    }))
    .await?;

    Ok(())
}
