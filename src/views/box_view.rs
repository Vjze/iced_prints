use crate::styles::{
    button_styles::transparent_button_with_rounded_border_theme,
    colors::{green, red},
    container_styles::{first_class_container_rounded_theme, second_class_container_rounded_theme},
    scrollable_styles::ScrollbarStyle,
};
use iced::{
    alignment::Horizontal,
    theme::Text,
    widget::{
        button, column, container, pick_list, row, scrollable, text, text_input, Container, Space,
    },
    Alignment, Font, Length, Renderer,
};
use once_cell::sync::Lazy;
pub static BOX_MESSAGE_LOG: Lazy<scrollable::Id> = Lazy::new(scrollable::Id::unique);
use super::main_ui::{Message, MyTools};

pub fn boxview(state: &MyTools) -> Container<'_, Message, Renderer> {
    let mut input = text_input("输入SN...", &state.box_input_value).size(16);

    let mut add_btn = button("添 加");

    let mut print_but = button("打 印").style(transparent_button_with_rounded_border_theme());
    let mut cance_btn = button("取 消").style(transparent_button_with_rounded_border_theme());
    if !state.show_modal
        && !state.box_selects.is_empty()
        && state.box_selected.is_some()
        && state.box_now_num < state.box_configs.max_num
    {
        input = input
            .on_input(Message::BoxTextchange)
            .on_submit(Message::BoxAddButton);
        add_btn = add_btn.on_press(Message::BoxAddButton);
        if state.box_now_num == state.box_configs.max_num {
            print_but = print_but.on_press(Message::BoxPrintButton)
        }
    }
    
    if !state.box_list.is_empty() {
        cance_btn = cance_btn.on_press(Message::BoxCance)
    }
    let first_row = row![input, add_btn, print_but, cance_btn]
        .spacing(10)
        .padding(5)
        .width(Length::Fill)
        .align_items(Horizontal::Center.into());
    let select_text = text("选择料号:");
    let pn_select = pick_list(
        &state.box_selects,
        state.box_selected.clone(),
        Message::BoxSelected,
    );
    let max_text = text(format!("最大装盒数:{}", state.box_configs.max_num));
    let sn_len_text = text(format!("SN长度:{}", state.box_configs.max_sn));
    let now_num = text(format!("当前扫描数量:{}", state.box_now_num));
    let yq_text = text("规格：").size(20).style(Text::Color(green()));
    let ith_text = text(format!(
        "ITH:{}-{}",
        state.box_configs.box_min_ith, state.box_configs.box_max_ith
    ))
    .style(Text::Color(red()))
    .font(Font {
        weight: iced::font::Weight::Bold,
        ..Default::default()
    });
    let po_text = text(format!(
        "PO:{}-{}",
        state.box_configs.box_min_po, state.box_configs.box_max_po
    ))
    .style(Text::Color(red()))
    .font(Font {
        weight: iced::font::Weight::Bold,
        ..Default::default()
    });
    let sen_text = text(format!(
        "SEN:{}-{}",
        state.box_configs.box_min_sen, state.box_configs.box_max_sen
    ))
    .style(Text::Color(red()))
    .font(Font {
        weight: iced::font::Weight::Bold,
        ..Default::default()
    });
    let res_text = text(format!(
        "RES:{}-{}",
        state.box_configs.box_min_res, state.box_configs.box_max_res
    ))
    .style(Text::Color(red()))
    .font(Font {
        weight: iced::font::Weight::Bold,
        ..Default::default()
    });
    let vf_text = text(format!(
        "VF:{}-{}",
        state.box_configs.box_min_vf, state.box_configs.box_max_vf
    ))
    .style(Text::Color(red()))
    .font(Font {
        weight: iced::font::Weight::Bold,
        ..Default::default()
    });
    let im_text = text(format!(
        "IM:{}-{}",
        state.box_configs.box_min_im, state.box_configs.box_max_im
    ))
    .style(Text::Color(red()))
    .font(Font {
        weight: iced::font::Weight::Bold,
        ..Default::default()
    });
    let se_text = text(format!(
        "SE:{}-{}",
        state.box_configs.box_min_se, state.box_configs.box_max_se
    ))
    .style(Text::Color(red()))
    .font(Font {
        weight: iced::font::Weight::Bold,
        ..Default::default()
    });
    let icc_text = text(format!(
        "ICC:{}-{}",
        state.box_configs.box_min_icc, state.box_configs.box_max_icc
    ))
    .style(Text::Color(red()))
    .font(Font {
        weight: iced::font::Weight::Bold,
        ..Default::default()
    });
    let row_2 =
        row!(yq_text, ith_text, po_text, sen_text, res_text, vf_text, im_text, se_text, icc_text,)
            .align_items(Alignment::Center)
            .spacing(25);
    let row = row!(
        select_text,
        pn_select,
        max_text,
        sn_len_text,
        Space::new(Length::Fill, Length::Fixed(25.0)),
        now_num
    )
    .spacing(20)
    .padding(5)
    .width(Length::Fill)
    .align_items(Horizontal::Center.into());
    let title = row!(
        text("序号").size(20),
        Space::new(Length::Fill, Length::Fixed(25.0)),
        text("SN").size(20),
        Space::new(Length::Fill, Length::Fixed(25.0)),
        text("工单号").size(20),
        Space::new(Length::Fill, Length::Fixed(25.0)),
        text("操作员").size(20)
    )
    .align_items(iced::Alignment::Center)
    .width(Length::Fill)
    .padding(10);
    let listview = state.box_list.iter().fold(
        column![].spacing(5).align_items(Alignment::Center),
        |col, i| {
            col.push(
                container(
                    row!(
                        text(i.id + 1.clone()),
                        Space::new(Length::Fill, Length::Shrink),
                        text(i.sn.clone()),
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
        .id(BOX_MESSAGE_LOG.clone());
    let col_1 =
        container(column!(title, list_scrollable)).style(second_class_container_rounded_theme());
    let cols = column!(first_row, row, row_2, col_1)
        .width(Length::Fill)
        .align_items(Alignment::Start);
    container(cols)
        .height(Length::Fill)
        .padding(5)
        .style(first_class_container_rounded_theme())
        .into()
}
