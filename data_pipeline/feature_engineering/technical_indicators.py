import pandas as pd

def moving_average(series, window=5):

    return series.rolling(window).mean()

def rsi(series, window=14):

    delta = series.diff()

    gain = delta.clip(lower=0).rolling(window).mean()

    loss = -delta.clip(upper=0).rolling(window).mean()

    rs = gain / loss

    return 100 - (100 / (1 + rs))
