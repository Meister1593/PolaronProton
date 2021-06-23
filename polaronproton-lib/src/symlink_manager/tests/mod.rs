use crate::symlink_manager::*;
use std::matches;
use std::{fs, io, u32};

#[test]
pub fn link_test() {
    let appid_1: u32 = 0;
    let appid_2: u32 = 1;

    create_appid_pfx_path(appid_1).unwrap();

    link_appids(appid_1, appid_2).unwrap();

    let entries = fs::read_dir(format!(
        "{}/{}",
        get_steam_compatdata_path().unwrap(),
        appid_2
    ))
    .unwrap()
    .map(|res| res.map(|e| e.path()))
    .collect::<Result<Vec<_>, io::Error>>()
    .unwrap();

    let appid_2_pfx_path_pos = entries.iter().position(|entry| entry.ends_with("pfx"));
    assert!(matches!(appid_2_pfx_path_pos, Some(_)));
    remove_appid_path(appid_1).unwrap();
    remove_appid_path(appid_2).unwrap();
}
