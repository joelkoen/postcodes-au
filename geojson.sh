#!/bin/sh

cat data/localities-au.json | jq -f geojson.jq > data/localities-au.geojson
cat data/postcodes-au.json | jq -f geojson.jq > data/postcodes-au.geojson
