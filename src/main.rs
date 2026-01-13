use iced::{Element, Task, widget::{Column, Row, button, image, row, text}};
mod card;

use crate::card::SCCard;

#[derive(Default)]
struct ProxyTool {
    cards: Vec<SCCard>
}

#[derive(Clone,Debug)]
enum Message {
    Random,
    CardFound(Result<SCCard,card::Error>)
}

fn main() -> iced::Result {
    iced::run(ProxyTool::update, ProxyTool::view)
}

impl ProxyTool {
    fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message{
            Message::Random => {
                Task::perform(SCCard::random(),Message::CardFound)
            },
            Message::CardFound(Ok(c)) => {
                self.cards.push(c);
                Task::none()
            },
            _ => {todo!()}
        }
    }
    fn view(&self) -> Element<'_,Message> {
        row![
        Column::new()
            .push(text!("{} Cards loaded",self.cards.len()))
            .push(
            button("Random")
            .on_press(Message::Random)
            ),
        ].extend(self.cards.iter().map(|c|c.view()))
        .into()
    }
}
