use crate::models::{QuoteTick, RawQuoteRow, RawTradeRow, TradeTick};
use chrono::DateTime;

pub fn parse_timestamp(s: &str) -> anyhow::Result<i64> {
    let dt = DateTime::parse_from_rfc3339(s)?;
    Ok(dt.timestamp_nanos_opt().unwrap_or(dt.timestamp() * 1_000_000_000))
}

pub fn price_to_raw(price: f64, precision: u8) -> i64 {
    let factor = 10f64.powi(precision as i32);
    (price * factor).round() as i64
}

pub fn size_to_raw(size: f64, precision: u8) -> u64 {
    let factor = 10f64.powi(precision as i32);
    (size * factor).round() as u64
}

pub fn convert_quotes(rows: Vec<RawQuoteRow>) -> Vec<QuoteTick> {
    rows.into_iter().filter_map(|row| {
        let ts = parse_timestamp(&row.timestamp).ok()?;
        Some(QuoteTick {
            ts_event: ts,
            ts_init: ts,
            instrument_id: row.instrument_id,
            bid_price_raw: price_to_raw(row.bid_price, 2),
            ask_price_raw: price_to_raw(row.ask_price, 2),
            bid_size_raw: size_to_raw(row.bid_size, 4),
            ask_size_raw: size_to_raw(row.ask_size, 4),
            bid_price_prec: 2,
            ask_price_prec: 2,
            bid_size_prec: 4,
            ask_size_prec: 4,
        })
    }).collect()
}

pub fn convert_trades(rows: Vec<RawTradeRow>) -> Vec<TradeTick> {
    rows.into_iter().enumerate().filter_map(|(i, row)| {
        let ts = parse_timestamp(&row.timestamp).ok()?;
        Some(TradeTick {
            ts_event: ts,
            ts_init: ts,
            instrument_id: row.instrument_id,
            price_raw: price_to_raw(row.price, 2),
            size_raw: size_to_raw(row.size, 4),
            price_prec: 2,
            size_prec: 4,
            aggressor_side: row.aggressor_side,
            trade_id: format!("TRADE-{:06}", i),
        })
    }).collect()
}
