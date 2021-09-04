#!/bin/sh

curl -s \
  -D "/dev/stderr" \
  -X POST \
  -H "Content-Type: application/json" \
  -d '{"first_name": "Esteban", "surname": "Borai", "email": "esteban@fluxcap.com", "username": "esteban", "password": "12345", "avatar_url": "https://via.placeholder.com/200"}' \
  http://0.0.0.0:3000/api/v1/users | json_pp
