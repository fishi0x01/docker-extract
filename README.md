**Status**

[![CircleCI](https://circleci.com/gh/fishi0x01/docker-extract.svg?style=svg)](https://circleci.com/gh/fishi0x01/docker-extract)

# docker-extract
Work in progress ..

Example extract `alpine:latest` to dir `./docker-fs`:
```rust
use docker_extract;
use std::path::Path;

fn main() {
    let image = "alpine";
    let tag = "latest";
    let to_dir = Path::new("./docker-fs");
    docker_extract::extract_image(image, tag, &to_dir);
}
```
