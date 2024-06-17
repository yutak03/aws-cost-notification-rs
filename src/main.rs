use aws_cost_notification::{
    aws::billing_period::BillingPeriod,
    slack::client::SlackClient,
};
use aws_sdk_costexplorer::types::ResultByTime;
use aws_sdk_costexplorer::types::{self, DateInterval};
use dotenv::dotenv;
use serde_json::json;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let config = aws_config::load_from_env().await;
    let aws_client = aws_sdk_costexplorer::Client::new(&config);
    let slack_client = SlackClient::default();

    let billing_date = BillingPeriod::get();
    let time_period = DateInterval::builder()
        .start(billing_date.start_date)
        .end(billing_date.end_date)
        .build()
        .expect("Failed to build DateInterval struct");

    // 請求額リクエスト
    let aws_response = match aws_client
        .get_cost_and_usage()
        .granularity(types::Granularity::Monthly)
        .time_period(time_period)
        .metrics("BlendedCost")
        .send()
        .await {
            Ok(resp) => resp,
            Err(e) => {
                eprintln!("Error sending request: {}", e);
                let err_contents = "Error";
                let err_payload = json!({
                    "text": err_contents,
                });
                slack_client.post_message(&err_payload).await?;
                return Err(e.into());
            }
    };

    let cost_result = aws_response.results_by_time();

    for res in cost_result {
        if let Some(amount) = get_amount_in_usd(res) {
            println!("金額: {} USD", amount);
        }
    }
    println!("{:?}", cost_result);
    let content = "";
    let payload = json!({
        "text": content,
    });

    slack_client.post_message(&payload).await?;

    Ok(())
}

fn get_amount_in_usd(res: &ResultByTime) -> Option<f64> {
    let total = res.total.as_ref()?;
    let metric = total.get("BlendedCost")?;
    let amount_str = metric.amount.as_ref()?;
    amount_str.parse::<f64>().ok()
}