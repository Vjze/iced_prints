use iced::{
    widget::{button, column, container, row, text, text_input, Container},
    Renderer,
};

use crate::views::setting::{Message, SettingView};

pub fn user_setting_view(state: &SettingView) -> Container<'_, Message, Renderer> {
    let id_text = text("工号:");
    
    let id_input =
        text_input("", &state.user_name).on_input(Message::UserNameChange);
    
    let password_text = text("密码:");
    let password_input = text_input("", &state.user_password).on_input(Message::UserPasswordChange);
    let qx_text = text("权限:");
    let qx_input = text_input("", &state.user_qx).on_input(Message::UserQxChange);
    let first_row = row!(
        id_text,
        id_input,
        password_text,
        password_input,
        qx_text,
        qx_input,
    )
    .align_items(iced::Alignment::Center)
    .spacing(25);
    let updata_btn = button("修改").on_press(Message::UserUpdateButton);
    let add_btn = button("新增").on_press(Message::UserAddButton);
    // if !state.carton_pn_new.is_empty()
    //     && !state.sn_len.is_empty()
    //     && !state.carton_max_num.is_empty()
    //     && !state.carton_btw_name.is_empty()
    //     && state.box_selected.is_none()
    // {
    //     add_btn = add_btn.on_press(Message::CartonAddButton)
    // }
    // if state.box_selected.is_some()
    //     && !state.sn_len.is_empty()
    //     && !state.carton_max_num.is_empty()
    //     && !state.carton_btw_name.is_empty()
    // {
    //     updata_btn = updata_btn.on_press(Message::CartonUpdateButton)
    // }
    let row = row!(updata_btn, add_btn)
        .align_items(iced::Alignment::Center)
        .spacing(30);
    let col = column!(first_row, row)
        .align_items(iced::Alignment::Center)
        .spacing(20);
    container(col).into()
}
