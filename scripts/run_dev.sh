#!/bin/bash

echo "DATABASE_URL=${DATABASE_URL}"

# run migrations
movine init
movine fix

cargo run
