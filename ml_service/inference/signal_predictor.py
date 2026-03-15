def generate_signal(prediction):

    if prediction > 0:
        return "BUY"

    if prediction < 0:
        return "SELL"

    return "HOLD"
