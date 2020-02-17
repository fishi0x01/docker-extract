**Status**

[![CircleCI](https://circleci.com/gh/fishi0x01/docker-extract.svg?style=svg)](https://circleci.com/gh/fishi0x01/docker-extract)

# docker-extract

`docker-extract` is a rust library that extracts the filesystem from a docker image. 

Here is an example to extract `alpine:latest` to directory `./docker-fs`:
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
