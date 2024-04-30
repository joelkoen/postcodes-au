use std::{
    collections::HashMap,
    fs::{self, File},
    io::{BufRead, BufReader},
};

use anyhow::Result;
use geojson::{Feature, FeatureCollection, Geometry};
use serde::Serialize;

fn main() -> Result<()> {
    let reader = BufReader::new(zstd::Decoder::new(File::open("gnaf-core.psv.zst")?)?);

    let mut states = HashMap::new();
    let mut postcodes: HashMap<String, HashMap<String, Vec<(f64, f64)>>> = HashMap::new();
    for result in reader.lines().skip(1) {
        let line = result?;
        let fields: Vec<_> = line.split('|').collect();
        assert!(fields.len() == 27);

        let latitude: f64 = fields[26].parse()?;
        let longitude: f64 = fields[25].parse()?;
        let postcode = fields[17];
        let state = fields[16];
        let locality = fields[15];

        if !states.contains_key(postcode) {
            states.insert(postcode.to_string(), state.to_string());
        }

        let point = (latitude, longitude);

        if !postcodes.contains_key(postcode) {
            postcodes.insert(postcode.to_string(), HashMap::new());
        }
        let localities = postcodes.get_mut(postcode).unwrap();
        if let Some(points) = localities.get_mut(locality) {
            points.push(point)
        } else {
            localities.insert(locality.to_string(), vec![point]);
        }
    }

    let mut output_localities = Vec::new();
    let mut output_postcodes = Vec::new();
    for (postcode, localities) in postcodes {
        let mut p_count = 0;
        let mut p_latitude = 0.0;
        let mut p_longitude = 0.0;

        let state = states.remove(&postcode).unwrap();

        for (locality, points) in localities {
            let count = points.len();
            let mut latitude = 0.0;
            let mut longitude = 0.0;
            for (x, y) in points {
                latitude += x;
                longitude += y;
            }

            p_count += count;
            p_latitude += latitude;
            p_longitude += longitude;

            latitude /= count as f64;
            longitude /= count as f64;
            latitude = (latitude * 10000.0).round() / 10000.0;
            longitude = (longitude * 10000.0).round() / 10000.0;

            let mut should_caps: bool = true;
            let mut new = String::new();
            for x in locality.chars() {
                let x = if should_caps {
                    should_caps = false;
                    x
                } else {
                    if x == ' ' {
                        should_caps = true;
                    }
                    x.to_ascii_lowercase()
                };
                new.push(x);
            }

            output_localities.push(Locality {
                locality: new,
                state: state.clone(),
                postcode: postcode.clone(),
                count,
                latitude,
                longitude,
            })
        }

        p_latitude /= p_count as f64;
        p_longitude /= p_count as f64;
        p_latitude = (p_latitude * 10000.0).round() / 10000.0;
        p_longitude = (p_longitude * 10000.0).round() / 10000.0;

        output_postcodes.push(Postcode {
            state,
            postcode,
            count: p_count,
            latitude: p_latitude,
            longitude: p_longitude,
        })
    }

    output_localities.sort_by(|a, b| b.count.cmp(&a.count));
    fs::write(
        "localities.json",
        serde_json::to_string_pretty(&output_localities)?,
    )?;
    output_postcodes.sort_by(|a, b| b.count.cmp(&a.count));
    fs::write(
        "postcodes.json",
        serde_json::to_string_pretty(&output_postcodes)?,
    )?;

    Ok(())
}

#[derive(Serialize)]
struct Locality {
    locality: String,
    state: String,
    postcode: String,
    count: usize,
    latitude: f64,
    longitude: f64,
}

#[derive(Serialize)]
struct Postcode {
    state: String,
    postcode: String,
    count: usize,
    latitude: f64,
    longitude: f64,
}
