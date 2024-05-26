use std::cmp::min;
use std::error::Error;
use open_meteo_api::query::OpenMeteo;
use open_meteo_api::models::{OpenMeteoData};
use tokio::runtime::Runtime;
use std::ffi::CString;
use serde_json::Value;

enum Location {
    City(String),
    Coordinates(f32, f32),
}

fn parse_location(location: &str) -> Result<Location, Box<dyn Error>> {
    let parts: Vec<&str> = location.split(',').collect();
    if parts.len() == 2 {
        let lat = parts[0].parse();
        let lon = parts[1].parse();
        match (lat, lon) {
            (Ok(lat), Ok(lon)) => Ok(Location::Coordinates(lat, lon)),
            _ => Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid coordinates"))),
        } 
    } else {
        Ok(Location::City(location.to_string()))
    }
}

#[no_mangle]
pub extern "C" fn weather(location: *const i8, response: *mut u8, response_buffer_size: usize) {
    let location_str = unsafe {
        std::ffi::CStr::from_ptr(location).to_str().unwrap()
    };
    let location_enum = parse_location(location_str);
    let rt = Runtime::new().unwrap();
    let meteo_data: Option<OpenMeteoData> = match location_enum {
        Ok(location) => {
            let fetched_data: Result<OpenMeteoData, Box<dyn Error>> = rt.block_on(async {
                fetch_weather(location).await
            });
            match fetched_data {
                Ok(data) => Some(data),
                Err(_) => None,
            }
        }
        Err(_) => None,
    };

    let temperature = match meteo_data.as_ref() {
        Some(data) => data.current_weather.as_ref().unwrap().temperature.to_string(),
        None => "Some error occurred.".to_string()
    };
    match meteo_data.as_ref() {
        Some(data) => {
            println!("data: {:?}", data);
        }
        None => println!("Error: could not fetch weather data for the location"),
    }

    let meteo_data_str = format!("{:?}", temperature);
    let c_string = CString::new(meteo_data_str).unwrap();
    unsafe {
        std::ptr::copy_nonoverlapping(
            c_string.as_bytes_with_nul().as_ptr(),
            response as *mut u8,
            min(c_string.to_bytes_with_nul().len(), response_buffer_size),
        );
    }
}

async fn fetch_weather(location_enum: Location) -> Result<OpenMeteoData, Box<dyn Error>> {
    let open_meteo = OpenMeteo::new();
    let open_meteo = match location_enum {
            Location::City(city) => {
                let lat_lon = get_coordinates(&city).await;
                match lat_lon {
                    Ok(Location::Coordinates(lat, lon)) => Ok(open_meteo.coordinates(lat, lon)),
                    _ => Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid coordinates"))),
                }
            }
            Location::Coordinates(lat, lon) => Ok(open_meteo.coordinates(lat, lon)),
    };
    let open_meteo = match open_meteo {
        Ok(om) => Ok(om.unwrap().current_weather()),
        Err(_) => Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid coordinates"))),
    };
    
    let data = match open_meteo {
        Ok(om) => om?.query().await,
        Err(_) => Err("Failed to fetch weather data".into()),
    };

    Ok(data?)
}

async fn get_coordinates(place_name: &str) -> Result<Location, Box<dyn Error>> {
    let url = format!("https://geocode.maps.co/search?q={}", place_name);
    let response = reqwest::get(url).await?.text().await?;

    let json: Value = match serde_json::from_str(&response) {
        Ok(json) => json,
        Err(_) => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid place name"))),
    };

    let (lat, lon) = 
        (json[0]["lat"].as_str().unwrap_or("failed")
            .parse::<f32>(),
         json[0]["lon"].as_str().unwrap_or("failed")
            .parse::<f32>(),);

    let result = match (lat, lon) {
        (Ok(lat), Ok(lon)) => Ok(Location::Coordinates(lat, lon)),
        _ => Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid place name"))),
    };

    Ok(result?)
}


