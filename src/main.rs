use iced::{
    Element, Length, Task,
    widget::{Column, Row, button, container, image, row, text, text_editor},
};
mod card;
mod panels;

use crate::panels::DetailPanel;
use crate::{card::SCCard, panels::OptionPanel};

#[derive(Default)]
struct ProxyTool<'a> {
    cards: Vec<SCCard>,
    details: DetailPanel<'a>,
    options: OptionPanel,
}

#[derive(Clone, Debug)]
enum Message {
    Random,
    CardFound(Result<SCCard, card::Error>),
    CardInput(text_editor::Action),
}

fn main() -> iced::Result {
    iced::run(ProxyTool::update, ProxyTool::view)
}

impl ProxyTool<'_> {
    fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::Random => Task::perform(SCCard::random(), Message::CardFound),
            Message::CardFound(Ok(c)) => {
                self.cards.push(c);
                Task::none()
            }
            Message::CardInput(_) => self.options.update(message), // Forward message to options
                                                                   // handler,
            _ => {
                todo!()
            }
        }
    }
    fn view(&self) -> Element<'_, Message> {
        row![
            Column::new()
                .push(text! {"LEFT"})
                .push(self.options.view())
                .width(Length::FillPortion(15)),
            Column::new()
                .width(Length::FillPortion(70))
                .extend(self.cards.iter().map(|c| c.view())),
            Column::new()
                .push(text! {"RIGHT"})
                .push(self.details.view())
                .width(Length::FillPortion(15)),
        ]
        .into()
    }
}
