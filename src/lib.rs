// pub mod util;
pub mod styles;
pub mod views;
pub mod logos {
    // pub static IMG_LOGO_DARK: &[u8] = include_bytes!("../resources/logo/logo_dark.png");
    pub static IMG_LOGO_LIGHT: &[u8] = include_bytes!("../resources/logo/logo.png");
}
#[derive(Debug, Clone, Copy, Default)]
pub enum TipType {
    #[default]
    EmptyTip,
    LoginTip,
    Cm,
    Hl,
    Notest,
    PoErr,
    LenErr,
    IthErr,
    VfErr,
    ImErr,
    SenErr,
    ResErr,
    SeErr,
    IccErr,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Pnselect {
    pub(crate) pn: String,
}
impl Default for Pnselect {
    fn default() -> Self {
        Self {
            pn: Default::default(),
        }
    }
}
impl std::fmt::Display for Pnselect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.pn)
    }
}
#[derive(Debug, Clone, Default)]
pub struct BoxDataInfos {
    pub(crate) id: u32,
    pub(crate) sn: String,
    pub(crate) order: String,
    pub(crate) worker: String,
}
#[derive(Debug, Clone, Default)]
pub struct CartonDataInfos {
    pub(crate) id: u32,
    pub(crate) box_sn: String,
    pub(crate) order: String,
    pub(crate) worker: String,
}
#[derive(Debug, Clone, Default)]
pub struct BoxConfigs {
    max_sn: u32,
    max_num: u32,
    pn: String,
    pub box_min_ith: f64,
    pub box_min_po: u32,
    pub box_min_sen: f64,
    pub box_min_res: f64,
    pub box_min_se: f64,
    pub box_min_icc: f64,
    pub box_min_vf: f64,
    pub box_min_im: f64,
    pub box_max_ith: f64,
    pub box_max_po: u32,
    pub box_max_sen: f64,
    pub box_max_res: f64,
    pub box_max_se: f64,
    pub box_max_icc: f64,
    pub box_max_vf: f64,
    pub box_max_im: f64,

}
#[derive(Debug, Clone, Default)]
pub struct CartonConfigs {
    max_num: u32,
    pn: String,
}
#[derive(Debug, Clone, Default)]
pub struct User {
    name: String,
    password: String,
    qx: String,
}
#[derive(Debug, Clone, Default)]
pub struct BoxDatas {
    pub order: String,
    pub box_ith: f64,
    pub box_po: u32,
    pub box_sen: f64,
    pub box_res: f64,
    pub box_se: f64,
    pub box_icc: f64,
    pub box_vf: f64,
    pub box_im: f64,

}