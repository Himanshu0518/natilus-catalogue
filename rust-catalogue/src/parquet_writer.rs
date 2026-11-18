use crate::models::{QuoteTick, TradeTick};
use arrow::array::{Int64Array, StringArray, UInt64Array};
use arrow::datatypes::{DataType, Field, Schema};
use arrow::record_batch::RecordBatch;
use parquet::arrow::arrow_writer::ArrowWriter;
use parquet::file::reader::{FileReader, SerializedFileReader};
use std::fs::File;
use std::path::Path;
use std::sync::Arc;

pub fn write_quotes_parquet(quotes: &[QuoteTick], output_path: &Path) -> anyhow::Result<()> {
    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let schema = Arc::new(Schema::new(vec![
        Field::new("ts_event", DataType::Int64, false),
        Field::new("ts_init", DataType::Int64, false),
        Field::new("instrument_id", DataType::Utf8, false),
        Field::new("bid_price", DataType::Int64, false),
        Field::new("ask_price", DataType::Int64, false),
        Field::new("bid_size", DataType::UInt64, false),
        Field::new("ask_size", DataType::UInt64, false),
    ]));

    let ts_event: Int64Array = quotes.iter().map(|q| q.ts_event).collect();
    let ts_init: Int64Array = quotes.iter().map(|q| q.ts_init).collect();
    let instrument_id: StringArray = quotes.iter().map(|q| Some(q.instrument_id.as_str())).collect();
    let bid_price: Int64Array = quotes.iter().map(|q| q.bid_price_raw).collect();
    let ask_price: Int64Array = quotes.iter().map(|q| q.ask_price_raw).collect();
    let bid_size: UInt64Array = quotes.iter().map(|q| q.bid_size_raw).collect();
    let ask_size: UInt64Array = quotes.iter().map(|q| q.ask_size_raw).collect();

    let batch = RecordBatch::try_new(
        schema.clone(),
        vec![
            Arc::new(ts_event),
            Arc::new(ts_init),
            Arc::new(instrument_id),
            Arc::new(bid_price),
            Arc::new(ask_price),
            Arc::new(bid_size),
            Arc::new(ask_size),
        ],
    )?;

    let file = File::create(output_path)?;
    let mut writer = ArrowWriter::try_new(file, schema, None)?;
    writer.write(&batch)?;
    writer.close()?;

    Ok(())
}

pub fn write_trades_parquet(trades: &[TradeTick], output_path: &Path) -> anyhow::Result<()> {
    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let schema = Arc::new(Schema::new(vec![
        Field::new("ts_event", DataType::Int64, false),
        Field::new("ts_init", DataType::Int64, false),
        Field::new("instrument_id", DataType::Utf8, false),
        Field::new("price", DataType::Int64, false),
        Field::new("size", DataType::UInt64, false),
        Field::new("aggressor_side", DataType::Utf8, false),
        Field::new("trade_id", DataType::Utf8, false),
    ]));

    let ts_event: Int64Array = trades.iter().map(|t| t.ts_event).collect();
    let ts_init: Int64Array = trades.iter().map(|t| t.ts_init).collect();
    let instrument_id: StringArray = trades.iter().map(|t| Some(t.instrument_id.as_str())).collect();
    let price: Int64Array = trades.iter().map(|t| t.price_raw).collect();
    let size: UInt64Array = trades.iter().map(|t| t.size_raw).collect();
    let aggressor_side: StringArray = trades.iter().map(|t| Some(t.aggressor_side.as_str())).collect();
    let trade_id: StringArray = trades.iter().map(|t| Some(t.trade_id.as_str())).collect();

    let batch = RecordBatch::try_new(
        schema.clone(),
        vec![
            Arc::new(ts_event),
            Arc::new(ts_init),
            Arc::new(instrument_id),
            Arc::new(price),
            Arc::new(size),
            Arc::new(aggressor_side),
            Arc::new(trade_id),
        ],
    )?;

    let file = File::create(output_path)?;
    let mut writer = ArrowWriter::try_new(file, schema, None)?;
    writer.write(&batch)?;
    writer.close()?;

    Ok(())
}

fn walk_dir(dir: &Path, parquet_files: &mut Vec<std::path::PathBuf>) -> anyhow::Result<()> {
    if dir.is_dir() {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                walk_dir(&path, parquet_files)?;
            } else if path.extension().and_then(|e| e.to_str()) == Some("parquet") {
                parquet_files.push(path);
            }
        }
    }
    Ok(())
}

pub fn catalogue_info(catalogue_dir: &Path) -> anyhow::Result<()> {
    let mut parquet_files: Vec<std::path::PathBuf> = Vec::new();
    walk_dir(catalogue_dir, &mut parquet_files)?;

    if parquet_files.is_empty() {
        println!("No parquet files found in {:?}", catalogue_dir);
        return Ok(());
    }

    println!("\n{:<60} {:>10} {}", "File", "Rows", "Schema");
    println!("{}", "-".repeat(100));

    let mut total_rows: i64 = 0;

    for path in &parquet_files {
        let file = File::open(path)?;
        let reader = SerializedFileReader::new(file)?;
        let meta = reader.metadata();
        let row_count: i64 = meta.row_groups().iter().map(|rg| rg.num_rows()).sum();
        total_rows += row_count;

        let schema = meta.file_metadata().schema();
        let field_names: Vec<String> = schema
            .get_fields()
            .iter()
            .map(|f| f.name().to_string())
            .collect();

        println!(
            "{:<60} {:>10} [{}]",
            path.display(),
            row_count,
            field_names.join(", ")
        );
    }

    println!("{}", "-".repeat(100));
    println!("Total: {} files, {} rows", parquet_files.len(), total_rows);

    Ok(())
}
