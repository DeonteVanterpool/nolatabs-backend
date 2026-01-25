#!/bin/bash
source ./.env
curl -s "https://identitytoolkit.googleapis.com/v1/accounts:signUp?key=${FIREBASE_API_KEY}" \
-H 'Content-Type: application/json' \
--data-binary '{"email":"deonte.vanterpool@outlook.com","password":"password","returnSecureToken":true}' | jq -r '.idToken'
