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
    card_input: String,
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
    fn parse_card_list(&mut self) {
        self.cards.clear();
        for line in self.card_input.lines() {
            let line = line.trim();

            // skip empty lines and comments:
            if line.is_empty() || line.starts_with("#") || line.starts_with("//") {
                continue
            }

            let (quantity, name) = if let Some(first_char) = line.chars().next() {
                if first_char.is_ascii_digit() {
                    let mut split_idx = 0;

                    for (i, c) in line.char_indices() {
                        if !c.is_ascii_digit() {
                            split_idx = i;
                            break;
                        }
                    }
                    let qty_str = &line[..split_idx];
                    let mut rest = line[split_idx..].trim_start();

                    if rest.starts_with("x") || rest.starts_with("X") {
                        rest = rest[1..].trim_start();
                    }

                    let qty: u32 = qty_str.parse().unwrap_or(1);
                    (qty, rest.to_string())
                } else {
                    (1, line.to_string())
                }
            } else {
                continue;
            };
        }
    }

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
