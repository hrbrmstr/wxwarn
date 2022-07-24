//! Display NOAA Weather Alerts For A Given Lat/Lon
//!
//! Grabs the NOAA weather alerts shapefile, checks to see if 
//! there are any alerts for the given coordinate, and prints
//! them if there are.
//!
//! # Examples
//! 
//! ## Rust
//! 
//! ```
//! extern crate wxwarn;
//! print_alert(43.2683199, -70.8635506);
//! ```
//! 
//! ## Command line
//! 
//! ```
//! $ wxwarn --lat="43.2683199" --lon="-70.8635506"
//! ```
//! 
//! ## Building
//! 
//! ```
//! git clone git@github.com:hrbrmstr/wxwarn
//! cargo build --release 
//! ```
//! 
//! ## Installing
//! 
//! The following will put:
//! 
//! - `wxwarn`
//! 
//! into `~/.cargo/bin` unless you've modified the behaviour of `cargo install`.
//! 
//! ```
//! $ cargo install --git https://github.com/hrbrmstr/wxwarn
//! ```
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use clap::{Parser};

use geo::prelude::Contains;
use std::io;
use std::fs::File;
use std::path::Path;
use tempfile::NamedTempFile;
use flate2::read::GzDecoder;
use tar::Archive;

/* -------------------------------------------------------------------------- */
/*            Helpers for parsing NOAA Weather API Alert responses            */
/* -------------------------------------------------------------------------- */

#[derive(Debug, Serialize, Deserialize)]
pub struct Alert {
    #[serde(rename = "@context")]
    pub context: Vec<ContextElement>,
    pub id: String,
    #[serde(rename = "type")]
    pub alert_type: String,
    pub geometry: serde_json::Value,
    pub properties: Properties,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContextClass {
    #[serde(rename = "@version")]
    pub version: String,
    pub wx: String,
    #[serde(rename = "@vocab")]
    pub vocab: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Properties {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@type")]
    pub properties_type: String,
    #[serde(rename = "id")]
    pub properties_id: String,
    #[serde(rename = "areaDesc")]
    pub area_desc: String,
    pub geocode: Geocode,
    #[serde(rename = "affectedZones")]
    pub affected_zones: Vec<String>,
    pub references: Vec<Reference>,
    pub sent: String,
    pub effective: String,
    pub onset: String,
    pub expires: String,
    pub ends: String,
    pub status: String,
    #[serde(rename = "messageType")]
    pub message_type: String,
    pub category: String,
    pub severity: String,
    pub certainty: String,
    pub urgency: String,
    pub event: String,
    pub sender: String,
    #[serde(rename = "senderName")]
    pub sender_name: String,
    pub headline: String,
    pub description: String,
    pub instruction: String,
    pub response: String,
    pub parameters: Parameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Geocode {
    #[serde(rename = "SAME")]
    pub same: Vec<String>,
    #[serde(rename = "UGC")]
    pub ugc: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Parameters {
    #[serde(rename = "AWIPSidentifier")]
    pub awip_sidentifier: Vec<String>,
    #[serde(rename = "WMOidentifier")]
    pub wm_oidentifier: Vec<String>,
    #[serde(rename = "NWSheadline")]
    pub nw_sheadline: Vec<String>,
    #[serde(rename = "BLOCKCHANNEL")]
    pub blockchannel: Vec<String>,
    #[serde(rename = "VTEC")]
    pub vtec: Vec<String>,
    #[serde(rename = "eventEndingTime")]
    pub event_ending_time: Vec<String>,
    #[serde(rename = "expiredReferences")]
    pub expired_references: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reference {
    #[serde(rename = "@id")]
    pub id: String,
    pub identifier: String,
    pub sender: String,
    pub sent: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContextElement {
    ContextClass(ContextClass),
    String(String),
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {

  // latitude
  #[clap(long, default_value_t = String::from("43.2683199"))]
  lat: String,

  // longitude
  #[clap(long, default_value_t = String::from("-70.8635506"))]
  lon: String,
  
}

pub fn print_alert(lat: f64, lon: f64) {

  /* -------------------------------------------------------------------------- */
  /*           We'll use this request client here and for the NOAA API          */
  /* -------------------------------------------------------------------------- */

  let builder = reqwest::blocking::ClientBuilder::new();
  let client = builder.build().expect("Could not build client");

  // Grab the alerts shapfile
  let mut resp = client
    .get("https://tgftp.nws.noaa.gov/SL.us008001/DF.sha/DC.cap/DS.WWA/current_all.tar.gz")
    .send()
    .expect("Failed to download alerts shapefile.");

  // save it to a temporary directory
  let mut tf = NamedTempFile::new().expect("Cannot create temp file");
  io::copy(&mut resp, &mut tf).expect("failed to copy content");

  // unpack it
  let current_all_tar_gz = File::open(tf.path()).expect("Cannot open alerts shapefile.");
  let current_all_tar = GzDecoder::new(current_all_tar_gz);
  let mut archive = Archive::new(current_all_tar);
  let expand_path = Path::new(tf.path()).parent().expect("Allocation or parse error");
  archive.unpack(expand_path).expect("Error unpacking tar file");

  // read it in
  let polygons = shapefile::read_as::<_, shapefile::Polygon, shapefile::dbase::Record>(
    format!("{}/current_all.shp", expand_path.to_str().unwrap()),
  )
  .expect("Could not open polygon-shapefile");

  let mut times = 0;
  
  // go through each polygon. if our location is within one of the polygons
  // get the relevant info to use with the NOAA API
  for (polygon, polygon_record) in polygons {

    let geo_polygon: geo::MultiPolygon<f64> = polygon.into();

    if geo_polygon.contains(&geo::point!(x: lon, y: lat)) {

      times += 1;

      let cap_id = match polygon_record.get("CAP_ID") {
        Some(shapefile::dbase::FieldValue::Character(Some(x))) => x,
        Some(_) => panic!("Expected 'CAP_ID' to be a character"),
        None => panic!("Field 'CAP_ID' is not within the record"),
      };

      // TODO mimic the R script and only show the latest alert; we need these fields for that
      let _prod_type = match polygon_record.get("PROD_TYPE") {
        Some(shapefile::dbase::FieldValue::Character(Some(x))) => x,
        Some(_) => panic!("Expected 'PROD_TYPE' to be a character"),
        None => panic!("Field 'PROD_TYPE' is not within the record"),
      };

      let _issuance = match polygon_record.get("ISSUANCE") {
        Some(shapefile::dbase::FieldValue::Character(Some(x))) => x,
        Some(_) => panic!("Expected 'ISSUANCE' to be a character"),
        None => panic!("Field 'ISSUANCE' is not within the record"),
      };
   
      let resp = client
        .get(format!("https://api.weather.gov/alerts/{}", cap_id))
        .header("User-Agent", "(rud.is, bob@rud.is)")
        .header("Accept", "application/application/geo+json")
        .send();

      if times >= 1 {
        println!("===============================");
      } 

      match resp {
        Ok(response) => {
         let alert = response.json::<Alert>().unwrap();
         println!("{}\n", alert.properties.headline);
         println!("{}\n", alert.properties.description);
         println!("{}\n", alert.properties.instruction);
         println!("{}\n", alert.properties.area_desc);
        },
        Err(_) => println!("ERROR")
      }
     
    }

  }

}

fn main() {

  let args = Args::parse();

  let lat: f64 = args.lat.parse().expect("Error parsing latitude");
  let lon: f64 = args.lon.parse().expect("Error parsing longitude");

  print_alert(lat, lon);


}
