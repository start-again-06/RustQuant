from fastapi import Request
from inference.predictor import predict

async def predict_signal(request: Request):

    data = await request.json()

    prices = data["prices"]

    signal = predict(prices)

    return {
        "signal": signal
    }
