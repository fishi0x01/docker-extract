
extern crate tar;

use std::process::Command;
use std::fs::File;
use tar::Archive;

#[cfg(test)]
mod tests {
    use crate::save_image;

    #[test]
    fn test_save_image() {
        assert_eq!(save_image(), true);
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
            let mut archive = Archive::new(File::open("docker.tar").expect("Error opening file"));
            archive.unpack("docker").expect("Error to untar");
            true
        }
    }
}
