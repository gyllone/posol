use std::{fs::OpenOptions, path::PathBuf};
use ark_serialize::{CanonicalSerialize, CanonicalDeserialize};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::{from_reader, to_writer_pretty};

pub fn json_from_file<De: DeserializeOwned>(path: &PathBuf) -> De {
    let file = OpenOptions::new()
        .read(true)
        .open(path)
        .unwrap_or_else(|_| panic!("unable to open file {:?}", path));
    from_reader(&file)
        .unwrap_or_else(|_| panic!("unable to parse json file {:?}", path))
}

pub fn json_to_file<Se: Serialize>(se: &Se, path: &PathBuf) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)
        .unwrap_or_else(|_| panic!("unable to open file {:?}", path));
    to_writer_pretty(&mut file, se)
        .unwrap_or_else(|_| panic!("unable to unparse json file {:?}", path))
}

pub fn deser_from_file<De: CanonicalDeserialize>(path: &PathBuf) -> De {
    let file = OpenOptions::new()
        .read(true)
        .open(path)
        .unwrap_or_else(|_| panic!("unable to open file {:?}", path));
    
    CanonicalDeserialize::deserialize_uncompressed(file)
        .unwrap_or_else(|_| panic!("unable to deserialize file {:?}", path))
}

pub fn ser_to_file<Se: CanonicalSerialize>(se: &Se, path: &PathBuf) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)
        .unwrap_or_else(|_| panic!("unable to open file {:?}", path));
    se.serialize_uncompressed(&mut file)
        .unwrap_or_else(|_| panic!("unable to serialize file {:?}", path))
}
