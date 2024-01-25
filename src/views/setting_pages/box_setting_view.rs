use iced::{
    widget::{button, column, container, pick_list, row, text, text_input, Container},
    Length, Renderer,
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
    let ith_min_input =
        text_input("ith下限", &state.box_min_ith).on_input(Message::BoxIthMinChange);
    let ith_max_input =
        text_input("ith上限", &state.box_max_ith).on_input(Message::BoxIthMaxChange);
    let po_text = text("PO:");
    let po_min_input = text_input("po下限", &state.box_min_po).on_input(Message::BoxPoMinChange);
    let po_max_input = text_input("po上限", &state.box_max_po).on_input(Message::BoxPoMaxChange);

    let sen_text = text("Sen:");
    let sen_min_input =
        text_input("sen下限", &state.box_min_sen).on_input(Message::BoxSenMinChange);
    let sen_max_input =
        text_input("sen上限", &state.box_max_sen).on_input(Message::BoxSenMaxChange);

    let res_text = text("Res:");
    let res_min_input =
        text_input("res下限", &state.box_min_res).on_input(Message::BoxResMinChange);
    let res_max_input =
        text_input("res上限", &state.box_max_res).on_input(Message::BoxResMaxChange);

    let vf_text = text("VF:");
    let vf_min_input = text_input("vf下限", &state.box_min_vf).on_input(Message::BoxVfMinChange);
    let vf_max_input = text_input("vf上限", &state.box_max_vf).on_input(Message::BoxVfMaxChange);

    let im_text = text("IM:");
    let im_min_input = text_input("im下限", &state.box_min_im).on_input(Message::BoxImMinChange);
    let im_max_input = text_input("im上限", &state.box_max_im).on_input(Message::BoxImMaxChange);

    let se_text = text("SE:");
    let se_min_input = text_input("se下限", &state.box_min_se).on_input(Message::BoxSeMinChange);
    let se_max_input = text_input("se上限", &state.box_max_se).on_input(Message::BoxSeMaxChange);

    let icc_text = text("ICC:");
    let icc_min_input =
        text_input("icc下限", &state.box_min_icc).on_input(Message::BoxIccMinChange);
    let icc_max_input =
        text_input("icc上限", &state.box_max_icc).on_input(Message::BoxIccMaxChange);

    let row_2 = row!(
        ith_text,
        ith_min_input,
        ith_max_input,
        po_text,
        po_min_input,
        po_max_input,
        vf_text,
        vf_min_input,
        vf_max_input,
        im_text,
        im_min_input,
        im_max_input,
    )
    .align_items(iced::Alignment::Center)
    .spacing(10);
    let row_3 = row!(
        sen_text,
        sen_min_input,
        sen_max_input,
        res_text,
        res_min_input,
        res_max_input,
        icc_text,
        icc_min_input,
        icc_max_input,
        se_text,
        se_min_input,
        se_max_input
    )
    .align_items(iced::Alignment::Center)
    .spacing(10);
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
    let col = column!(first_row, row_2, row_3, row)
        .align_items(iced::Alignment::Center)
        .spacing(20);
    container(col).height(Length::Fill).into()
}
