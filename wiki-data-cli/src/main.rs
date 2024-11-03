use std::{
    collections::{HashMap, HashSet},
    fs::File,
    path::PathBuf,
};

use clap::Parser;
use rand::{seq::SliceRandom as _, thread_rng};
use rmp_serde::config::BytesMode;
use serde::Serialize as _;
use tracing::level_filters::LevelFilter;
use tracing_indicatif::{
    filter::{hide_indicatif_span_fields, IndicatifFilter},
    IndicatifLayer,
};
use tracing_subscriber::{
    fmt::format::DefaultFields, layer::SubscriberExt as _, util::SubscriberInitExt as _, Layer,
};

use wiki_data::{
    image::Image,
    item::{Item, RawItem},
    ImageLocation,
};

mod download;
mod generate;

#[derive(Parser)]
struct Args {
    #[clap(subcommand)]
    subcmd: Subcommand,
}

#[derive(clap::Subcommand)]
enum Subcommand {
    Download {},
    Parse {
        #[clap(long)]
        generate_rust: Option<PathBuf>,

        #[clap(long)]
        generate_msgpack: Option<PathBuf>,
    },
    LocateImages {},
    DownloadImages {},
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

            tracing::info!("downloaded {} items", items.len());

            let out = rmp_serde::to_vec(&items)?;
            std::fs::write("raw-items.bin", out)?;
        }
        Subcommand::Parse { generate_rust, generate_msgpack } => {
            let raw_items = parse_raw_items()?;
            let image_locations = parse_image_locations()?
                .into_iter()
                .map(|il| (il.name.clone(), il))
                .collect::<HashMap<_, _>>();

            let pb = indicatif::ProgressBar::new(raw_items.len() as u64);
            let mut items = pb
                .wrap_iter(raw_items.iter())
                .filter_map(|raw_item| {
                    Item::from_raw(
                        raw_item,
                        image_locations.get(raw_item.imagefile()?.as_str()).cloned(),
                    )
                })
                .collect::<Vec<Item>>();

            pb.finish();

            items.sort_by_key(|i| i.item_id);

            if let Some(path) = generate_rust {
                generate::rust(&items, path)?
            }

            if let Some(path) = generate_msgpack {
                generate::msgpack(&items, path)?
            }

            for item in items.choose_multiple(&mut thread_rng(), 10) {
                tracing::info!("{:#?}", item);
            }

            tracing::info!("parsed {}/{} items", items.len(), raw_items.len());
        }
        Subcommand::LocateImages {} => {
            let mut images = File::open("image-locations.bin")
                .ok()
                .and_then(|f| rmp_serde::from_read::<_, Vec<ImageLocation>>(f).ok())
                .unwrap_or_default();

            let mut items = parse_raw_items()?;

            let contained_names = images
                .iter()
                .map(|i| dbg!(i.name.as_str()))
                .collect::<HashSet<_>>();

            items.retain(|item| {
                if let Some(imagefile) = item.imagefile() {
                    !contained_names.contains(imagefile.as_str())
                } else {
                    false
                }
            });

            for chunk in items.chunks(50) {
                tracing::info!("{chunk:#?}");

                images.append(
                    &mut download::images(chunk.iter().filter_map(|i| i.imagefile()).collect())
                        .await?,
                );

                tracing::info!("{images:#?}");
            }

            images.sort_by_key(|i| i.name.clone());

            let out = rmp_serde::to_vec(&images)?;
            std::fs::write("image-locations.bin", out)?;

            tracing::info!("now have {} images", images.len());
        }
        Subcommand::DownloadImages {} => {
            let mut out = File::open("images.bin")
                .ok()
                .and_then(|f| rmp_serde::from_read::<_, Vec<Image>>(f).ok())
                .unwrap_or_default();

            let mut images = parse_image_locations()?;

            let downloaded = out.iter().map(|i| i.name.as_str()).collect::<HashSet<_>>();

            images.retain(|i| !downloaded.contains(i.name.as_str()));

            let pb = indicatif::ProgressBar::new(images.len() as u64);
            let images = pb.wrap_iter(images.into_iter());

            let mut client = surf::Client::new();

            for image in images {
                match download::image(&image, &mut client).await {
                    Ok(image) => {
                        out.push(image);
                    }
                    Err(e) => {
                        tracing::error!("failed to download {}: {}", image.name, e);
                        break;
                    }
                }
            }

            pb.finish();

            let mut bytes = Vec::new();
            let mut serializer =
                rmp_serde::Serializer::new(&mut bytes).with_bytes(BytesMode::ForceAll);
            out.serialize(&mut serializer)?;
            std::fs::write("wiki-data/src/images.bin", bytes)?;
        }
    }

    Ok(())
}

fn parse_raw_items() -> anyhow::Result<Vec<RawItem>> {
    let reader = File::open("raw-items.bin")?;
    let len = reader.metadata()?.len();
    let pb = indicatif::ProgressBar::new(len);
    let reader = pb.wrap_read(reader);

    let raw_items = rmp_serde::from_read::<_, Vec<RawItem>>(reader)?;
    pb.finish();

    Ok(raw_items)
}

fn parse_image_locations() -> anyhow::Result<Vec<ImageLocation>> {
    let reader = File::open("image-locations.bin")?;
    let len = reader.metadata()?.len();
    let pb = indicatif::ProgressBar::new(len);
    let reader = pb.wrap_read(reader);

    let image_locations = rmp_serde::from_read::<_, Vec<ImageLocation>>(reader)?;
    pb.finish();

    Ok(image_locations)
}
