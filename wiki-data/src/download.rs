use anyhow::Context;
use indicatif::ProgressStyle;
use serde::Deserialize;
use tracing::{info_span, Span};
use tracing_indicatif::span_ext::IndicatifSpanExt as _;
use url::Url;

use wiki_data::item::raw::RawItem;

#[derive(Debug, Default, Deserialize)]
pub struct CargoQuery<T> {
    pub cargoquery: Vec<CargoQueryEntry<T>>,
}

#[derive(Debug, Default, Deserialize)]
pub struct CargoQueryEntry<T> {
    pub title: T,
}

#[derive(Debug, Default, Deserialize)]
pub struct Count {
    #[serde(rename = "count(*)")]
    pub count: String,
}

#[tracing::instrument(fields(indicatif.pb_show))]
pub async fn count() -> anyhow::Result<usize> {
    let url = Url::parse_with_params(
        "https://terraria.wiki.gg/api.php",
        [
            ("action", "cargoquery"),
            ("tables", "Items"),
            ("format", "json"),
            ("limit", "max"),
            ("fields", "count(*)"),
        ],
    )
    .unwrap();

    surf::get(url)
        .await
        .map_err(|e| e.into_inner())?
        .body_json::<CargoQuery<Count>>()
        .await
        .map_err(|e| e.into_inner())?
        .cargoquery
        .into_iter()
        .next()
        .context("no count returned")?
        .title
        .count
        .parse()
        .context("failed to parse count")
    // .map(|v: usize| v / 10)
}

#[tracing::instrument(fields(indicatif.pb_show))]
pub async fn items(offset: usize) -> anyhow::Result<Vec<RawItem>> {
    tracing::info!("fetching items from offset {}", offset);

    let url = Url::parse_with_params(
        "https://terraria.wiki.gg/api.php",
        [
            ("action", "cargoquery"),
            ("tables", "Items"),
            ("format", "json"),
            ("limit", "max"),
            ("offset", &offset.to_string()),
            ("order_by", "name"),
            ("fields", &RawItem::fields().join(",")),
        ],
    )
    .unwrap();

    Ok(surf::get(url)
        .await
        .map_err(|e| e.into_inner())?
        .body_json::<CargoQuery<RawItem>>()
        .await
        .map_err(|e| e.into_inner())?
        .cargoquery
        .into_iter()
        .map(|entry| entry.title)
        .collect::<Vec<_>>())
}

pub async fn all_items() -> anyhow::Result<Vec<RawItem>> {
    let span = info_span!("Downloading items", indicatif.pb_show = true);
    span.pb_set_style(&ProgressStyle::default_bar());
    let _span_enter = span.enter();

    let count = count().await?;
    Span::current().pb_set_length(count as u64);

    let mut out = Vec::with_capacity(count);

    loop {
        let mut new_items = items(out.len()).await?;
        if out.len() >= count || new_items.is_empty() {
            break;
        }

        Span::current().pb_inc(new_items.len() as u64);

        out.append(&mut new_items);
    }

    Ok(out)
}
