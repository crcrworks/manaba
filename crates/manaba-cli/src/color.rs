#![allow(unused)]
use crate::APP_CONFIG;
use colored_text::Colorize;
use std::{fmt, sync::OnceLock};

pub static APP_COLOR: OnceLock<AppColor> = OnceLock::new();

pub struct AppColor {
    pub white: String,
    pub black: String,
    pub red: String,
    pub blue: String,
    pub aqua: String,
    pub yellow: String,
    pub green: String,
    pub gray: String,
}

impl Default for AppColor {
    fn default() -> Self {
        // From Everforest color theme
        Self {
            white: "D3C6AA".into(),
            black: "272E33".into(),
            red: "E67E80".into(),
            blue: "7FBBB3".into(),
            aqua: "83C092".into(),
            yellow: "DBBC7F".into(),
            green: "A7C080".into(),
            gray: "4F5B58".into(),
        }
    }
}

pub trait AppColorize {
    fn white(&self) -> String;
    fn black(&self) -> String;
    fn red(&self) -> String;
    fn blue(&self) -> String;
    fn aqua(&self) -> String;
    fn yellow(&self) -> String;
    fn green(&self) -> String;
    fn gray(&self) -> String;

    fn on_white(&self) -> String;
    fn on_black(&self) -> String;
    fn on_red(&self) -> String;
    fn on_blue(&self) -> String;
    fn on_aqua(&self) -> String;
    fn on_yellow(&self) -> String;
    fn on_green(&self) -> String;
    fn on_gray(&self) -> String;

    fn with_bold(&self) -> String;
}

impl<T: fmt::Display> AppColorize for T {
    fn white(&self) -> String {
        let color = &APP_COLOR.get_or_init(Default::default).white;
        self.hex(color)
    }

    fn black(&self) -> String {
        let black = &APP_COLOR.get_or_init(Default::default).black;
        self.hex(black)
    }

    fn red(&self) -> String {
        let red = &APP_COLOR.get_or_init(Default::default).red;
        self.hex(red)
    }

    fn blue(&self) -> String {
        let blue = &APP_COLOR.get_or_init(Default::default).red;
        self.hex(blue)
    }

    fn aqua(&self) -> String {
        let aqua = &APP_COLOR.get_or_init(Default::default).aqua;
        self.hex(aqua)
    }

    fn yellow(&self) -> String {
        let yellow = &APP_COLOR.get_or_init(Default::default).yellow;
        self.hex(yellow)
    }

    fn green(&self) -> String {
        let green = &APP_COLOR.get_or_init(Default::default).green;
        self.hex(green)
    }

    fn gray(&self) -> String {
        let gray = &APP_COLOR.get_or_init(Default::default).gray;
        self.hex(gray)
    }

    fn on_white(&self) -> String {
        let white = &APP_COLOR.get_or_init(Default::default).white;
        self.on_hex(white)
    }

    fn on_black(&self) -> String {
        let black = &APP_COLOR.get_or_init(Default::default).black;
        self.on_hex(black)
    }

    fn on_red(&self) -> String {
        let red = &APP_COLOR.get_or_init(Default::default).red;
        self.on_hex(red)
    }

    fn on_blue(&self) -> String {
        let blue = &APP_COLOR.get_or_init(Default::default).blue;
        self.on_hex(blue)
    }

    fn on_aqua(&self) -> String {
        let aqua = &APP_COLOR.get_or_init(Default::default).aqua;
        self.on_hex(aqua)
    }

    fn on_yellow(&self) -> String {
        let yellow = &APP_COLOR.get_or_init(Default::default).yellow;
        self.on_hex(yellow)
    }

    fn on_green(&self) -> String {
        let green = &APP_COLOR.get_or_init(Default::default).green;
        self.on_hex(green)
    }

    fn on_gray(&self) -> String {
        let gray = &APP_COLOR.get_or_init(Default::default).gray;
        self.on_hex(gray)
    }

    fn with_bold(&self) -> String {
        self.bold()
    }
}
