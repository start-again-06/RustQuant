import requests

API_KEY = "demo"

def fetch_alphavantage(symbol):

    url = f"https://www.alphavantage.co/query"

    params = {
        "function":"TIME_SERIES_DAILY",
        "symbol":symbol,
        "apikey":API_KEY
    }

    return requests.get(url,params=params).json()
