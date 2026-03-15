import numpy as np

def volatility(series):

    returns = series.pct_change()

    return np.std(returns)
