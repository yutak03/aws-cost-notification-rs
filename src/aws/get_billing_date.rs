use time::{macros::format_description, OffsetDateTime};

// TODO: もうちょっといい感じにかけると思う
pub struct BillingPeriod {
    pub start_date: String,
    pub end_date: String,
}

pub fn get_billing_period() -> BillingPeriod {
    // 現在の日付と時刻を取得
    let now = OffsetDateTime::now_utc();

    // 今月の1日の日付を作成
    let first_day_of_month = now.replace_day(1).expect("Failed to set day to 1");

    // 先月の1日を計算
    let first_day_of_last_month = if first_day_of_month.month() == time::Month::January {
        first_day_of_month
            .replace_year(first_day_of_month.year() - 1)
            .and_then(|dt| dt.replace_month(time::Month::December))
            .expect("Failed to set to last year's December")
    } else {
        first_day_of_month
            .replace_month(first_day_of_month.month().previous())
            .expect("Failed to set to previous month")
    };

    // 日付をyyyy-mm-dd形式でフォーマット
    let format = format_description!("[year]-[month]-[day]");
    let billing_start_date = first_day_of_last_month
        .format(&format)
        .expect("Failed to format date");
    let billing_end_date = first_day_of_month
        .format(&format)
        .expect("Failed to format date");

    BillingPeriod {
        start_date: billing_start_date,
        end_date: billing_end_date,
    }
}