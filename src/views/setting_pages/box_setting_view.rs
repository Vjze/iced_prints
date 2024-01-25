use iced::{
    widget::{button, column, container, pick_list, row, text, text_input, Container}, Length, Renderer
};

use crate::views::setting::{Message, SettingView};

pub fn box_setting_view(state: &SettingView) -> Container<'_, Message, Renderer> {
    let pn_text = text("选择料号:");
    let pn_select = pick_list(
        &state.box_selects,
        state.box_selected.clone(),
        Message::BoxSelected,
    );
    let pn_new_text = text("当前料号:");
    let pn_new_input =
        text_input("新增请直接在此输入料号", &state.box_pn_new).on_input(Message::BoxPnNewChange);
    let sn_len_text = text("SN长度:");
    let sn_len_input = text_input("", &state.sn_len).on_input(Message::SnLenChange);
    let max_num_text = text("装盒数量:");
    let max_num_input = text_input("", &state.box_max_num).on_input(Message::BoxMaxNumChange);
    let btw_modle_text = text("标签名称:");
    let btw_modle_input = text_input("", &state.box_btw_name).on_input(Message::BoxBtwNameChange);
    let first_row = row!(
        pn_text,
        pn_select,
        pn_new_text,
        pn_new_input,
        sn_len_text,
        sn_len_input,
        max_num_text,
        max_num_input,
        btw_modle_text,
        btw_modle_input
    )
    .align_items(iced::Alignment::Center)
    .spacing(25);
    let ith_text = text("ITH:");
    let ith_input = text_input("ith", &state.box_ith).on_input(Message::BoxIthChange);
    let po_text = text("PO:");
    let po_input = text_input("po", &state.box_po).on_input(Message::BoxPoChange);
    let sen_text = text("Sen:");
    let sen_input = text_input("sen", &state.box_sen).on_input(Message::BoxSenChange);
    let res_text = text("Res:");
    let res_input = text_input("res", &state.box_res).on_input(Message::BoxResChange);
    let vf_text = text("VF:");
    let vf_input = text_input("vf", &state.box_vf).on_input(Message::BoxVfChange);
    let im_text = text("IM:");
    let im_input = text_input("im", &state.box_im).on_input(Message::BoxImChange);
    let se_text = text("SE:");
    let se_input = text_input("se", &state.box_se).on_input(Message::BoxSeChange);
    let icc_text = text("ICC:");
    let icc_input = text_input("icc", &state.box_icc).on_input(Message::BoxIccChange);
    let row_2 = row!(
        ith_text, ith_input, po_text, po_input, vf_text, vf_input, im_text, im_input, sen_text, sen_input, res_text,
        res_input, icc_text, icc_input, se_text, se_input
    )
    .align_items(iced::Alignment::Center).spacing(10);
    let mut updata_btn = button("修改");
    let mut add_btn = button("新增");
    if !state.box_pn_new.is_empty()
        && !state.sn_len.is_empty()
        && !state.box_max_num.is_empty()
        && !state.box_btw_name.is_empty()
        && state.box_selected.is_none()
    {
        add_btn = add_btn.on_press(Message::BoxAddButton)
    }
    if state.box_selected.is_some()
        && !state.sn_len.is_empty()
        && !state.box_max_num.is_empty()
        && !state.box_btw_name.is_empty()
    {
        updata_btn = updata_btn.on_press(Message::BoxUpdateButton)
    }
    let row = row!(updata_btn, add_btn)
        .align_items(iced::Alignment::Center)
        .spacing(30);
    let col = column!(first_row, row_2, row)
        .align_items(iced::Alignment::Center)
        .spacing(20);
    container(col).height(Length::Fill).into()
}
