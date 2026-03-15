from fastapi import APIRouter
from .predict import predict_signal
from .health import health_check

router = APIRouter()

router.add_api_route("/health", health_check, methods=["GET"])
router.add_api_route("/predict", predict_signal, methods=["POST"])
