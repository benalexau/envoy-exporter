use std::error::Error;
use std::collections::HashMap;
use curl::easy::Easy;
use curl::easy::Auth;
use serde_json;
use serde_json::Value;

pub struct EnvoyReader<'a> {
    url: &'a str,
    user: &'a str,
    pass: &'a str,
    status: EnvoyStatus,
}

impl<'a> EnvoyReader<'a> {
    pub fn status(url: &'a str, user: &'a str, pass: &'a str) -> Result<EnvoyStatus, Box<Error>> {
        let mut reader = EnvoyReader {
            url,
            user,
            pass,
            status: EnvoyStatus::new(),
        };
        reader.production()?;
        reader.inverters()?;
        reader.status.online = true;
        Ok(reader.status)
    }

    fn fetch_json(&self, suffix: &str) -> Result<Value, Box<Error>> {
        let url = self.url.to_owned() + suffix;
        let mut auth = Auth::new();
        auth.digest(true);
        let mut handle = Easy::new();
        handle.http_auth(&auth).unwrap();
        handle.username(self.user).unwrap();
        handle.password(self.pass).unwrap();
        handle.url(&url).unwrap();
        let mut data = Vec::new();
        {
            let mut transfer = handle.transfer();
            transfer.write_function(|new_data| {
                data.extend_from_slice(new_data);
                Ok(new_data.len())
            }).unwrap();
            transfer.perform().unwrap();
        }
        let json: Value = serde_json::from_slice(&data)?;
        Ok(json)
    }

    fn production(&mut self) -> Result<(), Box<Error>> {
        let json: Value = self.fetch_json("/api/v1/production")?;
        self.status.watt_hours_lifetime = json["wattHoursLifetime"].as_i64().unwrap();
        self.status.watt_hours_today= json["wattHoursToday"].as_i64().unwrap();
        self.status.watts_now = json["wattsNow"].as_i64().unwrap();
        Ok(())
    }

    fn inverters(&mut self) -> Result<(), Box<Error>> {
        let json: Value = self.fetch_json("/api/v1/production/inverters")?;
        let inverters = json.as_array().unwrap();
        for inverter in inverters {
            let sn = inverter["serialNumber"].as_str().unwrap();
            let watts = inverter["lastReportWatts"].as_i64().unwrap();
            self.status.inverters.insert(sn.to_owned(), watts);
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct EnvoyStatus {
    pub online: bool,
    pub watt_hours_lifetime: i64,
    pub watt_hours_today: i64,
    pub watts_now: i64,
    pub inverters: HashMap<String, i64>,
}

impl EnvoyStatus {
    pub fn new() -> EnvoyStatus {
        EnvoyStatus {
            online: false,
            watt_hours_lifetime: 0,
            watt_hours_today: 0,
            watts_now: 0,
            inverters: HashMap::new(),

        }
    }
}
