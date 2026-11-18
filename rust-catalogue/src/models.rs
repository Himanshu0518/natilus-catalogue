#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct RawQuoteRow {
    pub timestamp: String,
    pub instrument_id: String,
    pub bid_price: f64,
    pub ask_price: f64,
    pub bid_size: f64,
    pub ask_size: f64,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct RawTradeRow {
    pub timestamp: String,
    pub instrument_id: String,
    pub price: f64,
    pub size: f64,
    pub aggressor_side: String,
}

#[derive(Debug, Clone)]
pub struct QuoteTick {
    pub ts_event: i64,
    pub ts_init: i64,
    pub instrument_id: String,
    pub bid_price_raw: i64,
    pub ask_price_raw: i64,
    pub bid_size_raw: u64,
    pub ask_size_raw: u64,
    pub bid_price_prec: u8,
    pub ask_price_prec: u8,
    pub bid_size_prec: u8,
    pub ask_size_prec: u8,
}

#[derive(Debug, Clone)]
pub struct TradeTick {
    pub ts_event: i64,
    pub ts_init: i64,
    pub instrument_id: String,
    pub price_raw: i64,
    pub size_raw: u64,
    pub price_prec: u8,
    pub size_prec: u8,
    pub aggressor_side: String,
    pub trade_id: String,
}
