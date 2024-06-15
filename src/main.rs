use aws_cost_notification::slack::post_message;
use aws_cost_notification::aws::get_billing_date;
use aws_sdk_costexplorer::types::ResultByTime;
use aws_sdk_costexplorer::types::{self, DateInterval};
use dotenv::dotenv;
use serde_json::json;
use std::error::Error;

fn get_amount_in_usd(res: &ResultByTime) -> Option<f64> {
    let total = res.total.as_ref()?;
    let metric = total.get("AmortizedCost")?;
    let amount_str = metric.amount.as_ref()?;
    amount_str.parse::<f64>().ok()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let config = aws_config::load_from_env().await;
    let client = aws_sdk_costexplorer::Client::new(&config);

    let billing_date = get_billing_date::get_billing_period();
    let time_period = DateInterval::builder()
        .start(billing_date.start_date)
        .end(billing_date.end_date)
        .build()
        .expect("Failed to build DateInterval struct");

    // 請求額リクエスト
    let response = client
        .get_cost_and_usage()
        .granularity(types::Granularity::Monthly)
        .time_period(time_period)
        .metrics("AmortizedCost")
        .send()
        .await?;

    let cost_result = response.results_by_time();
    // TODO: 実行時のレートを設定するようにする
    let rate = 157.42;

    for res in cost_result {
        if let Some(amount) = get_amount_in_usd(res) {
            println!("金額: {}", amount * rate);
        }
    }
    println!("{:?}", cost_result);
    let payload = json!({
        "text": "【6月 AWS請求金額】\n *¥10,684 (税込)* \n USD: $67.87\n (レート: $1 = ¥157.42)",
    });

    post_message::post_slack(&payload).await?;

    Ok(())
}
