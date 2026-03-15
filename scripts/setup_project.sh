#!/bin/bash

echo "Setting up RustQuantAI Project"

echo "Installing backend dependencies"
cd backend
cargo build

echo "Installing ML service dependencies"
cd ../ml_service
pip install -r requirements.txt

echo "Installing frontend dependencies"
cd ../frontend
npm install

echo "Setup complete"
