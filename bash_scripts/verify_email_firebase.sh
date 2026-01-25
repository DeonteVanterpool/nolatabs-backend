#!/bin/bash
source ./.env
read -p "Enter the ID token: " ID_TOKEN
curl "https://identitytoolkit.googleapis.com/v1/accounts:sendOobCode?key=$FIREBASE_API_KEY" \
-H 'Content-Type: application/json' \
  --data-binary "{\"requestType\":\"VERIFY_EMAIL\",\"idToken\":\"$ID_TOKEN\"}"
