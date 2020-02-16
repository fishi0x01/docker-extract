#[cfg(test)]
use docker_extract::{untar_layers, save_image, init_layer_vec};
use std::path::Path;

#[test]
fn test_extract() {
    assert_eq!(save_image(), true);
    // TODO: simply check for error -> not for fixed layer id
    let v = init_layer_vec();
    assert_eq!(v.len(), 4);
    untar_layers(v);
    assert_eq!(Path::new("docker/fs/var/lib/dpkg/status").exists(), true);
}
