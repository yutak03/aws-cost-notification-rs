use aws_sdk_costexplorer::operation::get_cost_and_usage::GetCostAndUsageOutput;
use aws_sdk_costexplorer::types::{DateInterval};

pub fn get_billing_cost(_timeperiod: DateInterval) -> GetCostAndUsageOutput {
    unimplemented!()
}