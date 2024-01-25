use iced::widget::{button, column, container, row};
use iced::{Command, Length, Renderer};

use crate::styles::container_styles::first_class_container_rounded_theme;
use crate::Pnselect;

use super::setting_pages::box_setting_view::box_setting_view;
use super::setting_pages::carton_setting_view::carton_setting_view;
use super::setting_pages::user_setting_view::user_setting_view;

#[derive(Clone, Debug)]
pub enum Message {
    BoxSettingAction,
    CartonSettingAction,
    UserSetingAction,
    SnLenChange(String),
    BoxMaxNumChange(String),
    BoxBtwNameChange(String),
    BoxPnNewChange(String),
    BoxSelected(Pnselect),
    BoxAddButton,
    BoxUpdateButton,
    CartonMaxNumChange(String),
    CartonBtwNameChange(String),
    CartonPnNewChange(String),
    CartonSelected(Pnselect),
    CartonAddButton,
    CartonUpdateButton,
    UserNameChange(String),
    UserPasswordChange(String),
    UserQxChange(String),
    UserAddButton,
    UserUpdateButton,
    BoxIthMinChange(String),
    BoxPoMinChange(String),
    BoxSenMinChange(String),
    BoxResMinChange(String),
    BoxVfMinChange(String),
    BoxImMinChange(String),
    BoxSeMinChange(String),
    BoxIccMinChange(String),
    BoxIthMaxChange(String),
    BoxPoMaxChange(String),
    BoxSenMaxChange(String),
    BoxResMaxChange(String),
    BoxVfMaxChange(String),
    BoxImMaxChange(String),
    BoxSeMaxChange(String),
    BoxIccMaxChange(String),
}
#[derive(Clone, Debug, Default)]
pub enum View {
    #[default]
    BoxStting,
    CartonSetting,
    UserSetting,
}
#[derive(Clone, Debug, Default)]
pub struct SettingView {
    view: View,
    pub sn_len: String,
    pub box_max_num: String,
    pub box_btw_name: String,
    pub box_pn_new: String,
    pub box_selected: Option<Pnselect>,
    pub box_selects: Vec<Pnselect>,
    pub carton_max_num: String,
    pub carton_btw_name: String,
    pub carton_pn_new: String,
    pub carton_selected: Option<Pnselect>,
    pub carton_selects: Vec<Pnselect>,
    pub user_name: String,
    pub user_password: String,
    pub user_qx: String,
    pub box_min_ith: String,
    pub box_min_po: String,
    pub box_min_sen: String,
    pub box_min_res: String,
    pub box_min_se: String,
    pub box_min_icc: String,
    pub box_min_vf: String,
    pub box_min_im: String,
    pub box_max_ith: String,
    pub box_max_po: String,
    pub box_max_sen: String,
    pub box_max_res: String,
    pub box_max_se: String,
    pub box_max_icc: String,
    pub box_max_vf: String,
    pub box_max_im: String,
}

impl SettingView {
    pub fn new() -> Self {
        Self {
            view: View::BoxStting,
            ..Default::default()
            // box_max_num: Default::default(),
            // box_btw_name: Default::default(),
            // box_selected: Default::default(),
            // box_selects: Default::default(),
            // box_pn_new: Default::default(),
            // carton_max_num: Default::default(),
            // carton_btw_name: Default::default(),
            // carton_pn_new: Default::default(),
            // carton_selected: Default::default(),
            // carton_selects: Default::default(),
            // user_name: todo!(),
            // user_password: todo!(),
            // user_qx: todo!(),
        }
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::BoxSettingAction => {
                self.view = View::BoxStting;
                Command::none()
            }
            Message::CartonSettingAction => {
                self.view = View::CartonSetting;
                Command::none()
            }
            Message::UserSetingAction => {
                self.view = View::UserSetting;
                Command::none()
            }
            Message::SnLenChange(l) => {
                self.sn_len = l;
                Command::none()
            }
            Message::BoxMaxNumChange(m) => {
                self.box_max_num = m;
                Command::none()
            }
            Message::BoxBtwNameChange(b) => {
                self.box_btw_name = b;
                Command::none()
            }
            Message::BoxSelected(s) => {
                self.box_selected = Some(s);
                Command::none()
            }
            Message::BoxPnNewChange(n) => {
                self.box_pn_new = n;
                Command::none()
            }
            Message::BoxAddButton => todo!(),
            Message::BoxUpdateButton => todo!(),
            Message::CartonMaxNumChange(m) => {
                self.carton_max_num = m;
                Command::none()
            }
            Message::CartonBtwNameChange(b) => {
                self.carton_btw_name = b;
                Command::none()
            }
            Message::CartonPnNewChange(p) => {
                self.carton_pn_new = p;
                Command::none()
            }
            Message::CartonSelected(c) => {
                self.carton_selected = Some(c);
                Command::none()
            }
            Message::CartonAddButton => todo!(),
            Message::CartonUpdateButton => todo!(),
            Message::UserNameChange(name) => {
                self.user_name = name;
                Command::none()
            }
            Message::UserPasswordChange(p) => {
                self.user_password = p;
                Command::none()
            }
            Message::UserQxChange(q) => {
                self.user_qx = q;
                Command::none()
            }
            Message::UserAddButton => todo!(),
            Message::UserUpdateButton => todo!(),
            Message::BoxIthMinChange(ith) => {
                self.box_min_ith = ith;
                Command::none()
            },
            Message::BoxPoMinChange(po) =>{
                self.box_min_po = po;
                Command::none()
            },
            Message::BoxSeMinChange(sen) => {
                self.box_min_sen = sen;
                Command::none()
            },
            Message::BoxResMinChange(res) => {
                self.box_min_res = res;
                Command::none()
            },
            Message::BoxVfMinChange(vf) => {
                self.box_min_vf = vf;
                Command::none()
            },
            Message::BoxImMinChange(im) => {
                self.box_min_im = im;
                Command::none()
            },
            Message::BoxIccMinChange(icc) => {
                self.box_min_icc = icc;
                Command::none()
            },
            Message::BoxIthMaxChange(ith) => {
                self.box_max_ith = ith;
                Command::none()
            },
            Message::BoxPoMaxChange(po) =>{
                self.box_max_po = po;
                Command::none()
            },
            Message::BoxSeMaxChange(sen) => {
                self.box_max_sen = sen;
                Command::none()
            },
            Message::BoxResMaxChange(res) => {
                self.box_max_res = res;
                Command::none()
            },
            Message::BoxVfMaxChange(vf) => {
                self.box_max_vf = vf;
                Command::none()
            },
            Message::BoxImMaxChange(im) => {
                self.box_max_im = im;
                Command::none()
            },
            Message::BoxIccMaxChange(icc) => {
                self.box_max_icc = icc;
                Command::none()
            },
            Message::BoxSenMinChange(sen) => {
                self.box_min_sen = sen;
                Command::none()
            },
            Message::BoxSenMaxChange(sen) => {
                self.box_max_sen = sen;
                Command::none()
            },
        }
    }

    pub fn view(&self) -> iced::Element<'_, Message, Renderer> {
        let box_s = button("装盒数据设置").on_press(Message::BoxSettingAction);
        let carton_s = button("装箱数据设置").on_press(Message::CartonSettingAction);
        let user_s = button("用户数据设置").on_press(Message::UserSetingAction);
        let first_row = row!(box_s, carton_s, user_s)
            .align_items(iced::Alignment::Center)
            .spacing(50);
        match self.view {
            View::BoxStting => {
                let view = box_setting_view(self);
                let col = column!(first_row, view).spacing(25);
                container(col).height(Length::Fill).padding(15).style(first_class_container_rounded_theme()).into()
            }
            View::CartonSetting => {
                let view = carton_setting_view(self);
                let col = column!(first_row, view).spacing(25);
                container(col).height(Length::Fill).padding(15).into()
            }
            View::UserSetting => {
                let view = user_setting_view(self);
                let col = column!(first_row, view).spacing(25);
                container(col).height(Length::Fill).padding(15).into()
            }
        }
    }
}
