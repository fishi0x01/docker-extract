#[cfg(test)]
use docker_extract::extract_image;
use std::path::Path;
use tempdir::TempDir;
use std::io;

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
        Path::new(format!("{}/lib/apk/db/installed", tmp_dir_str.as_str()).as_str()).exists()
    );
}

#[test]
fn test_extract_non_existing() {
    let tmp_dir = TempDir::new("docker-extract-test").unwrap();
    let tmp_dir_str = String::from(tmp_dir.path().to_str().unwrap());
    let result = extract_image("does-not", "exist", Path::new(tmp_dir_str.as_str()));
    assert_eq!(
        result.err().unwrap().to_string(),
        io::Error::new(io::ErrorKind::Other, "Error running 'docker save does-not:exist'").to_string()
    );
}
