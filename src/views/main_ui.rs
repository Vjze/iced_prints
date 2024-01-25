use crate::{
    styles::{self, container_styles::release_time_container_theme},
    BoxConfigs,
    BoxDataInfos,
    BoxDatas,
    CartonConfigs,
    CartonDataInfos,
    Pnselect,
    TipType,
    User, // WindowState,
};

use iced::widget::scrollable;
use iced::{
    widget::{column, container},
    window, Application, Command, Element, Theme,
};

use super::{bar::bar, box_view::boxview, login::login, tip::tip};
use super::{
    box_view::BOX_MESSAGE_LOG,
    carton_view::{cartonview, CARTON_MESSAGE_LOG},
    setting::{Message as SettingMessage, SettingView},
};
#[derive(Debug, Default)]
pub struct MyTools {
    pub view: TabId,
    pub setting_view: SettingView,
    pub box_input_value: String,
    pub box_selected: Option<Pnselect>,
    pub box_selects: Vec<Pnselect>,
    pub show_modal: bool,
    pub logined: bool,
    pub user: User,
    pub tip_type: TipType,
    pub theme: bool,
    pub box_now_num: u32,
    pub box_list: Vec<BoxDataInfos>,
    pub order: String,
    pub box_configs: BoxConfigs,
    pub selected_carton: Option<Pnselect>,
    pub selects_carton: Vec<Pnselect>,
    pub carton_configs: CartonConfigs,
    pub carton_now_num: u32,
    pub carton_list: Vec<CartonDataInfos>,
    pub carton_input_value: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TabId {
    #[default]
    Box,
    Carton,
    Query,
    Unband,
    Setting,
}

#[derive(Clone, Debug)]
pub enum Message {
    BoxAddButton,
    BoxAddReback(Result<BoxDatas, TipType>),
    BoxPrintButton,
    BoxTextchange(String),
    BoxSelected(Pnselect),
    BoxCance,
    OpenModal,
    CloseModal,
    Logined(bool),
    Namechange(String),
    Passchange(String),
    Logingpre,
    LoginCence,
    Loginbool(bool),
    BoxAction,
    CartonAction,
    QueryAction,
    UnbandAction,
    SettingAction,
    Minwindow,
    // Maxwindow(WindowState),
    Closewindow,
    GetPns(Vec<Pnselect>),
    GetBoxConfigs(BoxConfigs),
    GetCartonConfigs(CartonConfigs),
    CartonSelected(Pnselect),
    CartonTextchange(String),
    CartonAddButton,
    CartonAddReback(bool),
    CartonPrintButton,
    CartonCance,
    Lout,
    SettingViewMessage(SettingMessage),
}

impl Application for MyTools {
    type Message = Message;
    type Theme = Theme;
    type Executor = tokio::runtime::Runtime;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                // view: Default::default(),
                // box_input_value: Default::default(),
                // box_selected: Default::default(),
                // show_modal: false,
                // logined: false,
                // user: Default::default(),
                // tip_type: Default::default(),
                // theme: Default::default(),
                // box_selects: Default::default(),
                // box_now_num: Default::default(),
                // box_list: Default::default(),
                // order: Default::default(),
                // box_configs: Default::default(),
                // selected_carton:  Default::default(),
                // selects_carton:  Default::default(),
                // carton_configs: Default::default(),
                // carton_now_num: Default::default(),
                // carton_list: Default::default(),
                // carton_input_value: Default::default(),
                setting_view: SettingView::new(),
                ..Default::default()
            },
            Command::perform(get_pns(), Message::GetPns),
        )
    }

    fn title(&self) -> String {
        String::from(format!(
            "{} - V {}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        ))
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::BoxAddButton => {
                if self.box_input_value.is_empty() {
                    self.show_modal = true;
                    self.tip_type = TipType::EmptyTip;
                    Command::none()
                } else if self.box_input_value.len() != self.box_configs.max_sn.try_into().unwrap()
                {
                    self.show_modal = true;
                    self.tip_type = TipType::LenErr;
                    self.box_input_value.clear();
                    Command::none()
                } else {
                    let sn = self.box_input_value.clone();
                    Command::perform(box_add(sn), Message::BoxAddReback)
                }
            }
            Message::BoxTextchange(value) => {
                self.box_input_value = value;
                Command::none()
            }
            Message::OpenModal => {
                self.show_modal = true;
                Command::none()
            }
            Message::CloseModal => {
                self.show_modal = false;
                Command::none()
            }
            Message::Logined(value) => {
                self.logined = value;
                Command::none()
            }
            Message::Namechange(value) => {
                self.user.name = value;
                Command::none()
            }
            Message::Passchange(value) => {
                self.user.password = value;
                Command::none()
            }
            Message::Logingpre => {
                let name = self.user.name.clone();
                let password = self.user.password.clone();
                if name == "z" && password == "0" {
                    self.logined = true;
                    self.user.qx = "工程".to_string();
                } else if name == "x" && password == "1" {
                    self.logined = true;
                    self.user.qx = "作业员".to_string();
                } else {
                    self.logined = false;
                    self.show_modal = true;
                    self.tip_type = TipType::LoginTip;
                }
                Command::none()
            }
            Message::LoginCence => {
                self.logined = false;
                window::close()
            }
            Message::Loginbool(bool) => {
                self.logined = bool;
                if !self.logined {
                    self.show_modal = true;
                    self.tip_type = TipType::LoginTip;
                }
                Command::none()
            }
            Message::QueryAction => {
                self.view = TabId::Query;
                Command::none()
            }
            Message::BoxAction => {
                self.view = TabId::Box;
                Command::none()
            }
            Message::CartonAction => {
                self.view = TabId::Carton;
                Command::none()
            }
            Message::UnbandAction => {
                self.view = TabId::Unband;
                Command::none()
            }
            Message::Minwindow => window::minimize(true),
            // Message::Maxwindow(value) => match value {
            //     WindowState::Max => {
            //         window::resize(iced::Size{width: 1650, height: 800 })
            //     },
            //     WindowState::Default => {
            //         window::maximize(true)
            //     },
            // },
            Message::Closewindow => window::close(),
            Message::BoxSelected(s) => {
                self.box_selected = Some(s);
                let pn = self.box_selected.clone().unwrap().pn;
                self.box_input_value.clear();
                self.box_list.clear();
                self.box_now_num = 0;
                Command::perform(get_box_configs(pn), Message::GetBoxConfigs)
            }
            Message::BoxPrintButton => todo!(),
            Message::BoxAddReback(b) => match b {
                Ok(res) => {
                    if res.box_po < self.box_configs.box_min_po
                        || res.box_po > self.box_configs.box_max_po
                    {
                        self.tip_type = TipType::PoErr;
                        self.show_modal = true;
                        Command::none()
                    } else if res.box_sen < self.box_configs.box_min_sen
                        || res.box_sen > self.box_configs.box_max_sen
                    {
                        self.tip_type = TipType::SenErr;
                        self.show_modal = true;
                        Command::none()
                    } else if res.box_res < self.box_configs.box_min_res
                        || res.box_res > self.box_configs.box_max_res
                    {
                        self.tip_type = TipType::ResErr;
                        self.show_modal = true;
                        Command::none()
                    } else if res.box_vf < self.box_configs.box_min_vf
                        || res.box_vf > self.box_configs.box_max_vf
                    {
                        self.tip_type = TipType::VfErr;
                        self.show_modal = true;
                        Command::none()
                    } else if res.box_im < self.box_configs.box_min_im
                        || res.box_im > self.box_configs.box_max_im
                    {
                        self.tip_type = TipType::ImErr;
                        self.show_modal = true;
                        Command::none()
                    } else if res.box_ith < self.box_configs.box_min_ith
                        || res.box_ith > self.box_configs.box_max_ith
                    {
                        self.tip_type = TipType::IthErr;
                        self.show_modal = true;
                        Command::none()
                    } else if res.box_se < self.box_configs.box_min_se
                        || res.box_se > self.box_configs.box_max_se
                    {
                        self.tip_type = TipType::SeErr;
                        self.show_modal = true;
                        Command::none()
                    } else if res.box_icc < self.box_configs.box_min_icc
                        || res.box_icc > self.box_configs.box_max_icc
                    {
                        self.tip_type = TipType::IccErr;
                        self.show_modal = true;
                        Command::none()
                    } else {
                        self.order = res.order;
                        let id = self.box_now_num.clone();
                        let sn = self.box_input_value.clone();
                        let order = self.order.clone();
                        let worker = self.user.name.clone();
                        let data = BoxDataInfos {
                            id,
                            sn,
                            order,
                            worker,
                        };
                        self.box_list.push(data);

                        self.box_now_num += 1;
                        self.box_input_value.clear();
                        scrollable::snap_to(
                            BOX_MESSAGE_LOG.clone(),
                            scrollable::RelativeOffset::END,
                        )
                    }
                }
                Err(e) => {
                    self.tip_type = e;
                    self.show_modal = true;
                    Command::none()
                }
            },
            Message::GetBoxConfigs(v) => {
                self.box_configs = v;
                Command::none()
            }
            Message::GetPns(v) => {
                self.box_selects = v.clone();
                self.selects_carton = v;
                Command::none()
            }
            Message::CartonSelected(s) => {
                self.selected_carton = Some(s);
                let pn = self.selected_carton.clone().unwrap().pn;
                self.carton_input_value.clear();
                self.carton_list.clear();
                self.carton_now_num = 0;
                Command::perform(get_carton_configs(pn), Message::GetCartonConfigs)
            }
            Message::CartonTextchange(s) => {
                self.carton_input_value = s;
                Command::none()
            }
            Message::GetCartonConfigs(s) => {
                self.carton_configs = s;
                Command::none()
            }
            Message::CartonAddButton => {
                if self.carton_input_value.is_empty() {
                    self.show_modal = true;
                    self.tip_type = TipType::EmptyTip;
                    Command::none()
                } else {
                    let sn = self.carton_input_value.clone();
                    Command::perform(carton_add(sn), Message::CartonAddReback)
                }
            }
            Message::CartonAddReback(b) => match b {
                true => {
                    let id = self.carton_now_num.clone();
                    let box_sn = self.carton_input_value.clone();
                    let order = self.order.clone();
                    let worker = self.user.name.clone();
                    let data = CartonDataInfos {
                        id,
                        box_sn,
                        order,
                        worker,
                    };
                    self.carton_list.push(data);

                    self.carton_now_num += 1;
                    self.carton_input_value.clear();
                    scrollable::snap_to(CARTON_MESSAGE_LOG.clone(), scrollable::RelativeOffset::END)
                }
                false => todo!(),
            },
            Message::CartonPrintButton => todo!(),
            Message::Lout => {
                self.user.name.clear();
                self.user.password.clear();
                self.user.qx.clear();
                self.logined = false;
                self.carton_input_value.clear();
                self.carton_list.clear();
                self.carton_now_num = 0;
                self.box_input_value.clear();
                self.box_list.clear();
                self.box_now_num = 0;
                Command::none()
            }
            Message::SettingAction => {
                self.view = TabId::Setting;
                Command::none()
            }
            Message::SettingViewMessage(msg) => self
                .setting_view
                .update(msg)
                .map(Message::SettingViewMessage),
            Message::BoxCance => {
                self.box_input_value.clear();
                self.box_list.clear();
                self.box_now_num = 0;
                Command::none()
            }
            Message::CartonCance => {
                self.carton_input_value.clear();
                self.carton_list.clear();
                self.carton_now_num = 0;
                Command::none()
            }
        }
    }
    // fn subscription(&self) -> Subscription<Message> {
    //     iced::time::every(std::time::Duration::from_millis(1000)).map(|_| {
    //         Message::Datetime(
    //             time::OffsetDateTime::now_local()
    //                 .unwrap_or_else(|_| time::OffsetDateTime::now_utc()),
    //         )
    //     })
    // }
    fn view(&self) -> Element<'_, Self::Message> {
        let bar = bar(self);
        let mian_view = match self.view {
            TabId::Box => container(boxview(self)),
            TabId::Carton => container(cartonview(self)),
            TabId::Query => todo!(),
            TabId::Unband => todo!(),
            TabId::Setting => container(self.setting_view.view().map(Message::SettingViewMessage)),
        };

        let login = login(self);
        let body = match self.tip_type {
            TipType::EmptyTip => "输入框不能为空,请输入SN再进行绑定!",
            TipType::LoginTip => "登录异常，工号不存在或者密码错误，请确认后再进行登录!",
            TipType::Cm => "当前SN已绑定过其它盒号!",
            TipType::Hl => "当前SN料号和选择料号不同,请先进行确认!",
            TipType::Notest => "当前SN没有测试数据,无法进行绑定!",
            TipType::PoErr => "当前SN的po测试数据不符合要求,请确认！",
            TipType::LenErr => "SN长度与系统设置长度不符,请确认SN是否正常!",
            TipType::IthErr => "当前SN的Ith测试数据不符合要求,请确认！",
            TipType::VfErr => "当前SN的Vf测试数据不符合要求,请确认！",
            TipType::ImErr => "当前SN的Im测试数据不符合要求,请确认！",
            TipType::SenErr => "当前SN的Sen测试数据不符合要求,请确认！",
            TipType::ResErr => "当前SN的Res测试数据不符合要求,请确认！",
            TipType::SeErr => "当前SN的Se测试数据不符合要求,请确认！",
            TipType::IccErr => "当前SN的Icc测试数据不符合要求,请确认！",
        };
        let all_tip = tip(self, login, body);
        let col = column![bar, mian_view, all_tip];
        container(col)
            .padding(5)
            .style(release_time_container_theme())
            .into()
    }
    fn theme(&self) -> Self::Theme {
        let custom_theme = Box::new(
            match self.theme {
                true => styles::theme::TroxideTheme::Dark,
                false => styles::theme::TroxideTheme::Light,
            }
            .get_custom_theme(),
        );
        iced::Theme::Custom(custom_theme)
    }
}
async fn box_add(sn: String) -> Result<BoxDatas, TipType> {
    let data = if sn == "1" {
        BoxDatas {
            box_ith: 8.5,
            box_po: 1800,
            box_sen: -29.5,
            box_res: 0.75,
            box_se: 1.234,
            box_icc: 32.85,
            box_vf: 1.25,
            box_im: 22.33,
            order: "240102".to_owned(),
        }
    } else if sn == "a" {
        return Err(TipType::Hl);
    } else if sn == "s" {
        return Err(TipType::Notest);
    } else {
        BoxDatas {
            box_ith: 8.4,
            box_po: 1801,
            box_sen: -29.5,
            box_res: 0.75,
            box_se: 1.234,
            box_icc: 32.85,
            box_vf: 1.25,
            box_im: 22.33,
            order: "240102".to_owned(),
        }
    };
    Ok(data)
}
async fn carton_add(_sn: String) -> bool {
    true
}
async fn get_pns() -> Vec<Pnselect> {
    let configs = vec![
        Pnselect {
            pn: "30300010".to_string(),
        },
        Pnselect {
            pn: "30300011".to_string(),
        },
        Pnselect {
            pn: "30300012".to_string(),
        },
        Pnselect {
            pn: "30300013".to_string(),
        },
    ];
    configs
}
async fn get_box_configs(pn: String) -> BoxConfigs {
    let configs = if pn == "30300010" {
        BoxConfigs {
            max_num: 2,
            max_sn: 1,
            pn,
            box_min_ith: 8.5,
            box_min_po: 1800,
            box_min_sen: -29.5,
            box_min_res: 0.75,
            box_min_se: 1.234,
            box_min_icc: 32.85,
            box_min_vf: 1.25,
            box_min_im: 22.33,
            box_max_ith: 12.5,
            box_max_po: 1800,
            box_max_sen: -28.5,
            box_max_res: 1.75,
            box_max_se: 1.534,
            box_max_icc: 42.85,
            box_max_vf: 1.35,
            box_max_im: 25.33,
        }
    } else if pn == "30300011" {
        BoxConfigs {
            max_num: 120,
            max_sn: 15,
            pn,
            box_min_ith: 8.5,
            box_min_po: 1800,
            box_min_sen: -29.5,
            box_min_res: 0.75,
            box_min_se: 1.234,
            box_min_icc: 32.85,
            box_min_vf: 1.25,
            box_min_im: 22.33,
            box_max_ith: 12.5,
            box_max_po: 1800,
            box_max_sen: -28.5,
            box_max_res: 1.75,
            box_max_se: 1.534,
            box_max_icc: 42.85,
            box_max_vf: 1.35,
            box_max_im: 25.33,
        }
    } else if pn == "30300012" {
        BoxConfigs {
            max_num: 130,
            max_sn: 16,
            pn,
            box_min_ith: 8.5,
            box_min_po: 1800,
            box_min_sen: -29.5,
            box_min_res: 0.75,
            box_min_se: 1.234,
            box_min_icc: 32.85,
            box_min_vf: 1.25,
            box_min_im: 22.33,
            box_max_ith: 12.5,
            box_max_po: 1800,
            box_max_sen: -28.5,
            box_max_res: 1.75,
            box_max_se: 1.534,
            box_max_icc: 42.85,
            box_max_vf: 1.35,
            box_max_im: 25.33,
        }
    } else {
        BoxConfigs {
            max_num: 140,
            max_sn: 18,
            pn,
            box_min_ith: 8.5,
            box_min_po: 1800,
            box_min_sen: -29.5,
            box_min_res: 0.75,
            box_min_se: 1.234,
            box_min_icc: 32.85,
            box_min_vf: 1.25,
            box_min_im: 22.33,
            box_max_ith: 12.5,
            box_max_po: 1800,
            box_max_sen: -28.5,
            box_max_res: 1.75,
            box_max_se: 1.534,
            box_max_icc: 42.85,
            box_max_vf: 1.35,
            box_max_im: 25.33,
        }
    };
    configs
}
async fn get_carton_configs(pn: String) -> CartonConfigs {
    let configs = if pn == "30300010" {
        CartonConfigs { max_num: 2, pn }
    } else if pn == "30300011" {
        CartonConfigs { max_num: 1200, pn }
    } else if pn == "30300012" {
        CartonConfigs { max_num: 1300, pn }
    } else {
        CartonConfigs { max_num: 1400, pn }
    };
    configs
}
