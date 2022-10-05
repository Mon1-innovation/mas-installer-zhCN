/// The module with styles for our app

use std::sync::Mutex;

use fltk::{
    enums::{
        Color,
        Font
    },
    image::PngImage
};

use lazy_static::lazy_static;


// App title
pub const WIN_TITLE: &str = "Monika After Story 安装器";
// Err window title
pub const ALERT_WIN_TITLE: &str = "错误!";
// Msg window title
pub const MSG_WIN_TITLE: &str = "注意!";


// Window consts
pub const WIN_WIDTH: i32 = 600;
pub const WIN_HEIGHT: i32 = 500;

pub const WIN_PADDING: i32 = 4;

pub const INNER_WIN_WIDTH: i32 = WIN_WIDTH - 2*WIN_PADDING;
pub const INNER_WIN_HEIGHT: i32 = WIN_HEIGHT - 2*WIN_PADDING;

pub const ALERT_WIN_WIDTH: i32 = 500;
pub const ALERT_WIN_HEIGHT: i32 = 200;

pub const INNER_ALERT_WIN_WIDTH: i32 = ALERT_WIN_WIDTH - 2*WIN_PADDING;
pub const INNER_ALERT_WIN_HEIGHT: i32 = ALERT_WIN_HEIGHT - 2*WIN_PADDING;

pub const MSG_WIN_WIDTH: i32 = ALERT_WIN_WIDTH;
pub const MSG_WIN_HEIGHT: i32 = ALERT_WIN_HEIGHT;

pub const INNER_MSG_WIN_WIDTH: i32 = MSG_WIN_WIDTH - 2*WIN_PADDING;
pub const INNER_MSG_WIN_HEIGHT: i32 = MSG_WIN_HEIGHT - 2*WIN_PADDING;

pub const INNER_WIN_CONTENT_XPADDING: i32 = 20;
pub const INNER_WIN_CONTENT_YPADDING: i32 = INNER_WIN_CONTENT_XPADDING;


// Buttons consts
pub const BUT_WIDTH: i32 = 130;
pub const BUT_HEIGHT: i32 = 35;

pub const BUT_MUTE_WIDTH: i32 = BUT_HEIGHT;
pub const BUT_MUTE_HEIGHT: i32 = BUT_MUTE_WIDTH;

pub const BUT_DLX_VER_CHECK_WIDTH: i32 = BUT_WIDTH + 225;
pub const BUT_DLX_VER_CHECK_HEIGHT: i32 = BUT_HEIGHT;

pub const BUT_INSTALL_SPR_CHECK_WIDTH: i32 = BUT_WIDTH + 380;
pub const BUT_INSTALL_SPR_CHECK_HEIGHT: i32 = BUT_HEIGHT;

// pub const BUT_CREDITS_WIDTH: i32 = BUT_WIDTH;
// pub const BUT_CREDITS_HEIGHT: i32 = BUT_HEIGHT;

// padding of the frame within buttons
pub const BUT_PADDING: i32 = 3;
// Spacing between teh buttons
pub const BUT_SPACING: i32 = 5;

pub const BUT_FONT_SIZE: i32 = 16;
pub const BUT_FONT: Font = Font::HelveticaBold;

pub const BUT_ABORT_LABEL: &str = "取消";
pub const BUT_BACK_LABEL: &str = "@< 返回 ";
pub const BUT_CONTINUE_LABEL: &str = " 继续@>";
pub const BUT_SELECT_DIR_LABEL: &str = "浏览 @fileopen";
pub const BUT_DLX_VER_CHECK_LABEL: &str = "豪华版 (预安装精灵包)";
pub const BUT_INSTALL_SPR_CHECK_LABEL: &str = "下载精灵包 (另外下载到 '/spritepacks')";
pub const BUT_INSTALL_LABEL: &str = "安装";
pub const BUT_OK_LABEL: &str = "确认";
pub const BUT_EXIT_LABEL: &str = "退出";
pub const BUT_CREDITS_LABEL: &str = "MyNewSoundtrack\nYouTube 频道";
pub const BUT_CHANGELOG_LABEL: &str = "更新日志";

pub const BUT_ALERT_WIN_PADDING: i32 = 10;
pub const BUT_MSG_WIN_PADDING: i32 = BUT_ALERT_WIN_PADDING;
pub const BUT_PACK_YPADDING: i32 = INNER_WIN_CONTENT_YPADDING;


// Frame consts
pub const TOP_FRAME_LABEL_SIZE: i32 = LABEL_SIZE_LARGE;
pub const TOP_FRAME_XPOS: i32 = 0;
pub const TOP_FRAME_YPOS: i32 = INNER_WIN_CONTENT_YPADDING;
pub const TOP_FRAME_WIDTH: i32 = INNER_WIN_WIDTH;
pub const TOP_FRAME_HEIGHT: i32 = 35;

pub const MID_FRAME_XPOS: i32 = TOP_FRAME_XPOS;
pub const MID_FRAME_YPOS: i32 = 2*TOP_FRAME_YPOS + TOP_FRAME_HEIGHT;
pub const MID_FRAME_WIDTH: i32 = TOP_FRAME_WIDTH;
pub const MID_FRAME_HEIGHT: i32 = INNER_WIN_HEIGHT - MID_FRAME_YPOS - BUT_HEIGHT - 2*INNER_WIN_CONTENT_YPADDING;
pub const MID_FRAME_LABEL_SIZE: i32 = TOP_FRAME_LABEL_SIZE;

pub const MSG_FRAME_LABEL_SIZE: i32 = LABEL_SIZE_MED;

pub const CREDITS_FRAME_XPOS: i32 = 0;
pub const CREDITS_FRAME_YPOS: i32 = INNER_WIN_HEIGHT - CREDITS_FRAME_HEIGHT;
pub const CREDITS_FRAME_WIDTH: i32 = INNER_WIN_WIDTH;
pub const CREDITS_FRAME_HEIGHT: i32 = 20;
pub const CREDITS_FRAME_LABEL_SIZE: i32 = 14;


// Text display constants
pub const TXT_DISP_XPOS: i32 = INNER_WIN_CONTENT_XPADDING;
pub const TXT_DISP_YPOS: i32 = MID_FRAME_YPOS;
pub const TXT_DISP_WIDTH: i32 = INNER_WIN_WIDTH - 2*INNER_WIN_CONTENT_XPADDING;
pub const TXT_DISP_HEIGHT: i32 = MID_FRAME_HEIGHT;


// Text consts
pub const SEL_DIR_TXT_XPOS: i32 = INNER_WIN_CONTENT_XPADDING;
pub const SEL_DIR_TXT_YPOS: i32 = INNER_WIN_HEIGHT/2 - SEL_DIR_TXT_HEIGHT - BUT_SPACING/2;
pub const SEL_DIR_TXT_WIDTH: i32 = INNER_WIN_WIDTH - 2*INNER_WIN_CONTENT_XPADDING;
pub const SEL_DIR_TXT_HEIGHT: i32 = 28;
pub const SEL_DIR_TXT_SIZE: i32 = 18;

pub const SEL_DIR_DLG_PROMPT: &str = "选择一个 Doki Doki Literature Club 文件夹";

pub const LABEL_SIZE_LARGE: i32 = 28;
pub const LABEL_SIZE_MED: i32 = 20;


// Progress bar consts
pub const PB_WIDTH: i32 = INNER_WIN_WIDTH - 2*INNER_WIN_CONTENT_XPADDING;
pub const PB_HEIGHT: i32 = BUT_HEIGHT;


// Slider consts
pub const SCROLL_AMOUNT: f64 = 3.0;
// The number of characters to ignore by the slider,
// there doesn't appear to be a better way, this value works for us
// so we hard-code it
pub const LICENSE_SLIDER_LINES_IGNORE: i32 = 910;
pub const LICENSE_SLIDER_WIDTH: i32 = INNER_WIN_CONTENT_XPADDING;
pub const LICENSE_SLIDER_HEIGHT: i32 = TXT_DISP_HEIGHT;
pub const LICENSE_SLIDER_SIZE: f32 = 0.2;


// Color constants
pub const C_BLACK: Color = Color::Black;
pub const C_WHITE: Color = Color::White;

pub const C_DDLC_PEACH: Color = Color::from_hex(0xffaa99);
pub const C_DDLC_WHITE_IDLE: Color = Color::from_hex(0xffe6f4);
pub const C_DDLC_PINK_IDLE: Color = Color::from_hex(0xffbde1);
pub const C_DDLC_PINK_DARK: Color = Color::from_hex(0xbb5599);

pub const C_DDLC_WHITE_ACT: Color = Color::from_hex(0xffffff);
pub const C_DDLC_PINK_ACT: Color = C_DDLC_PINK_IDLE;

pub const C_BRIGHT_GREEN: Color = Color::from_hex(0x00ff00);


// Raw text
pub const WELCOME_TOP_FRAME_LABEL: &str = "欢迎使用 MAS 安装器";
pub const WELCOME_MID_FRAME_LABEL: &str = concat!(
    "这个程序将自动安装\n",
    "Monika After Story mod 在你的PC上",
);
pub const LICENSE_FRAME_LABEL: &str = "您需要同意我们的条款才能继续";
pub const SELECT_DIR_FRAME_LABEL: &str = "选择一个 Doki Doki Literature Club 文件夹";
pub const OPTIONS_FRAME_LABEL: &str = "设置其它安装项";
pub const PROGRESS_FRAME_LABEL: &str = "正在安装, 请稍后...";
pub const ABORT_TOP_FRAME_LABEL: &str = "已取消";
pub const ABORT_MID_FRAME_LABEL: &str = concat!(
    "安装已经取消.\n",
    "任何已被安装的文件将被保留"
);
pub const DONE_TOP_FRAME_LABEL: &str = "Finished";
pub const DONE_MID_FRAME_LABEL: &str = concat!(
    "Monika After Story 已经成功安装在\n",
    "你的电脑上"
);
pub const CREDITS_FRAME_LABEL: &str = "安装器主题 by MyNewSoundtrack";


// Define images
lazy_static! {
    pub static ref VOLUME_BUT_CHECK_IMG: Mutex<PngImage> = Mutex::new(
        PngImage::from_data(crate::static_data::VOLUME_BUT_CHECK_DATA).unwrap()
    );
    pub static ref VOLUME_BUT_CHECK_HOVER_IMG: Mutex<PngImage> = Mutex::new(
        PngImage::from_data(crate::static_data::VOLUME_BUT_CHECK_HOVER_DATA).unwrap()
    );
    pub static ref VOLUME_BUT_UNCHECK_IMG: Mutex<PngImage> = Mutex::new(
        PngImage::from_data(crate::static_data::VOLUME_BUT_UNCHECK_DATA).unwrap()
    );
    pub static ref VOLUME_BUT_UNCHECK_HOVER_IMG: Mutex<PngImage> = Mutex::new(
        PngImage::from_data(crate::static_data::VOLUME_BUT_UNCHECK_HOVER_DATA).unwrap()
    );
}
