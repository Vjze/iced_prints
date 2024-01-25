use crate::styles::{
    button_styles::transparent_button_with_rounded_border_theme,
    container_styles::{first_class_container_rounded_theme, second_class_container_rounded_theme}, scrollable_styles::ScrollbarStyle,
};
use iced::{
    alignment::Horizontal, widget::{
        button, column, container, pick_list, row, scrollable, text, text_input, Container, Space,
    }, Alignment, Length, Renderer
};
use once_cell::sync::Lazy;
pub static CARTON_MESSAGE_LOG: Lazy<scrollable::Id> = Lazy::new(scrollable::Id::unique);
use super::main_ui::{Message, MyTools};

pub fn cartonview(state: &MyTools) -> Container<'_, Message, Renderer> {
    let mut input = text_input("输入盒号...", &state.carton_input_value).size(16);

    let mut add_btn = button("添 加");

    let mut print_but = button("打 印").style(transparent_button_with_rounded_border_theme());
    let mut cance_but = button("取 消").style(transparent_button_with_rounded_border_theme());
    if !state.show_modal && !state.selects_carton.is_empty() && state.selected_carton.is_some() && state.carton_now_num < state.carton_configs.max_num {
        input = input
            .on_input(Message::CartonTextchange)
            .on_submit(Message::CartonAddButton);
        add_btn = add_btn.on_press(Message::CartonAddButton);
        if state.carton_now_num == state.carton_configs.max_num {
            print_but = print_but.on_press(Message::CartonPrintButton)
        }
    }
    if !state.carton_list.is_empty(){
        cance_but = cance_but.on_press(Message::CartonCance)
    }
    let first_row = row![input, add_btn, print_but, cance_but]
        .spacing(10)
        .padding(5)
        .width(Length::Fill)
        .align_items(Horizontal::Center.into());
    let select_text = text("选择料号:");
    let pn_select = pick_list(&state.selects_carton, state.selected_carton.clone(), Message::CartonSelected);
    let max_text = text(format!("最大装箱数:{}", state.carton_configs.max_num));
    let now_num = text(format!("当前扫描数量:{}", state.carton_now_num));
    let row = row!(
        select_text,
        pn_select,
        max_text,
        Space::new(Length::Fill, Length::Fixed(25.0)),
        now_num
    )
    .spacing(10)
    .padding(5)
    .width(Length::Fill)
    .align_items(Horizontal::Center.into());
    let title = row!(
        text("序号").size(20),
        Space::new(Length::Fill, Length::Fixed(25.0)),
        text("盒号").size(20),
        Space::new(Length::Fill, Length::Fixed(25.0)),
        text("工单号").size(20),
        Space::new(Length::Fill, Length::Fixed(25.0)),
        text("操作员").size(20)
    )
    .align_items(iced::Alignment::Center)
    .width(Length::Fill)
    .padding(10);
    let listview = state.carton_list.iter().fold(
        column![].spacing(5).align_items(Alignment::Center),
        |col, i| {
            col.push(
                container(
                    row!(
                        text(i.id.clone()),
                        Space::new(Length::Fill, Length::Shrink),
                        text(i.box_sn.clone()),
                        Space::new(Length::Fill, Length::Shrink),
                        text(i.order.clone()),
                        Space::new(Length::Fill, Length::Shrink),
                        text(i.worker.clone()),
                    )
                    .width(Length::Fill)
                    .align_items(Alignment::Center)
                    .height(Length::Fill),
                    // .padding(5),
                )
                .width(Length::Shrink)
                .center_x()
                .center_y()
                .height(30)
                .padding(5)
                .style(first_class_container_rounded_theme()),
            )
        },
    );
    let list_scrollable = scrollable(listview)
        .height(Length::Fill)
        .width(Length::Fill)
        .style(ScrollbarStyle::theme())
        .id(CARTON_MESSAGE_LOG.clone());
    let col_1 = container(column!(title,list_scrollable)).style(second_class_container_rounded_theme());
    let cols = column!(first_row, row,col_1).width(Length::Fill);
    container(cols).padding(5).height(Length::Fill).style(first_class_container_rounded_theme()).into()
}
