use chrono::DateTime;
use chrono_tz::Asia::Tokyo;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Weather {
  #[serde(rename = "publicTime")]
  public_time: String,
  forecasts: Vec<Forecast>,
  location: Location,
}

#[derive(Debug, Deserialize)]
struct Forecast {
  date: String,
  #[serde(rename = "dateLabel")]
  date_label: String,
  telop: String,
  detail: Detail,
  temperature: Temperature,
  #[serde(rename = "chanceOfRain")]
  chance_of_rain: ChanceOfRain,
}

#[derive(Debug, Deserialize)]
struct Detail {
  weather: Option<String>,
  wind: Option<String>,
  wave: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Temperature {
  min: Option<TempValue>,
  max: Option<TempValue>,
}

#[derive(Debug, Deserialize)]
struct TempValue {
  celsius: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ChanceOfRain {
  #[serde(rename = "T00_06")]
  t00_06: Option<String>,
  #[serde(rename = "T06_12")]
  t06_12: Option<String>,
  #[serde(rename = "T12_18")]
  t12_18: Option<String>,
  #[serde(rename = "T18_24")]
  t18_24: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Location {
  city: String,
  prefecture: String,
}

#[tokio::main]
async fn main() {
  let url = "https://weather.tsukumijima.net/api/forecast/city/230010";
  loop {
    let body = reqwest::get(url)
      .await
      .expect("Failed to get response")
      .json::<Weather>()
      .await
      .expect("Failed to parse response");

    println!("\n===========================================");
    println!(
      "📍 地域: {} - {}",
      body.location.prefecture, body.location.city
    );
    println!("🕐 発表時刻: {}", format_datetime(&body.public_time));
    println!("===========================================");

    let forecast = &body.forecasts[0];

    println!("\n【{}（{}）】", forecast.date_label, forecast.date);
    println!("  ☀️ 天気: {}", forecast.telop);

    if let Some(ref weather) = forecast.detail.weather {
      println!("  📝 詳細: {}", weather);
    }

    let min_temp = forecast
      .temperature
      .min
      .as_ref()
      .and_then(|t| t.celsius.as_ref())
      .map(|t| t.as_str())
      .unwrap_or("-");
    let max_temp = forecast
      .temperature
      .max
      .as_ref()
      .and_then(|t| t.celsius.as_ref())
      .map(|t| t.as_str())
      .unwrap_or("-");
    println!("  🌡️ 気温: 最低{}℃ / 最高{}℃", min_temp, max_temp);
    println!("  ☔ 降水確率:");
    if let Some(ref rain) = forecast.chance_of_rain.t00_06 {
      println!("     00-06時: {}", rain);
    }
    if let Some(ref rain) = forecast.chance_of_rain.t06_12 {
      println!("     06-12時: {}", rain);
    }
    if let Some(ref rain) = forecast.chance_of_rain.t12_18 {
      println!("     12-18時: {}", rain);
    }
    if let Some(ref rain) = forecast.chance_of_rain.t18_24 {
      println!("     18-24時: {}", rain);
    }
    if let Some(ref wind) = forecast.detail.wind {
      println!("  💨 風: {}", wind);
    }
    if let Some(ref wave) = forecast.detail.wave {
      println!("  🌊 波: {}", wave);
    }
    println!("\n===========================================\n");
    tokio::time::sleep(std::time::Duration::from_secs(10)).await;
  }
}

fn format_datetime(datetime_str: &str) -> String {
  match DateTime::parse_from_rfc3339(datetime_str) {
    Ok(dt) => {
      let tokyo_time = dt.with_timezone(&Tokyo);
      tokyo_time.format("%Y年%m月%d日 %H時%M分").to_string()
    }
    Err(_) => datetime_str.to_string(),
  }
}
