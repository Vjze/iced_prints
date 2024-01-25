
use crate::logos::IMG_LOGO_LIGHT;
use crate::styles::button_styles::{
    transparent_button_theme, transparent_button_with_rounded_border_theme,
};

use super::main_ui::{Message, MyTools};
use iced::widget::image;

use iced::{
    widget::{button, container, image::Handle, row, text, Container, Space},
    Length, Renderer,
};
pub fn bar(state: &MyTools) -> Container<'static, Message, Renderer> {
    
    let min = button("-")
        .on_press(Message::Minwindow)
        .style(transparent_button_theme());
    // let max = button("口")
    //     .on_press(Message::Maxwindow)
    //     .style(transparent_button_theme());
    let close = button("X")
        .on_press(Message::Closewindow)
        .style(transparent_button_theme());
    let logo = image(Handle::from_memory(IMG_LOGO_LIGHT)).height(Length::Shrink);
    let title = text("深圳市讯通光通信有限公司").size(40.0);
    let query_button = button("装盒扫描")
        .on_press(Message::BoxAction)
        .style(transparent_button_with_rounded_border_theme());
    let bandin_button = button("装箱扫描")
        .on_press(Message::CartonAction)
        .style(transparent_button_with_rounded_border_theme());
    let order_button = button("数据查询")
        .on_press(Message::QueryAction)
        .style(transparent_button_with_rounded_border_theme());
    let mut unband_button = button("数据解绑")
        
        .style(transparent_button_with_rounded_border_theme());
    let mut setting_button = button("数据设置")
        
        .style(transparent_button_with_rounded_border_theme());
    if state.user.qx == "工程"{
        unband_button = unband_button.on_press(Message::UnbandAction);
        setting_button = setting_button.on_press(Message::SettingAction)
    }
    let user = text(format!("当前用户：{},权限组：{}",state.user.name,state.user.qx));
    let lout = button("退出账号").on_press(Message::Lout);
    let bar = row![
        logo,
        title,
        Space::new(Length::Fill, Length::Shrink),
        query_button,
        bandin_button,
        order_button,
        unband_button,
        setting_button,
        Space::new(Length::Fill, Length::Shrink),
        user,
        lout,
        Space::new(Length::Fill, Length::Shrink),
        min,
        // max,
        close,
    ]
    .spacing(20)
    .align_items(iced::Alignment::Center);
    container(bar).max_height(60.0).padding(5).into()
}
