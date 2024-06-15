use time::{macros::format_description, OffsetDateTime};

pub struct BillingPeriod {
    pub start_date: String,
    pub end_date: String,
}

impl BillingPeriod {
    // 請求期間の取得
    pub fn get_billing_date() -> Self {
        // 実行時現在の日時を取得
        let now = OffsetDateTime::now_utc();

        // 実行時の1日の日付を作成
        let first_day_of_month = now.replace_day(1).expect("Failed to set to first day");

        // 先月の1日を算出
        let first_day_of_last_month = if first_day_of_month.month() == time::Month::January {
            first_day_of_month
                .replace_year(first_day_of_month.year() - 1)
                .and_then(|dt| dt.replace_month(time::Month::December))
                .expect("Failed to set to last year")
        } else {
            first_day_of_month
                .replace_month(first_day_of_month.month().previous())
                .expect("Failed to set to previous month")
        };

        // 日付をフォーマット
        let format = format_description!("[year]-[month]-[day]");
        let billing_start_date = first_day_of_last_month
            .format(&format)
            .expect("Failed to format billing start date");
        let billing_end_date = first_day_of_month
            .format(&format)
            .expect("Failed to format billing end date");

        Self {
            start_date: billing_start_date,
            end_date: billing_end_date,
        }
    }
}
