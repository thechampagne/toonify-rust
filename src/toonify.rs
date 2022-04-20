/*
 * Copyright 2022 XXIV
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
use std::collections::HashMap;
use std::io::Read;
use crate::error::ToonifyError;

pub struct Toonify {
    #[doc(hidden)]
    response: Option<std::string::String>
}

#[doc(hidden)]
struct ToonifyInternal {
    api_key: String,
    params: HashMap<String, String>
}

impl Toonify {

    /// `image_uri` Image URI.
    ///
    /// `api_key` Toonify API key.
    pub fn new(image_uri: &str, api_key: &str) -> Self {
        let internal = ToonifyInternal::new(image_uri, api_key);
        let response = internal.http();
        Self {
            response
        }
    }

    #[doc(hidden)]
    fn is_error(&self) -> Option<std::string::String> {
        match self.response.clone() {
            Some(response) => match json::parse(&response) {
                Ok(json) => if json.has_key("err") {
                    Some(json["err"].to_string())
                } else if json.has_key("status") {
                    Some(json["status"].to_string())
                } else {
                    None
                },
                Err(_) => None
            },
            None => None
        }
    }

    #[doc(hidden)]
    fn json(&self, key: &str) -> Option<std::string::String> {
        match self.response.clone() {
            Some(response) => match json::parse(&response) {
                Ok(json) => if json.has_key(key) {
                    Some(json[key].to_string())
                } else {
                    None
                },
                Err(_) => None
            },
            None => None
        }
    }

    /// Return image URI.
    pub fn image(&self) -> Result<std::string::String, ToonifyError> {
        match self.is_error() {
            Some(error) => Err(ToonifyError::Error(error)),
            None => match self.json("output_url") {
                Some(image) => Ok(image),
                None => Err(ToonifyError::Null(String::from("null")))
            }
        }
    }

    /// Return image id.
    pub fn id(&self) -> Result<std::string::String, ToonifyError> {
        match self.is_error() {
            Some(error) => Err(ToonifyError::Error(error)),
            None => match self.json("id") {
                Some(id) => Ok(id),
                None => Err(ToonifyError::Null(String::from("null")))
            }
        }
    }
}

impl ToonifyInternal {
    fn new(image_uri: &str, api_key: &str) -> Self {
        let mut params = HashMap::new();
        params.insert("image".to_string(), image_uri.to_string());
        Self {
            api_key: api_key.to_string(),
            params
        }
    }

    fn http(&self) -> Option<std::string::String> {
        match reqwest::blocking::Client::new().post("https://api.deepai.org/api/toonify")
            .header("api-key", self.api_key.clone())
            .form(&self.params)
            .send() {
            Ok(mut data) => {
                let mut body = String::new();
                match data.read_to_string(&mut body) {
                    Ok(_) => Some(body),
                    Err(_) => None
                }
            },
            Err(_) => None
        }
    }
}