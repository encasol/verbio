use serde_json::Value as JsonValue;
use crate::fetch::{Fetcher};

const BASE_URL:&str = "https://api.openweathermap.org/data/2.5/";
const API_KEY:&str = "0a4241b2d75af7e616b4a768c58dafc2";


pub trait WeatherRetriever {
    fn get_current(&self, city:String, units:String) -> Result<f64, &str>; 
    fn get_forecast(&self, city:String, units:String) -> Result<f64, &str>;
}

pub struct OpenWeatherRetriever {
    pub ftch: Box<dyn Fetcher>,
}

impl WeatherRetriever for OpenWeatherRetriever {
    fn get_current(&self, city:String, units:String) -> Result<f64, &str> {
        let url = format!("{}weather?q={}&units={}&APPID={}", BASE_URL, city, units, API_KEY);
        let json_str = match self.ftch.get_text(&url) {
            Err(_) => return Err("Bad URL"),
            Ok(f) => f,
        };
        
        let res: JsonValue = match serde_json::from_str(&json_str) {
            Ok(v) => v,
            Err(_) => return Err("Bad Json"),
        };

        match res["main"]["temp"].as_f64() {
            Some(v) => return Ok(v),
            None => return Err("Param error"),
        };
    }

    fn get_forecast(&self, city:String, units:String) -> Result<f64, &str> {
        let url = format!("{}forecast?q={}&units={}&APPID={}", BASE_URL, city, units, API_KEY);
        let json_str = match self.ftch.get_text(&url) {
            Err(_) => return Err("Bad URL"),
            Ok(f) => f,
        };
        
        let res: JsonValue = match serde_json::from_str(&json_str) {
            Ok(v) => v,
            Err(_) => return Err("Bad Json"),
        };

        let list_size = match res["cnt"].as_u64() {
            Some(v) => v - 1,
            None => return Err("Param error"),
        };

        match res["list"][list_size as usize]["main"]["temp"].as_f64() {
            Some(v) => Ok(v),
            None => return Err("Param error"),
        }
    }
}

struct MockFetcherOk {
    to_return: String,
    expected_url: String,
}


impl Fetcher for MockFetcherOk {
    fn get_text(&self, url: &str) -> Result<String, reqwest::Error> {
        assert_eq!(self.expected_url, url.to_string());
        Ok(self.to_return.clone())
    }
}

#[test]
fn test_get_current() {
    let wthr = OpenWeatherRetriever{
        ftch: Box::new(MockFetcherOk{
            to_return: r#"{"main":{"temp":285.26}}"#.to_string(),
            expected_url: r#"https://api.openweathermap.org/data/2.5/weather?q=Barcelona&units=metric&APPID=0a4241b2d75af7e616b4a768c58dafc2"#.to_string()
        }),
    };
    let temp = wthr.get_current("Barcelona".to_string(), "metric".to_string()).unwrap();

    assert_eq!(temp, 285.26);
}

#[test]
fn test_get_current_bad_json() {
    let wthr = OpenWeatherRetriever{
        ftch: Box::new(MockFetcherOk{
            to_return: r#"{"main":"testBad"}"#.to_string(),
            expected_url: r#"https://api.openweathermap.org/data/2.5/weather?q=Barcelona&units=metric&APPID=0a4241b2d75af7e616b4a768c58dafc2"#.to_string()
        }),
    };
    let temp = wthr.get_current("Barcelona".to_string(), "metric".to_string());

    assert!(temp.is_err());
}

#[test]
fn test_get_forecast() {
    let wthr = OpenWeatherRetriever{
        ftch: Box::new(MockFetcherOk{
            to_return: r#"{ 
                "cnt":1,
                "list":[ 
                   {"main":{"temp":285.26}}
                ]
             }"#.to_string(),
            expected_url: r#"https://api.openweathermap.org/data/2.5/forecast?q=Barcelona&units=metric&APPID=0a4241b2d75af7e616b4a768c58dafc2"#.to_string()
        }),
    };
    let temp = wthr.get_forecast("Barcelona".to_string(), "metric".to_string()).unwrap();

    assert_eq!(temp, 285.26);
}

#[test]
fn test_get_forecast_more_in_list() {
    let wthr = OpenWeatherRetriever{
        ftch: Box::new(MockFetcherOk{
            to_return: r#"{ 
                "cnt":2,
                "list":[ 
                    {"main":{"temp":1}},
                    {"main":{"temp":2}}
                ]
             }"#.to_string(),
            expected_url: r#"https://api.openweathermap.org/data/2.5/forecast?q=Barcelona&units=metric&APPID=0a4241b2d75af7e616b4a768c58dafc2"#.to_string()
        }),
    };
    let temp = wthr.get_forecast("Barcelona".to_string(), "metric".to_string()).unwrap();

    assert_eq!(temp, 2.0);
}

#[test]
fn test_get_forecast_bad_json() {
    let wthr = OpenWeatherRetriever{
        ftch: Box::new(MockFetcherOk{
            to_return: r#"{"main":"testBad"}"#.to_string(),
            expected_url: r#"https://api.openweathermap.org/data/2.5/forecast?q=Barcelona&units=metric&APPID=0a4241b2d75af7e616b4a768c58dafc2"#.to_string()
        }),
    };
    let temp = wthr.get_forecast("Barcelona".to_string(), "metric".to_string());

    assert!(temp.is_err());
}

