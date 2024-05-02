#!/bin/sh

cat localities.json | jq -f geojson.jq > localities.geojson
cat postcodes.json | jq -f geojson.jq > postcodes.geojson
