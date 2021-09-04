#!/bin/sh

curl -s \
  -D "/dev/stderr" \
  -X GET \
  -H "Content-Type: application/json" \
  -u esteban@fluxcap.com:12345 \
  http://0.0.0.0:3000/api/v1/users | json_pp
