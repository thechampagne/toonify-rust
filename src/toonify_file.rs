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
use std::io::{ErrorKind, Read};
use reqwest::blocking::multipart;
use crate::error::ToonifyError;

pub struct ToonifyFile {
    #[doc(hidden)]
    response: Result<std::string::String, String>
}

#[doc(hidden)]
struct ToonifyFileInternal {
    api_key: String,
    image_file: String
}

impl ToonifyFile {

    /// `image_file` Image file.
    ///
    /// `api_key` Toonify API key.
    pub fn new(image_file: &str, api_key: &str) -> Self {
        let internal = ToonifyFileInternal::new(image_file, api_key);
        let response = internal.http();
        Self {
            response
        }
    }

    #[doc(hidden)]
    fn is_error(&self) -> Option<std::string::String> {
        match self.response.clone() {
            Ok(response) => match json::parse(&response) {
                Ok(json) => if json.has_key("err") {
                    Some(json["err"].to_string())
                } else if json.has_key("status") {
                    Some(json["status"].to_string())
                } else {
                    None
                },
                Err(_) => None
            },
            Err(err) => if err.is_empty() {
                None
            } else {
                Some(err)
            }
        }
    }

    #[doc(hidden)]
    fn json(&self, key: &str) -> Option<std::string::String> {
        match self.response.clone() {
            Ok(response) => match json::parse(&response) {
                Ok(json) => if json.has_key(key) {
                    Some(json[key].to_string())
                } else {
                    None
                },
                Err(_) => None
            },
            Err(_) => None
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

impl ToonifyFileInternal {
    fn new(image_file: &str, api_key: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
            image_file: image_file.to_string()
        }
    }

    fn http(&self) -> Result<std::string::String, String> {
        match multipart::Form::new().file("image", &self.image_file) {
            Ok(form) => match reqwest::blocking::Client::new()
                .post("https://api.deepai.org/api/toonify")
                .header("api-key", self.api_key.clone())
                .multipart(form)
                .send() {
                Ok(mut data) => {
                    let mut body = std::string::String::new();
                    match data.read_to_string(&mut body) {
                        Ok(_) => Ok(body),
                        Err(_) => Err("".to_string())
                    }
                },
                Err(_) => Err("".to_string())
            },
            Err(err) => match err.kind() {
                ErrorKind::NotFound => Err(err.to_string()),
                _ => Err("".to_string())
            }
        }
    }
}