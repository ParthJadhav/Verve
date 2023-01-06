use chrono::{Local, TimeZone};
use chrono_tz::{OffsetName, Tz};

use num_format::SystemLocale;
use smartcalc::SmartCalc;

pub fn calculate(input: &str) -> String {
    let locale = SystemLocale::default().unwrap();
    let timezone = match localzone::get_local_zone() {
        Some(tz) => match tz.parse::<Tz>() {
            Ok(tz) => {
                let date_time = Local::now().date_naive();
                let mut offset = tz.offset_from_utc_date(&date_time)
                    .abbreviation()
                    .to_string();
                if offset.starts_with('+') || offset.starts_with('-') {
                    offset.insert_str(0, "GMT");
                }
                offset
            }
            Err(_) => "UTC".to_string(),
        },
        None => "UTC".to_string(),
    };

    let mut app = SmartCalc::default();

    app.set_decimal_seperator(locale.decimal().to_string());
    app.set_thousand_separator(locale.separator().to_string());
    app.set_timezone(timezone).unwrap();

    let language = "en".to_string();
    let results = app.execute(language, input);
    let mut rs = String::new();
    for result in results.lines.iter() {
        match result {
            Some(result) => match &result.result {
                Ok(output) => {
                    rs = output.output.to_string();
                }
                Err(error) => println!("Error : {}", error),
            },
            None => println!("No query"),
        }
    }
    return rs;
}
