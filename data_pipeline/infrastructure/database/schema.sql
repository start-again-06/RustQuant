CREATE TABLE market_prices(

    id SERIAL PRIMARY KEY,
    symbol TEXT,
    price FLOAT,
    timestamp TIMESTAMP
);

CREATE TABLE features(

    id SERIAL PRIMARY KEY,
    symbol TEXT,
    moving_avg FLOAT,
    volatility FLOAT
);
