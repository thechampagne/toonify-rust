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
//! Toonify API client
//!
//! Turn a photo of any face into a cartoon instantly with artificial intelligence.
//! Toonify uses a convolutional neural network to quickly transform the photo into a cartoon.
//! While generative adversarial networks (GANs) were used in the creation of Toonify,
//! they are not used in the final model.
mod error;
mod toonify;
mod toonify_file;
pub use toonify::Toonify;
pub use toonify_file::ToonifyFile;
pub use error::ToonifyError;