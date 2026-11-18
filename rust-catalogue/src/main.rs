use anyhow::Context;
use catalogue_converter::{converter, parquet_writer};
use clap::Parser;

#[derive(clap::Parser)]
#[command(name = "catalogue_converter", about = "NautilusTrader catalogue converter")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    Convert {
        #[command(subcommand)]
        data_type: DataTypeCmd,
    },
    Info {
        catalogue_dir: std::path::PathBuf,
    },
}

#[derive(clap::Subcommand)]
enum DataTypeCmd {
    Quotes {
        input_csv: std::path::PathBuf,
        output_dir: std::path::PathBuf,
    },
    Trades {
        input_csv: std::path::PathBuf,
        output_dir: std::path::PathBuf,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Convert { data_type } => match data_type {
            DataTypeCmd::Quotes { input_csv, output_dir } => {
                let mut rdr = csv::Reader::from_path(&input_csv)
                    .with_context(|| format!("Cannot open {}", input_csv.display()))?;

                let rows: Vec<catalogue_converter::models::RawQuoteRow> = rdr
                    .deserialize()
                    .collect::<Result<_, _>>()
                    .context("Failed to parse quote CSV")?;

                let count = rows.len();
                let ticks = converter::convert_quotes(rows);

                let output_path = output_dir.join("quote_ticks.parquet");
                parquet_writer::write_quotes_parquet(&ticks, &output_path)
                    .context("Failed to write quotes parquet")?;

                println!("Wrote {} quote ticks to {}", count, output_path.display());
            }
            DataTypeCmd::Trades { input_csv, output_dir } => {
                let mut rdr = csv::Reader::from_path(&input_csv)
                    .with_context(|| format!("Cannot open {}", input_csv.display()))?;

                let rows: Vec<catalogue_converter::models::RawTradeRow> = rdr
                    .deserialize()
                    .collect::<Result<_, _>>()
                    .context("Failed to parse trade CSV")?;

                let count = rows.len();
                let ticks = converter::convert_trades(rows);

                let output_path = output_dir.join("trade_ticks.parquet");
                parquet_writer::write_trades_parquet(&ticks, &output_path)
                    .context("Failed to write trades parquet")?;

                println!("Wrote {} trade ticks to {}", count, output_path.display());
            }
        },
        Commands::Info { catalogue_dir } => {
            parquet_writer::catalogue_info(&catalogue_dir)?;
        }
    }

    Ok(())
}
