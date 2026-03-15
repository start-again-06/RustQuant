from ingestion.yahoo_fetcher import fetch_yahoo
from preprocessing.clean_data import clean_data
from feature_engineering.technical_indicators import moving_average

def run_pipeline():

    print("Running data pipeline...")

    df = fetch_yahoo("AAPL")

    df = clean_data(df)

    df["ma"] = moving_average(df["price"])

    print(df.tail())
