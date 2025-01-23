use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, path::PathBuf};
use tokio::fs;

pub async fn read_nix_store() -> std::io::Result<fs::ReadDir> {
    fs::read_dir("/nix/store").await
}

pub async fn read_nix_store_handled() -> HashMap<usize, Vec<PathBuf>> {
    match read_nix_store().await {
        Ok(mut dir) => {
            // Collect directory entries of 50 items for pageination
            let mut store_items = HashMap::new();
            let mut i = 0;

            while let Some(entry) = dir.next_entry().await.unwrap() {
                store_items
                    .entry(i / 50)
                    .or_insert_with(Vec::new)
                    .push(entry.path());
                i += 1;
            }

            store_items
        }
        Err(e) => {
            eprintln!("Error reading nix store: {}", e);
            HashMap::new()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PathInfo {
    // consistant attributes
    pub path: String,
    pub valid: bool,

    pub ca: Option<String>,
    pub deriver: Option<String>,
    pub nar_hash: Option<String>,
    pub nar_size: Option<isize>,
    pub references: Option<Vec<String>>,
    pub registration_time: Option<isize>,
    pub signatures: Option<Vec<String>>,
    pub ultimate: Option<bool>,

    // --store flag attributes
    download_hash: Option<String>,
    download_size: Option<isize>,
    url: Option<String>,

    // custom attributes
    cached_in: Option<Vec<String>>,
}

pub type PathInfos = Vec<PathInfo>;

pub async fn parse_store_item(
    store_path: &PathBuf,
) -> Result<PathInfo, Box<dyn std::error::Error>> {
    let output = std::process::Command::new("nix")
        .arg("path-info")
        .arg(store_path)
        .arg("--json")
        .output()?;

    let v: Value = serde_json::from_slice(&output.stdout)?;
    let path_infos: PathInfos = serde_json::from_value(v)?;

    let path_info_pre_caches = path_infos.first().unwrap().clone();
    let path_info = is_nix_cached(path_info_pre_caches).await?;

    Ok(path_info)
}

fn get_substituters() -> Vec<String> {
    std::process::Command::new("nix")
        .args(["config", "show", "substituters"])
        .output()
        .unwrap()
        .stdout
        .split(|&byte| byte == b' ')
        .map(|line| String::from_utf8(line.to_vec()).unwrap().trim().to_string())
        .collect()
}

pub async fn is_cached_in(
    store_path: &str,
    substituter: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    let output = std::process::Command::new("nix")
        .arg("path-info")
        .arg(store_path)
        .arg("--json")
        .arg("--store")
        .arg(substituter)
        .output()?;

    let v: Value = serde_json::from_slice(&output.stdout)?;
    let path_infos: PathInfos = serde_json::from_value(v)?;

    let path_info = path_infos.first().unwrap();

    Ok(path_info.valid)
}

pub async fn is_nix_cached(path_info: PathInfo) -> Result<PathInfo, Box<dyn std::error::Error>> {
    let substituters = get_substituters();

    let mut pi = path_info;

    if pi.cached_in.is_none() {
        pi.cached_in = Some(Vec::new());
    }

    for substituter in substituters {
        if is_cached_in(&pi.path, &substituter).await? {
            pi.cached_in.as_mut().unwrap().push(substituter);
        };
    }

    Ok(pi)
}
