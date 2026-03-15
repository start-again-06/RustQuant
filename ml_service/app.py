from fastapi import FastAPI
from api.routes import router

app = FastAPI(title="AI Trading Service")

app.include_router(router)
