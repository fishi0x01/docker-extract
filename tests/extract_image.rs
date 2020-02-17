#[cfg(test)]
use docker_extract::extract_image;
use std::path::Path;
use tempdir::TempDir;

#[test]
fn test_extract_ubuntu() {
    let tmp_dir = TempDir::new("docker-extract-test").unwrap();
    let tmp_dir_str = String::from(tmp_dir.path().to_str().unwrap());
    extract_image("ubuntu", "bionic-20200112", Path::new(tmp_dir_str.as_str())).unwrap();
    assert_eq!(
        true,
        Path::new(format!("{}/var/lib/dpkg/status", tmp_dir_str).as_str()).exists()
    );
}

#[test]
fn test_extract_alpine() {
    let tmp_dir = TempDir::new("docker-extract-test").unwrap();
    let tmp_dir_str = String::from(tmp_dir.path().to_str().unwrap());
    extract_image("alpine", "3.11.3", Path::new(tmp_dir_str.as_str())).unwrap();
    assert_eq!(
        true,
        Path::new(format!("{}/etc/apk/world", tmp_dir_str.as_str()).as_str()).exists()
    );
}
