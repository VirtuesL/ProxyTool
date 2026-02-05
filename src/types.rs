use std::{
    collections::HashMap,
    fmt::{self, Display},
};

use bytes::Bytes;
use log::debug;
use reqwest::Url;
use serde::Deserialize;


#[derive(Deserialize, Debug, PartialEq)]
pub enum BulkType {
    #[serde(rename = "oracle_cards")]
    OracleCards,
    #[serde(rename = "unique_artwork")]
    UniqueArtowrk,
    #[serde(rename = "default_cards")]
    DefaultCards,
    #[serde(rename = "all_cards")]
    AllCards,
    #[serde(rename = "rulings")]
    Rulings,
}

#[derive(Debug, Deserialize)]
pub struct List {
    pub has_more: bool,
    pub data: Vec<BulkData>,
}

#[derive(Debug, Deserialize)]
pub struct BulkData {
    pub id: String,
    #[serde(rename = "type")]
    pub bulk_type: BulkType,
    pub updated_at: String,
    pub uri: String,
    pub name: String,
    pub description: String,
    pub size: u128,
    pub download_uri: Url,
    pub content_type: String,
    pub content_encoding: String,
}

#[derive(Deserialize, Debug)]
pub enum BulkLanguage {
    #[serde(rename = "en")]
    English,
    #[serde(rename = "de")]
    German,
    #[serde(rename = "ja")]
    Japanese,
    #[serde(rename = "fr")]
    French,
    #[serde(rename = "es")]
    Spanish,
    #[serde(rename = "zhs")]
    SimplifiedChinese,
    #[serde(rename = "zht")]
    TraditionalChinese,
    #[serde(rename = "ru")]
    Russian,
    #[serde(rename = "it")]
    Italian,
    #[serde(rename = "ph")]
    Phyrexian,
    #[serde(rename = "pt")]
    Portugese,
    #[serde(rename = "qya")]
    Quenya,
    #[serde(rename = "sa")]
    Sanskrit,
    #[serde(rename = "grc")]
    AncientGreek,
    #[serde(rename = "he")]
    Hebrew,
    #[serde(rename = "ko")]
    Korean,
}
#[derive(Deserialize, Debug)]
pub enum BulkEntryType {
    #[serde(rename = "card")]
    Card,
}

#[derive(Deserialize, Debug, Clone)]
pub struct BulkImageUris {
    pub small: String,
    pub normal: String,
    pub large: String,
    pub png: String,
    pub art_crop: String,
    pub border_crop: String,
}

#[derive(Deserialize, Debug)]
pub enum Colors {
    White,
    Blue,
    Black,
    Red,
    Green,
    Colorless,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")] 
pub enum CardLayout {
    Normal,
    Transform,
    ArtSeries,
    Token,
    Class,
    Planar,
    Saga,
    Scheme,
    DoubleFacedToken,
    Meld,
    Prototype,
    Vanguard,
    Emblem,
    ModalDfc,
    Split,
    Adventure,
    Augment,
    Flip,
    Host,
    Mutate,
    Leveler,
    Case,
}

#[derive(Debug, Deserialize)]
#[serde(try_from = "Vec<BulkEntry>")]
pub struct BulkDB(pub HashMap<String, BulkEntry>);

#[derive(Debug,Deserialize)]
pub struct CardFaces {
    pub name: String,
    pub image_uris: Option<BulkImageUris>
}


#[derive(Deserialize, Debug)]
pub struct BulkEntry {
    // object: BulkEntryType,
    pub id: String,
    pub oracle_id: Option<String>,
    pub name: String,
    pub lang: BulkLanguage,
    pub uri: Url,
    pub scryfall_uri: Url,
    pub layout: CardLayout,
    pub highres_image: bool,
    pub image_status: String, //Should be enum
    pub image_uris: Option<BulkImageUris>,
    pub card_faces: Option<Vec<CardFaces>>
    // oracle_text: String,
    // colors: Vec<Colors>,
    // color_identity: Vec<Colors>,
    // "oversized":false,
    // "set_id":"a2f58272-bba6-439d-871e-7a46686ac018",
    // "set":"blb",
    // "set_name":"Bloomburrow",
    // "prints_search_uri":"https://api.scryfall.com/cards/search?order=released&q=oracleid%3Ab34bb2dc-c1af-4d77-b0b3-a0fb342a5fc6&unique=prints",
    // "rarity":"common",
    // "card_back_id":"0aeebaf5-8c7d-4636-9e82-8c27447861f7",
}

impl fmt::Display for BulkEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}, has highres: {:?}, url: {:?}",
            self.name, self.highres_image, self.image_uris
        )
    }
}

impl Display for BulkImageUris {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.png)
    }
}

#[derive(Debug, Deserialize,Clone)]
pub struct CardEntry<'a> {
    pub quantity: u32,
    pub backface: Option<bool>, // None -> Not double sided ---- WARNING THE DESER IMPL IS NOT
                                // STANDARD WILL NOT WORK FOR ANY OTHER FIELD
    pub name: String,
    #[serde(skip)]
    pub url: Option<&'a str>,
    #[serde(skip)]
    pub data: Option<Bytes>,
}

impl TryFrom<Vec<BulkEntry>> for BulkDB {
    type Error = String;

    fn try_from(value: Vec<BulkEntry>) -> Result<Self, Self::Error> {
        // This could be value.into_iter().map(…).collect(), but I care about duplicates.
        let mut map = HashMap::with_capacity(value.len());
        for item in value {
            let BulkEntry {
                id,
                oracle_id,
                name,
                lang,
                uri,
                scryfall_uri,
                layout,
                highres_image,
                image_status,
                image_uris,
                card_faces
            } = item;
            match map.entry(name.clone()) {
                std::collections::hash_map::Entry::Occupied(entry) => {
                    debug!("duplicate entry: {:?}", entry)
                }
                std::collections::hash_map::Entry::Vacant(entry) => {
                    entry.insert(BulkEntry {
                        id,
                        oracle_id,
                        name,
                        lang,
                        uri,
                        scryfall_uri,
                        layout,
                        highres_image,
                        image_status,
                        image_uris,
                        card_faces,
                    });
                }
            };
        }
        Ok(BulkDB(map))
    }
}
