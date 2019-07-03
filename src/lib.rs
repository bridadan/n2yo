extern crate reqwest;
extern crate serde;
extern crate serde_json;

use std::error;
use std::fmt;

pub struct Client {
    api_key: String,
    base_url: String,
}

#[derive(Debug)]
pub enum Error {
	Request(reqwest::Error),
	Response(reqwest::Response),
	Parse(reqwest::Error),
}

// Implement std::fmt::Display for Error
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An error occurred when making the request to N2yo.")
    }
}

// This is important for other errors to wrap this one.
impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct Info {
    pub satid: u32,
    pub satname: String,
    pub transactionscount: u32,
}
	
#[derive(serde::Deserialize, Debug)]
pub struct TleResponse {
	pub info: Info,
    pub tle: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct SatellitePosition {
	pub satlatitude: f64,
	pub satlongitude: f64,
	pub sataltitude: f64,
	pub azimuth: f64,
	pub elevation: f64,
	pub ra: f64,
	pub dec: f64,
	pub timestamp: u32,
}

#[derive(serde::Deserialize, Debug)]
pub struct PositionsResponse {
	pub info: Info,
    pub positions: Vec<SatellitePosition>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SatelliteVisualPass {
	pub start_az: f64,
    pub start_az_compass: String,
    pub start_el: f64,
    #[serde(rename = "startUTC")]
    pub start_utc: u32,
    pub max_az: f64,
    pub max_az_compass: String,
    pub max_el: f64,
    #[serde(rename = "maxUTC")]
    pub max_utc: u32,
    pub end_az: f64,
    pub end_az_compass: String,
    pub end_el: f64,
    #[serde(rename = "endUTC")]
    pub end_utc: u32,
    pub mag: f64,
    pub duration: u32,
}

#[derive(serde::Deserialize, Debug)]
pub struct VisualPassesResponse {
	pub info: Info,
    pub passes: Vec<SatelliteVisualPass>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SatelliteRadioPass {
	pub start_az: f64,
    pub start_az_compass: String,
    #[serde(rename = "startUTC")]
    pub start_utc: u32,
    pub max_az: f64,
    pub max_az_compass: String,
    pub max_el: f64,
    #[serde(rename = "maxUTC")]
    pub max_utc: u32,
    pub end_az: f64,
    pub end_az_compass: String,
    #[serde(rename = "endUTC")]
    pub end_utc: u32,
}

#[derive(serde::Deserialize, Debug)]
pub struct RadioPassesResponse {
	pub info: Info,
    pub passes: Vec<SatelliteRadioPass>,
}

const DEFAULT_BASE_URL : &str = "https://www.n2yo.com/rest/v1";


impl Client {
    pub fn new(api_key: &str) -> Client {
        Client {
            api_key: String::from(api_key),
            base_url: String::from(DEFAULT_BASE_URL),
        }
    }

    pub fn new_with_base_url(api_key: &str, base_url: &str) -> Client {
        Client {
            api_key: String::from(api_key),
            base_url: String::from(base_url),
        }
    }

    fn form_request_url(&self, url_part: &str) -> String {
        return format!("{}{}&apiKey={}", self.base_url, url_part, self.api_key);
    }

    pub fn tle(&self, id: u32) -> Result<TleResponse, Error> {
        let url_part = format!("/satellite/tle/{}", id);
        let url = self.form_request_url(url_part.as_str());
		let mut response = reqwest::get(url.as_str()).map_err(Error::Request)?;
        if response.status() != reqwest::StatusCode::OK {
			return Err(Error::Response(response));
		}
		Ok(response.json().map_err(Error::Parse)?)
    }

	pub fn positions(
		&self,
		id: u32,
		observer_lat: f64,
		observer_lng: f64,
		observer_alt: f64,
		seconds: u32
	) -> Result<PositionsResponse, Error> {
        let url_part = format!(
			"/satellite/positions/{}/{}/{}/{}/{}",
			id,
			observer_lat,
			observer_lng,
			observer_alt,
			seconds
		);
        let url = self.form_request_url(url_part.as_str());
		let mut response = reqwest::get(url.as_str()).map_err(Error::Request)?;
        if response.status() != reqwest::StatusCode::OK {
			return Err(Error::Response(response));
		}
		Ok(response.json().map_err(Error::Parse)?)
    }

	pub fn visual_passes(
		&self,
		id: u32,
		observer_lat: f64,
		observer_lng: f64,
		observer_alt: f64,
		days: u32,
        min_visibility: u32
	) -> Result<VisualPassesResponse, Error> {
        let url_part = format!(
			"/satellite/visualpasses/{}/{}/{}/{}/{}/{}",
			id,
			observer_lat,
			observer_lng,
			observer_alt,
			days,
            min_visibility
		);
        let url = self.form_request_url(url_part.as_str());
		let mut response = reqwest::get(url.as_str()).map_err(Error::Request)?;
        if response.status() != reqwest::StatusCode::OK {
			return Err(Error::Response(response));
		}
		Ok(response.json().map_err(Error::Parse)?)
    }

	pub fn radio_passes(
		&self,
		id: u32,
		observer_lat: f64,
		observer_lng: f64,
		observer_alt: f64,
		days: u32,
        min_elevation: u32
	) -> Result<RadioPassesResponse, Error> {
        let url_part = format!(
			"/satellite/radiopasses/{}/{}/{}/{}/{}/{}",
			id,
			observer_lat,
			observer_lng,
			observer_alt,
			days,
            min_elevation
		);
        let url = self.form_request_url(url_part.as_str());
		let mut response = reqwest::get(url.as_str()).map_err(Error::Request)?;
        if response.status() != reqwest::StatusCode::OK {
			return Err(Error::Response(response));
		}
		Ok(response.json().map_err(Error::Parse)?)
    }
}

#[cfg(test)]
mod tests {
	extern crate mockito;
    use mockito::mock;
    use super::*;
    use std::path::PathBuf;

	fn get_mock_result_path(mock_result_name: &str) -> String {
		let mut mock_result_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
		mock_result_dir.push("mock_results");
		mock_result_dir.push(format!("{}.json", mock_result_name));
		String::from(mock_result_dir.to_str().unwrap())
	}

    #[test]
    fn normal_constructor() {
        const KEY : &str = "dummy_key";
        let client : Client = Client::new(KEY);
        assert_eq!(client.api_key, KEY);
        assert_eq!(client.base_url, DEFAULT_BASE_URL);
    }
    
    #[test]
    fn base_url_constructor() {
        const KEY : &str = "dummy_key";
        const URL : &str = "dummy_url";
        let client : Client = Client::new_with_base_url(KEY, URL);
        assert_eq!(client.api_key, KEY);
        assert_eq!(client.base_url, URL);
    }

    #[test]
    fn form_request_url() {
        const KEY : &str = "dummy_key";
        const URL : &str = "dummy_url";
        let client : Client = Client::new_with_base_url(KEY, URL);
        assert_eq!(client.form_request_url("/test"), "dummy_url/test&apiKey=dummy_key");
    }

	#[test]
	fn tle() {
        const KEY : &str = "dummy_key";
		const SATID: u32 = 25544;
		let path = format!("/satellite/tle/{}&apiKey={}", SATID, KEY);
        let _m = mock("GET", path.as_str())
            .with_status(200)
            .with_body_from_file(get_mock_result_path("tle").as_str())
			.create();
        let client : Client = Client::new_with_base_url(KEY, &mockito::server_url());
		let response = client.tle(SATID).unwrap();
		assert_eq!(response.info.satid, SATID);
		assert_eq!(response.info.satname, "SPACE STATION");
		assert!(response.tle.contains("\r\n"));
	}

	#[test]
	fn positions() {
        const KEY : &str = "dummy_key";
		const SATID: u32 = 25544;
		const OBSERVER_LAT: f64 = 41.702;
		const OBSERVER_LNG: f64 = -76.014;
		const OBSERVER_ALT: f64 = 0.0;
		const SECONDS: u32 = 2;
		let path = format!(
			"/satellite/positions/{}/{}/{}/{}/{}&apiKey={}",
			SATID,
			OBSERVER_LAT,
			OBSERVER_LNG,
			OBSERVER_ALT,
			SECONDS,
			KEY
		);
        let _m = mock("GET", path.as_str())
            .with_status(200)
            .with_body_from_file(get_mock_result_path("positions").as_str())
			.create();
        let client : Client = Client::new_with_base_url(KEY, &mockito::server_url());
		let response = client.positions(
			SATID,
			OBSERVER_LAT,
			OBSERVER_LNG,
			OBSERVER_ALT,
			SECONDS,
		).unwrap();
		assert_eq!(response.info.satid, SATID);
		assert_eq!(response.info.satname, "SPACE STATION");
        assert_eq!(response.positions.len(), 2);
	}

    #[test]
	fn visual_passes() {
        const KEY : &str = "dummy_key";
		const SATID: u32 = 25544;
		const OBSERVER_LAT: f64 = 41.702;
		const OBSERVER_LNG: f64 = -76.014;
		const OBSERVER_ALT: f64 = 0.0;
		const DAYS: u32 = 2;
        const MIN_VISIBILITY: u32 = 300;
		let path = format!(
			"/satellite/visualpasses/{}/{}/{}/{}/{}/{}&apiKey={}",
			SATID,
			OBSERVER_LAT,
			OBSERVER_LNG,
			OBSERVER_ALT,
			DAYS,
            MIN_VISIBILITY,
			KEY
		);
        let _m = mock("GET", path.as_str())
            .with_status(200)
            .with_body_from_file(get_mock_result_path("visualpasses").as_str())
			.create();
        let client : Client = Client::new_with_base_url(KEY, &mockito::server_url());
		let response = client.visual_passes(
			SATID,
			OBSERVER_LAT,
			OBSERVER_LNG,
			OBSERVER_ALT,
			DAYS,
            MIN_VISIBILITY
		).unwrap();
		assert_eq!(response.info.satid, SATID);
		assert_eq!(response.info.satname, "SPACE STATION");
        assert_eq!(response.passes.len(), 3);
	}

    #[test]
	fn radio_passes() {
        const KEY : &str = "dummy_key";
		const SATID: u32 = 25544;
		const OBSERVER_LAT: f64 = 41.702;
		const OBSERVER_LNG: f64 = -76.014;
		const OBSERVER_ALT: f64 = 0.0;
		const DAYS: u32 = 2;
        const MIN_ELEVATION: u32 = 40;
		let path = format!(
			"/satellite/radiopasses/{}/{}/{}/{}/{}/{}&apiKey={}",
			SATID,
			OBSERVER_LAT,
			OBSERVER_LNG,
			OBSERVER_ALT,
			DAYS,
            MIN_ELEVATION,
			KEY
		);
        let _m = mock("GET", path.as_str())
            .with_status(200)
            .with_body_from_file(get_mock_result_path("radiopasses").as_str())
			.create();
        let client : Client = Client::new_with_base_url(KEY, &mockito::server_url());
		let response = client.radio_passes(
			SATID,
			OBSERVER_LAT,
			OBSERVER_LNG,
			OBSERVER_ALT,
			DAYS,
            MIN_ELEVATION
		).unwrap();
		assert_eq!(response.info.satid, SATID);
		assert_eq!(response.info.satname, "SPACE STATION");
        assert_eq!(response.passes.len(), 2);
	}
}
