"""
Convert raw historical CSV data to a NautilusTrader ParquetDataCatalog.

Usage (from inside this folder):
    uv run python convert_to_catalogue.py
"""

import uuid
from pathlib import Path

import pandas as pd

from nautilus_trader.model.data import QuoteTick, TradeTick
from nautilus_trader.model.identifiers import InstrumentId, TradeId
from nautilus_trader.model.objects import Price, Quantity
from nautilus_trader.model.enums import AggressorSide
from nautilus_trader.persistence.catalog import ParquetDataCatalog

BASE_DIR = Path(__file__).parent
QUOTES_CSV = BASE_DIR / "sample_data" / "btc_quotes.csv"
TRADES_CSV = BASE_DIR / "sample_data" / "btc_trades.csv"
CATALOGUE_PATH = BASE_DIR / "catalogue"


def build_quote_ticks(csv_path: Path) -> list[QuoteTick]:
    df = pd.read_csv(csv_path)
    ticks: list[QuoteTick] = []
    for _, row in df.iterrows():
        instrument_id = InstrumentId.from_str(str(row["instrument_id"]))
        ts_ns = pd.Timestamp(row["timestamp"]).value
        tick = QuoteTick(
            instrument_id=instrument_id,
            bid_price=Price(float(row["bid_price"]), 2),
            ask_price=Price(float(row["ask_price"]), 2),
            bid_size=Quantity(float(row["bid_size"]), 4),
            ask_size=Quantity(float(row["ask_size"]), 4),
            ts_event=ts_ns,
            ts_init=ts_ns,
        )
        ticks.append(tick)
    return ticks


def build_trade_ticks(csv_path: Path) -> list[TradeTick]:
    df = pd.read_csv(csv_path)
    ticks: list[TradeTick] = []
    for _, row in df.iterrows():
        instrument_id = InstrumentId.from_str(str(row["instrument_id"]))
        ts_ns = pd.Timestamp(row["timestamp"]).value
        side = (
            AggressorSide.BUYER
            if row["aggressor_side"] == "BUYER"
            else AggressorSide.SELLER
        )
        tick = TradeTick(
            instrument_id=instrument_id,
            price=Price(float(row["price"]), 2),
            size=Quantity(float(row["size"]), 4),
            aggressor_side=side,
            trade_id=TradeId(str(uuid.uuid4())),
            ts_event=ts_ns,
            ts_init=ts_ns,
        )
        ticks.append(tick)
    return ticks


def main() -> None:
    print("=" * 60)
    print("NautilusTrader CSV → ParquetDataCatalog converter")
    print("=" * 60)

    # --- Build tick objects ---
    print(f"\nReading quotes from: {QUOTES_CSV}")
    quote_ticks = build_quote_ticks(QUOTES_CSV)
    print(f"  Built {len(quote_ticks)} QuoteTick objects")

    print(f"\nReading trades from: {TRADES_CSV}")
    trade_ticks = build_trade_ticks(TRADES_CSV)
    print(f"  Built {len(trade_ticks)} TradeTick objects")

    # --- Write to catalogue ---
    print(f"\nWriting to catalogue at: {CATALOGUE_PATH}")
    CATALOGUE_PATH.mkdir(parents=True, exist_ok=True)
    catalog = ParquetDataCatalog(str(CATALOGUE_PATH))

    catalog.write_data(quote_ticks)
    catalog.write_data(trade_ticks)

    # --- Per-instrument summary ---
    print("\n--- Ticks written per instrument ---")
    from collections import Counter
    quote_counts: Counter = Counter(str(t.instrument_id) for t in quote_ticks)
    trade_counts: Counter = Counter(str(t.instrument_id) for t in trade_ticks)

    all_instruments = sorted(set(quote_counts) | set(trade_counts))
    for inst in all_instruments:
        print(
            f"  {inst}: {quote_counts.get(inst, 0)} quotes, "
            f"{trade_counts.get(inst, 0)} trades"
        )

    # --- Read back and verify ---
    print("\n--- Read-back verification ---")
    instrument_id_strings = [str(iid) for iid in {t.instrument_id for t in quote_ticks}]

    quotes_back = catalog.quote_ticks(instrument_ids=instrument_id_strings)
    trades_back = catalog.trade_ticks(instrument_ids=instrument_id_strings)

    print(f"  Quotes read back : {len(quotes_back)}")
    print(f"  Trades read back : {len(trades_back)}")

    if quotes_back:
        print("\nSample QuoteTick (first):")
        q = quotes_back[0]
        print(f"  instrument_id : {q.instrument_id}")
        print(f"  bid_price     : {q.bid_price}")
        print(f"  ask_price     : {q.ask_price}")
        print(f"  bid_size      : {q.bid_size}")
        print(f"  ask_size      : {q.ask_size}")
        print(f"  ts_event      : {q.ts_event}")

    if trades_back:
        print("\nSample TradeTick (first):")
        t = trades_back[0]
        print(f"  instrument_id   : {t.instrument_id}")
        print(f"  price           : {t.price}")
        print(f"  size            : {t.size}")
        print(f"  aggressor_side  : {t.aggressor_side}")
        print(f"  trade_id        : {t.trade_id}")
        print(f"  ts_event        : {t.ts_event}")

    print("\n✓ Done")


if __name__ == "__main__":
    main()
