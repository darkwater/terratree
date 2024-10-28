use std::fs::File;

use clap::Parser;
use rand::{seq::SliceRandom as _, thread_rng};
use tracing::level_filters::LevelFilter;
use tracing_indicatif::{
    filter::{hide_indicatif_span_fields, IndicatifFilter},
    IndicatifLayer,
};
use tracing_subscriber::{
    fmt::format::DefaultFields, layer::SubscriberExt as _, util::SubscriberInitExt as _, Layer,
};

mod download;
mod item;

#[derive(Parser)]
struct Args {
    #[clap(subcommand)]
    subcmd: Subcommand,
}

#[derive(clap::Subcommand)]
enum Subcommand {
    Parse {},
    Download {},
}

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let indicatif_layer = IndicatifLayer::new()
        .with_span_field_formatter(hide_indicatif_span_fields(DefaultFields::new()));

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(indicatif_layer.get_stderr_writer())
                .with_filter(LevelFilter::INFO),
        )
        .with(indicatif_layer.with_filter(IndicatifFilter::new(false)))
        .init();

    let args = Args::parse();

    match args.subcmd {
        Subcommand::Download {} => {
            let items = download::all_items().await?;

            let out = rmp_serde::to_vec(&items)?;
            std::fs::write("raw-items.bin", out)?;
        }
        Subcommand::Parse {} => {
            let reader = File::open("raw-items.bin")?;
            let len = reader.metadata()?.len();
            let pb = indicatif::ProgressBar::new(len);
            let reader = pb.wrap_read(reader);

            let items = rmp_serde::from_read::<_, Vec<item::RawItem>>(reader)?;
            pb.finish();

            let pb = indicatif::ProgressBar::new(items.len() as u64);
            let items = pb
                .wrap_iter(items.into_iter())
                .filter_map(|raw_item| raw_item.parse())
                .collect::<Vec<item::Item>>();

            pb.finish();

            for item in items.choose_multiple(&mut thread_rng(), 10) {
                tracing::info!("{:#?}", item);
            }

            tracing::info!("parsed {} items", items.len());
        }
    }

    Ok(())
}
