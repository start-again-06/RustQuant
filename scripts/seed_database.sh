#!/bin/bash

echo "Seeding database..."

psql $DATABASE_URL -f infrastructure/database/seed_data.sql

echo "Database seeded"
