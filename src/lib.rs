#[macro_use]
extern crate derive_builder;
extern crate tar;

use serde_json::Value;
use std::io::{Error, ErrorKind};
use std::path::Path;
use std::process::Command;
use std::{fs, io};
use tar::Archive;
use tempdir::TempDir;

/// Extract filesystem of the given docker image (`{image}:{tag}`) to the given path `to_dir`.
/// Does not extract symlinks to absolute paths, as they will point to wrong references anyways.
///
/// **Example:**
/// ```rust
///use std::path::Path;
///use std::io;
///# use tempdir::TempDir;
///
///fn main() -> io::Result<()>{
///    let image = "alpine";
///    let tag = "3.11.3";
///#    let tmp_dir = TempDir::new("docker-extract-docu").expect("");
///#    let dir_string = String::from(tmp_dir.path().to_str().unwrap());
///    let to_dir = Path::new(dir_string.as_str());
///    docker_extract::extract_image(image, tag, &to_dir)
///}
/// ```
pub fn extract_image(image: &str, tag: &str, to_dir: &Path) -> io::Result<()> {
    let tmp_dir = TempDir::new("docker-extract")?;
    let tmp_dir_str = String::from(tmp_dir.path().to_str().unwrap());
    save_image(image, tag, tmp_dir_str.as_str())?;
    untar_layers(get_layers(image, tag, tmp_dir_str.as_str())?, to_dir)?;
    Ok(())
}

#[derive(Builder)]
struct Layer {
    tar_file_path: String,
    meta_json_str: String,
}

impl Layer {
    fn get_tar_file_path(&self) -> &str {
        self.tar_file_path.as_str()
    }

    fn get_meta_json_str(&self) -> &str {
        self.meta_json_str.as_str()
    }
}

fn save_image(image: &str, tag: &str, to_dir: &str) -> io::Result<()> {
    let tar_file = String::from(format!("{}/image.tar", to_dir));
    let unpacked_dir = String::from(format!("{}/image", to_dir));
    match Command::new("docker")
        .args(&[
            "save",
            format!("{}:{}", image, tag).as_str(),
            "-o",
            tar_file.as_str(),
        ])
        .status()
    {
        Err(e) => Err(e),
        Ok(f) => {
            if !f.success() {
                Err(Error::new(
                    ErrorKind::Other,
                    format!("Error running 'docker save {}:{}'", image, tag).as_str(),
                ))
            } else {
                Archive::new(fs::File::open(tar_file.as_str())?).unpack(unpacked_dir.as_str())?;
                Ok(())
            }
        }
    }
}

fn get_layers(image: &str, tag: &str, tmp_dir: &str) -> io::Result<Vec<Layer>> {
    let repositories = fs::read_to_string(format!("{}/image/repositories", tmp_dir))?;
    let j: Value = serde_json::from_str(repositories.as_str())?;
    let id = j[image][tag].as_str().unwrap_or("");
    let json_str = fs::read_to_string(format!("{}/image/{}/json", tmp_dir, id))?;
    Ok(get_parent_layer(
        vec![LayerBuilder::default()
            .tar_file_path(String::from(format!("{}/image/{}/layer.tar", tmp_dir, id)))
            .meta_json_str(String::from(json_str))
            .build()
            .unwrap()],
        tmp_dir,
    ))
}

fn get_parent_layer(mut v: Vec<Layer>, tmp_dir: &str) -> Vec<Layer> {
    let meta: Value = serde_json::from_str(v.last().unwrap().get_meta_json_str()).unwrap();
    if meta["parent"].is_string() {
        let parent_id = meta["parent"].as_str().unwrap_or("");
        let parent_meta_json =
            fs::read_to_string(format!("{}/image/{}/json", tmp_dir, parent_id)).unwrap();
        v.push(
            LayerBuilder::default()
                .tar_file_path(String::from(format!(
                    "{}/image/{}/layer.tar",
                    tmp_dir, parent_id
                )))
                .meta_json_str(String::from(parent_meta_json))
                .build()
                .unwrap(),
        );
        v = get_parent_layer(v, tmp_dir);
    }
    v
}

fn untar_layers(v: Vec<Layer>, dst: &Path) -> io::Result<()> {
    for l in v.iter().rev() {
        let mut archive = Archive::new(fs::File::open(l.get_tar_file_path())?);
        for file_raw in archive.entries().unwrap() {
            let mut do_unpack = true;
            let mut file = file_raw.unwrap();
            if file.header().entry_type().is_symlink() {
                let symlink = String::from(file
                    .header()
                    .link_name()
                    .unwrap()
                    .unwrap()
                    .display()
                    .to_string());
                if symlink.starts_with("/") {
                    // Absolute symlinks will point to wrong destinations
                    // TODO: make absolute symlink relative to {dst} instead
                    do_unpack = false;
                }
            }
            if do_unpack {
                match file.unpack_in(dst.display().to_string()) {
                    Err(e) => {
                        println!("{}", e);
                        ()
                    }
                    Ok(_) => (),
                }
            }
        }
    }
    Ok(())
}
