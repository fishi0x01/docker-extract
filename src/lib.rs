#[macro_use]
extern crate derive_builder;
extern crate tar;

use std::fs;
use std::process::Command;
use tar::Archive;
use serde_json::Value;

pub fn save_image() -> bool {
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
pub struct Layer {
    id: String,
    tar_file_path: String,
    meta_json_str: String,
}

impl Layer {
    fn get_id(&self) -> &str {
        self.id.as_str()
    }

    fn get_tar_file_path(&self) -> &str {
        self.tar_file_path.as_str()
    }

    fn get_meta_json_str(&self) -> &str {
        self.meta_json_str.as_str()
    }
}

pub fn init_layer_vec() -> Vec<Layer> {
    let repositories = fs::read_to_string("docker/repositories")
        .expect("Cannot read repositories file");
    let j: Value = serde_json::from_str(repositories.as_str()).expect("Could not parse repositories file");
    let id = j["ubuntu"]["bionic-20200112"].as_str().unwrap();
    let json_str = fs::read_to_string(format!("docker/{}/json", id))
        .expect("Cannot read repositories file");
    get_parent_layer(vec![LayerBuilder::default()
        .id(String::from(id))
        .tar_file_path(String::from(format!("docker/{}/layer.tar", id)))
        .meta_json_str(String::from(json_str))
        .build()
        .unwrap()])
}

fn get_parent_layer(mut v: Vec<Layer>) -> Vec<Layer> {
    let meta: Value = serde_json::from_str(v.last().unwrap().get_meta_json_str())
        .expect("Could not parse meta json file");
    if meta["parent"].is_string() {
        let parent_id = meta["parent"].as_str().unwrap();
        let parent_meta_json = fs::read_to_string(format!("docker/{}/json", parent_id))
            .expect("Cannot read meta json file");
        v.push(LayerBuilder::default()
            .id(String::from(parent_id))
            .tar_file_path(String::from(format!("docker/{}/layer.tar", parent_id)))
            .meta_json_str(String::from(parent_meta_json))
            .build()
            .unwrap());
        v = get_parent_layer(v);
    }
    v
}

pub fn untar_layers(v: Vec<Layer>) {
    for l in v.iter().rev() {
        println!("{}", l.get_id());
        let mut archive = Archive::new(fs::File::open(l.get_tar_file_path()).expect("Error opening tar layer"));
        archive.unpack("docker/fs").expect("Error to untar");
    }
}
