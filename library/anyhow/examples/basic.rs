use std::collections::HashMap;
use anyhow::Result;
use std::fs;
use serde_json;

type ClusterMap = HashMap<String, String>;

fn get_cluster_info() -> Result<ClusterMap> {
    let config = fs::read_to_string("cluster.json")?;
    let map: ClusterMap = serde_json::from_str(&config)?;
    Ok(map)
}

fn main() -> Result<()> {
    let cluster_info = get_cluster_info()?;
    println!("Cluster Info: {:?}", cluster_info);
    Ok(())
}
