use iced::{
    Element, Length, Task,
    widget::{
        container::{self, bordered_box},
        text, text_editor, text_input,
    },
};

use crate::{Message, card::SCCard};

#[derive(Default, Debug)]
pub struct OptionPanel {
    content: text_editor::Content,
}

impl OptionPanel {
    pub fn view(&self) -> Element<'_, Message> {
        container::Container::new(
            text_editor(&self.content)
                .placeholder("1x Island")
                .on_action(Message::CardInput),
        )
        .style(bordered_box)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::CardInput(action) => {
                self.content.perform(action);
            }
            _ => {
                unreachable!("yay")
            }
        };
        Task::none()
    }
}

#[derive(Debug, Default)]
pub struct DetailPanel<'a> {
    state: DetailPanelState<'a>,
}

#[derive(Debug, Default)]
pub enum DetailPanelState<'a> {
    #[default]
    Empty,
    VariantPicker(&'a SCCard),
}

impl DetailPanel<'_> {
    pub fn view(&self) -> Element<'_, Message> {
        container::Container::new(text("test"))
            .style(bordered_box)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
