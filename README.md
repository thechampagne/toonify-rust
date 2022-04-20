# Toonify

[![](https://img.shields.io/github/v/tag/thechampagne/toonify-rust?label=version)](https://github.com/thechampagne/toonify-rust/releases/latest) [![](https://img.shields.io/github/license/thechampagne/toonify-rust)](https://github.com/thechampagne/toonify-rust/blob/main/LICENSE)

Turn a photo of any face into a cartoon instantly with artificial intelligence. Toonify uses a convolutional neural network to quickly transform the photo into a cartoon. While generative adversarial networks (GANs) were used in the creation of Toonify, they are not used in the final model.

### Download
[Crates](https://crates.io/crates/toonify/)

Add the following line to your Cargo.toml file:

```
toonify = "1.0.0"
```

### Example

```rust
use toonify::Toonify;

fn main() {
    let toon = Toonify::new("http://image-url.example",
    "API-key");
    println!("{}", toon.image().unwrap())
}
```

### License

Toonify is released under the [Apache License 2.0](https://github.com/thechampagne/toonify-rust/blob/main/LICENSE).

```
 Copyright 2022 XXIV

 Licensed under the Apache License, Version 2.0 (the "License");
 you may not use this file except in compliance with the License.
 You may obtain a copy of the License at

     http://www.apache.org/licenses/LICENSE-2.0

 Unless required by applicable law or agreed to in writing, software
 distributed under the License is distributed on an "AS IS" BASIS,
 WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 See the License for the specific language governing permissions and
 limitations under the License.
```