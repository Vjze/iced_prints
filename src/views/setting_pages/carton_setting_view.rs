use iced::{
    widget::{button, column, container, pick_list, row, text, text_input, Container},
    Renderer,
};

use crate::views::setting::{Message, SettingView};

pub fn carton_setting_view(state: &SettingView) -> Container<'_, Message, Renderer> {
    let pn_text = text("选择料号:");
    let pn_select = pick_list(
        &state.carton_selects,
        state.carton_selected.clone(),
        Message::BoxSelected,
    );
    let pn_new_text = text("当前料号:");
    let pn_new_input =
        text_input("新增请直接在此输入料号", &state.carton_pn_new).on_input(Message::CartonPnNewChange);
    
    let max_num_text = text("装箱数量:");
    let max_num_input = text_input("", &state.carton_max_num).on_input(Message::CartonMaxNumChange);
    let btw_modle_text = text("标签名称:");
    let btw_modle_input = text_input("", &state.carton_btw_name).on_input(Message::CartonBtwNameChange);
    let first_row = row!(
        pn_text,
        pn_select,
        pn_new_text,
        pn_new_input,
        max_num_text,
        max_num_input,
        btw_modle_text,
        btw_modle_input
    )
    .align_items(iced::Alignment::Center)
    .spacing(25);
    let mut updata_btn = button("修改");
    let mut add_btn = button("新增");
    if !state.carton_pn_new.is_empty()
        && !state.sn_len.is_empty()
        && !state.carton_max_num.is_empty()
        && !state.carton_btw_name.is_empty()
        && state.box_selected.is_none()
    {
        add_btn = add_btn.on_press(Message::CartonAddButton)
    }
    if state.box_selected.is_some()
        && !state.sn_len.is_empty()
        && !state.carton_max_num.is_empty()
        && !state.carton_btw_name.is_empty()
    {
        updata_btn = updata_btn.on_press(Message::CartonUpdateButton)
    }
    let row = row!(updata_btn, add_btn)
        .align_items(iced::Alignment::Center)
        .spacing(30);
    let col = column!(first_row, row)
        .align_items(iced::Alignment::Center)
        .spacing(20);
    container(col).into()
}
