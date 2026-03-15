import numpy as np

def predict(prices):

    prices = np.array(prices)

    if prices[-1] > prices.mean():

        return "BUY"

    return "SELL"
