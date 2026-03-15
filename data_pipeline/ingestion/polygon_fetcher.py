import requests

def fetch_polygon(symbol):

    url = f"https://api.polygon.io/v2/aggs/ticker/{symbol}"

    r = requests.get(url)

    return r.json()
