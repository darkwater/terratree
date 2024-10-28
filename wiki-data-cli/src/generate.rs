use std::path::PathBuf;

use wiki_data::item::Item;

pub fn rust(items: &[Item], path: PathBuf) -> anyhow::Result<()> {
    uneval::to_file(items, path)?;

    Ok(())
}

pub fn msgpack(items: &[Item], path: PathBuf) -> anyhow::Result<()> {
    let out = rmp_serde::to_vec(items)?;
    std::fs::write(path, out)?;

    Ok(())
}
