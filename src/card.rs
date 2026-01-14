use iced::{
    Element,
    widget::{Image, image, row},
};
use scryfall::card::Card;

use crate::Message;

#[derive(Debug, Clone)]
pub struct SCCard {
    name: String,
    image: image::Handle,
}

impl SCCard {
    pub fn view(&self) -> Element<'_, Message> {
        row![image::viewer(self.image.clone())].into()
    }

    pub async fn random() -> Result<Self, Error> {
        let c = Card::random().await?;
        let bytes = reqwest::get(c.image_uris.unwrap().png.unwrap())
            .await?
            .bytes()
            .await?;
        Ok(Self {
            name: c.name,
            image: image::Handle::from_bytes(bytes),
        })
    }
}

#[derive(Debug, Clone)]
pub enum Error {
    APIError,
    LanguageError,
}

impl From<scryfall::Error> for Error {
    fn from(error: scryfall::Error) -> Error {
        dbg!(error);

        Error::APIError
    }
}
impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Error {
        dbg!(error);

        Error::APIError
    }
}
