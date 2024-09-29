use lazy_static::lazy_static;
use std::sync::Mutex;
use tauri::{CustomMenuItem, Menu, Submenu};

lazy_static! {
    static ref CURRENT_LANG: Mutex<String> = Mutex::new("en".to_string());
}

struct Language {
    quit: BilingualMenuItem,
    file: BilingualMenuItem,
    open_file: BilingualMenuItem,
}

impl Language {
    fn new() -> Self {
        Language {
            file: BilingualMenuItem::new("File", "文件"),
            quit: BilingualMenuItem::new("Quit", "退出"),
            open_file: BilingualMenuItem::new("Open File...", "打开文件..."),
        }
    }
}

struct BilingualMenuItem {
    en: String,
    zh: String,
}

impl BilingualMenuItem {
    fn new(en: &str, zh: &str) -> Self {
        BilingualMenuItem {
            en: en.to_string(),
            zh: zh.to_string(),
        }
    }

    fn get_name(&self, lang: &str) -> String {
        match lang {
            "zh" => self.zh.clone(),
            _ => self.en.clone(),
        }
    }
}

pub fn window_menu() -> Menu {
    let lang = CURRENT_LANG.lock().unwrap().clone();
    let lang_str = lang.as_str(); // 将 lang 转换为 &str
    let language = Language::new();

    let quit_item = CustomMenuItem::new("quit", language.quit.get_name(lang_str));
    let close_item = CustomMenuItem::new("open_file", language.open_file.get_name(lang_str));

    let submenu = Submenu::new(
        language.file.get_name(lang_str),
        Menu::new().add_item(quit_item).add_item(close_item),
    );

    let zh_menu_item = if lang.as_str() == "zh" {
        CustomMenuItem::new("zh_menu", "中文").selected()
    } else {
        CustomMenuItem::new("zh_menu", "中文")
    };
     zh_menu_item

    let en_menu_item = if lang.as_str() == "en" {
        CustomMenuItem::new("en_menu", "English").selected()
    } else {
        CustomMenuItem::new("en_menu", "English")
    };
    let lang_submenu = Submenu::new(
        "Language",
        Menu::new().add_item(zh_menu_item).add_item(en_menu_item),
    );

    let menu = Menu::new().add_submenu(submenu).add_submenu(lang_submenu);

    return menu;
}

pub fn toggle_language() {
    let mut lang = CURRENT_LANG.lock().unwrap();
    if *lang == "en" {
        *lang = "zh".to_string();
    } else {
        *lang = "en".to_string();
    }
}
