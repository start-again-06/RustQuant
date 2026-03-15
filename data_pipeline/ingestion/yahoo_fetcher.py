import requests
import pandas as pd

def fetch_yahoo(symbol):

    url = f"https://query1.finance.yahoo.com/v7/finance/chart/{symbol}"

    r = requests.get(url).json()

    prices = r["chart"]["result"][0]["indicators"]["quote"][0]["close"]

    df = pd.DataFrame({
        "price": prices
    })

    return df
