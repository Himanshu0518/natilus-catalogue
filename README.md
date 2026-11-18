# NautilusTrader Catalogue — BTC Options

Convert raw historical CSV data into a NautilusTrader `ParquetDataCatalog` and an equivalent Rust binary.

---

## What is the ParquetDataCatalog?

[NautilusTrader](https://nautilustrader.io/) stores market data in a `ParquetDataCatalog` — a directory of Parquet files organised by data type and instrument. The layout mirrors the Python class hierarchy so the backtesting engine can load data efficiently without scanning every file.

### Folder structure

```
catalogue/
└── data/
    ├── quote_tick/
    │   ├── BTC-27DEC24-100000-C.BYBIT.parquet
    │   └── BTC-27DEC24-100000-P.BYBIT.parquet
    └── trade_tick/
        ├── BTC-27DEC24-100000-C.BYBIT.parquet
        └── BTC-27DEC24-100000-P.BYBIT.parquet
```

Each Parquet file contains all ticks for one instrument and one data type, sorted by `ts_event` (nanosecond Unix timestamp).

---

## How to run the Python converter

From the **straddle project root**:

```powershell
uv run python natilus-catalogue/convert_to_catalogue.py
```

This reads the two CSV files in `sample_data/`, builds NautilusTrader tick objects, writes them to `natilus-catalogue/catalogue/`, then reads them back and prints a sample to verify.

---

## CSV column mapping

### btc_quotes.csv → QuoteTick

| CSV column      | NautilusTrader field | Type       | Notes                        |
|-----------------|----------------------|------------|------------------------------|
| `timestamp`     | `ts_event`, `ts_init`| `int` (ns) | ISO 8601 → nanoseconds       |
| `instrument_id` | `instrument_id`      | `InstrumentId` | `SYMBOL.VENUE` format    |
| `bid_price`     | `bid_price`          | `Price`    | 2 decimal places             |
| `ask_price`     | `ask_price`          | `Price`    | 2 decimal places             |
| `bid_size`      | `bid_size`           | `Quantity` | 4 decimal places (BTC)       |
| `ask_size`      | `ask_size`           | `Quantity` | 4 decimal places (BTC)       |

### btc_trades.csv → TradeTick

| CSV column        | NautilusTrader field | Type          | Notes                   |
|-------------------|----------------------|---------------|-------------------------|
| `timestamp`       | `ts_event`, `ts_init`| `int` (ns)    | ISO 8601 → nanoseconds  |
| `instrument_id`   | `instrument_id`      | `InstrumentId`|                         |
| `price`           | `price`              | `Price`       | 2 decimal places        |
| `size`            | `size`               | `Quantity`    | 4 decimal places (BTC)  |
| `aggressor_side`  | `aggressor_side`     | `AggressorSide`| `BUYER` / `SELLER`     |
| *(auto)*          | `trade_id`           | `TradeId`     | UUID4 generated         |

---

## Rust port — `rust-catalogue/`

The `rust-catalogue/` directory contains a Cargo project that replicates the same pipeline in Rust.

### Build

```powershell
cd natilus-catalogue/rust-catalogue
cargo build --release
```

### Convert quotes CSV to Parquet

```powershell
cargo run -- convert quotes ..\sample_data\btc_quotes.csv .\output
```

### Convert trades CSV to Parquet

```powershell
cargo run -- convert trades ..\sample_data\btc_trades.csv .\output
```

### Inspect a catalogue directory

```powershell
cargo run -- info .\output
```

### Rust source layout

| File                    | Purpose                                            |
|-------------------------|----------------------------------------------------|
| `src/models.rs`         | `RawQuoteRow`, `RawTradeRow`, `QuoteTick`, `TradeTick` structs |
| `src/converter.rs`      | CSV row → typed tick conversion, timestamp parsing |
| `src/parquet_writer.rs` | Arrow schema + Parquet file writing, catalogue info |
| `src/main.rs`           | CLI entry point (clap)                             |
| `src/lib.rs`            | Re-exports all modules                             |
