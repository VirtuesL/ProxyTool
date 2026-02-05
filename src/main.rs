use clap::Parser;
use futures_util::{StreamExt, stream};
use std::{
    io::{Cursor, IsTerminal, Read, Write},
    path::PathBuf, process::exit, str::FromStr,
};

use crate::types::{BulkDB, CardEntry};
use consts::*;
use image::{ImageBuffer, ImageReader};
use log::{debug, info, warn};

mod consts;
mod parser;
mod types;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short, long)]
    file: PathBuf,
}

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

#[tokio::main]
async fn main() {
    env_logger::init();
    let client = reqwest::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()
        .unwrap();
    let database_name = if let Some(path) = glob::glob("oracle-cards-*").unwrap().next() {
        info!("We have db. parsing, this may take a second");
        path.map_err(anyhow::Error::new)
    } else {
        warn!("No db found, updating with default db");
        let res = client
            .get("https://api.scryfall.com/bulk-data")
            .send()
            .await
            .unwrap();
        let json = res.json::<types::List>().await.unwrap();
        let default_cards_url = json
            .data
            .iter()
            .find(|p| p.bulk_type == types::BulkType::OracleCards)
            .unwrap();
        let db_name = default_cards_url
            .download_uri
            .path()
            .split("/")
            .last()
            .unwrap();
        let mut dest = std::fs::File::create(db_name).unwrap();
        let b = reqwest::get(default_cards_url.download_uri.clone())
            .await
            .unwrap()
            .bytes()
            .await
            .unwrap();
        let _ = dest.write(&b);
        PathBuf::from_str(db_name).map_err(anyhow::Error::new)
    };
    let testjson = std::fs::read(database_name.unwrap()).unwrap();
    let database_handle = tokio::spawn(async move {
        serde_json::from_slice::<BulkDB>(&testjson)
    });
    let mut stdin = std::io::stdin();
    let cardtext = if stdin.is_terminal() {
        let args = Args::parse();
        info!("Parser got args {:?}", args);
        std::fs::read_to_string(args.file).unwrap()
    } else {
        let mut cardtext = Default::default();
        let _read = stdin.read_to_string(&mut cardtext);
        cardtext
    };
    let cards = parser::from_str::<Vec<CardEntry>>(&cardtext).unwrap();
    info!("Got {:?} cards", cards);
    let mut cards: Vec<CardEntry> = cards.into_iter()
        .flat_map(|card| if card.backface.is_some(){let mut c2 = card.clone(); c2.backface.replace(true); vec![card.clone(),c2]}else{vec![card]})
        .collect();
    let database = database_handle.await.unwrap();
    match database {
        Ok(ref db) => {
            cards.iter_mut().for_each(|card| {
                let foundc = db.0.get(&card.name);
                if let Some(dbc) = foundc {
                    let source = match dbc.layout {
                        types::CardLayout::DoubleFacedToken | types::CardLayout::Flip | types::CardLayout::Meld | types::CardLayout::Transform
                            => {
                                if let Some(ref faces) = dbc.card_faces {
                                    match card.backface {
                                        Some(false) => &faces[0].image_uris,
                                        Some(true) => &faces[1].image_uris,
                                        None => &dbc.image_uris

                                    }
                                } else {&dbc.image_uris}
                            },
                        _   => {&dbc.image_uris}

                    };
                    if let Some(uri) = source {card.url = Some(uri.png.as_str())};
                    
                } else {
                    warn!("couldn't find card in database : {}",card.name)
                };
            });
        }
        Err(e) => {
            panic!("couldnt complete db, {}", e)
        }
    }
    let bodies = stream::iter(cards)
        .map(|mut card| {
            let client = &client;
            async move {
                match card.url {
                    Some(url) => {
                        info!("Downloading image for {}",card.name);
                        let resp = client.get(url).send().await.unwrap();
                        let bytes = resp.bytes().await;
                        card.data = bytes.map(Some).expect(":)")
                    }
                    None => {
                        warn!("could not find card downlaod image for {}", card.name);
                    },
                };
                card
            }
        })
        .buffer_unordered(8)
        .collect::<Vec<_>>();
    info!("Downloading data");
    let cards = bodies.await;
    info!("Done downloading data, generating proxies");
    let res = generate_proxy_pages(&cards, "img", false);
    info!("Done, {res:?}")
}

fn generate_proxy_pages(
    cards: &Vec<CardEntry>,
    output_dir: &str,
    _show_cut_lines: bool,
) -> anyhow::Result<()> {
    // Create output directory
    std::fs::create_dir_all(output_dir)?;

    // Collect unique card names
    let cards = cards.iter().flat_map(|c|{vec![c; c.quantity as usize] }).collect::<Vec<&CardEntry>>();
    for (page_num, card_chunk) in cards.chunks(9).enumerate() {
        info!("Generating page {page_num}");
        let mut page = ImageBuffer::new(A4_WIDTH as u32, A4_HEIGHT as u32);
        page.fill(255);
        card_chunk.iter().enumerate().for_each(|(i, card)| {
            debug!("Putting card {}",card.name);
            let data = card.data.as_ref().unwrap();
            let mut card_image = ImageReader::new(Cursor::new(data));
            card_image.set_format(image::ImageFormat::Png);
            let (x, y) = (
                CARD_WIDTH * (i / CARDS_PER_ROW) + MARGIN_X,
                CARD_HEIGHT * (i % CARDS_PER_COL) + MARGIN_Y,
            );
            image::imageops::overlay(&mut page, &card_image.decode().unwrap(), x as i64, y as i64);
            debug!("Done with {}",card.name);

        });
        info!("Saving page");
        page.save(format!("{output_dir}/page-{page_num}.png"))?;
    }
    Ok(())
}
