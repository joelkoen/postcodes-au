{
  "type": "FeatureCollection",
  "features": sort_by(.count) | map(
    {
      "type": "Feature",
      "properties": { 
        locality, state, postcode, count,
        "marker-size": "small",
      },
      "geometry": {
        "type": "Point",
        "coordinates": [ .longitude, .latitude ]
      }
    })
}
