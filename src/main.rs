use aws_sdk_costexplorer::types::ResultByTime;
use aws_sdk_costexplorer::{
    self as costexplorer,
    types::{self, DateInterval},
};

#[::tokio::main]
async fn main() -> Result<(), costexplorer::Error> {
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_costexplorer::Client::new(&config);

    // TODO: endは実行日、startはその１ヶ月前で算出させるようにする。
    let start_date = "2024-05-01";
    let end_date = "2024-06-01";

    let time_period = DateInterval::builder()
        .start(start_date)
        .end(end_date)
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
    // println!("{:?}", cost_result);
    Ok(())
}

fn get_amount_in_usd(res: &ResultByTime) -> Option<f64> {
    let total = res.total.as_ref()?;
    let metric = total.get("AmortizedCost")?;
    let amount_str = metric.amount.as_ref()?;
    amount_str.parse::<f64>().ok()
}
