#[macro_use]
extern crate derive_builder;
extern crate tar;

use std::fs;
use std::process::Command;
use tar::Archive;
use serde_json::Value;

#[cfg(test)]
mod tests {
    use crate::{save_image, init_layer_vec};

    #[test]
    fn test_extract() {
        assert_eq!(save_image(), true);
        // TODO: simply check for error -> not for fixed layer id
        let v = init_layer_vec();
        assert_eq!(v.len(), 1)
    }
}

fn save_image() -> bool {
    match Command::new("docker")
        .args(&["save", "ubuntu:bionic-20200112", "-o" , "docker.tar"])
        .status()
        .expect("failed to run 'docker save'")
        .success() {
        false => false,
        true => {
            let mut archive = Archive::new(fs::File::open("docker.tar").expect("Error opening file"));
            archive.unpack("docker").expect("Error to untar");
            true
        }
    }
}

#[derive(Builder)]
struct Layer {
    id: String,
    tar_file_path: String,
    json_str: String,
}

fn init_layer_vec() -> Vec<Layer> {
    let repositories = fs::read_to_string("docker/repositories")
        .expect("Cannot read repositories file");
    let v: Value = serde_json::from_str(repositories.as_str()).expect("Could not parse repositories file");
    let mut l = Vec::new();
    let id = v["ubuntu"]["bionic-20200112"].as_str().unwrap();
    let json_str = fs::read_to_string(format!("docker/{}/json", id))
        .expect("Cannot read repositories file");
    l.push(LayerBuilder::default()
        .id(String::from(id))
        .tar_file_path(String::from(format!("docker/{}/layer.tar", id)))
        .json_str(String::from(json_str))
        .build()
        .unwrap());
    l
}

//fn get_parent_layer(v: Vec<Layer>) -> Vec<Layer> {
//    let current
//    let repositories = fs::read_to_string("docker/")
//        .expect("Cannot read repositories file");
//    let v: Value = serde_json::from_str(repositories.as_str()).expect("Could not parse repositories file");
//    println!("Top layer: {}", v["ubuntu"]["bionic-20200112"].as_str().unwrap());
//}
