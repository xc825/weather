use std::error::Error;
use open_meteo_api::query::OpenMeteo;
use open_meteo_api::models::{OpenMeteoData,TimeZone};
use tokio::runtime::Runtime;

#[no_mangle]
pub extern "C" fn rust_function() {
    println!("Hello from Rust!!!");

    // Create a new Tokio runtime
    let rt = Runtime::new().unwrap();

    // Use the runtime to block on the async function
    rt.block_on(async {
        let data1 = example().await.unwrap();
        println!("sunstet: {:?}", data1.daily.unwrap().sunset);
    });

    println!("After Example()");
}

async fn example() -> Result<OpenMeteoData, Box<dyn Error>> {

    println!("Hello from Example!!!");
    // parsed json with (almost) all data you may need
    // for more info see open-meteo.com/en/docs

    let data1 = OpenMeteo::new() 
            .location("Riga").await? // add location
            .forecast_days(10)?  // add forecast data
            .current_weather()?  // add current weather data
            .past_days(10)? // add past days data
            .time_zone(TimeZone::Auto)? // set time zone for using .daily()
            .hourly()? // add hourly weather data
            .daily()? // add daily weather data
            .query()
            .await?;

    // using start date and end date

    let data2 = OpenMeteo::new()
            .coordinates(51.0, 0.0)? // you can also use .coordinates(lat, lon) to set location
            .start_date("2024-05-22")?
            .end_date("2024-05-28")?
            .time_zone(TimeZone::Auto)?
            .hourly()?
            .daily()?
            .query()
            .await?;

    // accessing data fields
    // current_weather, hourly_units, hourly, daily_units, daily have Option type
    // fields of ".hourly" and ".daily" have Vec<Option<T>> type
    
    //let temperature = data1.current_weather.unwrap().temperature;
    //let temperature_2m = data2.hourly.unwrap().temperature_2m;

    //println!("temperature:{}", temperature );
    //println!("temparature_2m:{:?}", temperature_2m);
    //println!("forecast:{:?}", data1.daily.unwrap());
    //println!("sunset:{:?}", data1.daily.unwrap().sunset);
        
    Ok(data1)
}
