#!/bin/sh
source ./.env
export DATABASE_URL=postgresql://${DB_USER}:$(aws secretsmanager get-secret-value --secret-id 'arn:aws:secretsmanager:us-east-1:076224130336:secret:rds!db-b2c92cbd-d8ca-4fbc-a686-d0702ed28d8c-iM4IiH' --query SecretString --output text | jq -r '.password')@${DB_HOST}:${DB_PORT}/${DB_NAME}
