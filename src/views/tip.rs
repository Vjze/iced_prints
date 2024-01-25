use iced::{
    alignment::{self, Horizontal},
    widget::{button, container, row, column, text, Container},
    Length,
};
use iced_aw::{modal, Card};

use super::main_ui::{Message, MyTools};

pub fn tip(
    state: &MyTools,
    c: Container<'static, Message>,
    body: &str,
) -> Container<'static, Message> {
    let overlay = if state.show_modal {
        Some(
            Card::new(
                container(column!(text("提示")).align_items(iced::Alignment::Center)).center_x().width(Length::Fill),
                text(format!("{}", body)).horizontal_alignment(Horizontal::Center),
            )
            .foot(
                row![
                    button(text("确认").horizontal_alignment(Horizontal::Center),)
                        .width(Length::Fill)
                        .on_press(Message::CloseModal),
                ]
                .spacing(10)
                .padding(5)
                .width(Length::Fill),
            )
            .max_width(300.0),
            // .on_close(Message::CloseModal),
        )
    } else {
        None
    };
    let tip = modal(c, overlay)
        // .backdrop(Message::OpenModal)
        // .on_esc(Message::CloseModal)
        .align_y(alignment::Vertical::Center);

    container(tip).into()
}
