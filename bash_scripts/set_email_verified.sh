#!/bin/bash
source ./.env
read -p "Enter the ID token: " ID_TOKEN
curl --header "Content-Type: application/json" \
    --request POST \
    --data "{\"idToken\": \"$ID_TOKEN\", \"emailVerified\": true}" \
    "https://identitytoolkit.googleapis.com/v1/accounts:update?key=$FIREBASE_API_KEY"
