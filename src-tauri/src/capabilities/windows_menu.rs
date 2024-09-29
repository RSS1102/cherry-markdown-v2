use lazy_static::lazy_static;
use std::sync::Mutex;
use tauri::{window::MenuHandle, CustomMenuItem, Menu, Submenu, WindowMenuEvent};

lazy_static! {
    static ref CURRENT_LANG: Mutex<String> = Mutex::new("en".to_string());
}

struct Language {
    file: BilingualMenuItem,
    new_file: BilingualMenuItem,
    open_file: BilingualMenuItem,
    save: BilingualMenuItem,
    save_as: BilingualMenuItem,
    quit: BilingualMenuItem,
    language: BilingualMenuItem,
}

impl Language {
    fn new() -> Self {
        Language {
            file: BilingualMenuItem::new("File", "文件"),
            new_file: BilingualMenuItem::new("New File", "新建文件"),
            open_file: BilingualMenuItem::new("Open File...", "打开文件..."),
            save: BilingualMenuItem::new("Save", "保存"),
            save_as: BilingualMenuItem::new("Save As...", "另存为..."),
            quit: BilingualMenuItem::new("Quit", "退出"),
            language: BilingualMenuItem::new("Language", "语言"),
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

    let new_file_item = CustomMenuItem::new("new_file", language.new_file.get_name(lang_str));
    let open_file_item = CustomMenuItem::new("open_file", language.open_file.get_name(lang_str));
    let save_item = CustomMenuItem::new("save", language.save.get_name(lang_str));
    let save_as_item = CustomMenuItem::new("save_as", language.save_as.get_name(lang_str));
    let quit_item = CustomMenuItem::new("quit", language.quit.get_name(lang_str));

    let submenu = Submenu::new(
        language.file.get_name(lang_str),
        Menu::new()
            .add_item(quit_item)
            .add_item(new_file_item)
            .add_item(open_file_item)
            .add_item(save_item)
            .add_item(save_as_item),
    );

    let zh_menu_item = if lang.as_str() == "zh" {
        CustomMenuItem::new("zh_menu", "中文").selected()
    } else {
        CustomMenuItem::new("zh_menu", "中文")
    };

    let en_menu_item = if lang.as_str() == "en" {
        CustomMenuItem::new("en_menu", "English").selected()
    } else {
        CustomMenuItem::new("en_menu", "English")
    };
    let lang_submenu = Submenu::new(
        language.language.get_name(lang_str),
        Menu::new().add_item(zh_menu_item).add_item(en_menu_item),
    );

    return Menu::new().add_submenu(submenu).add_submenu(lang_submenu);
}

fn set_menu_language(menu_handle: MenuHandle, lang: &str) {
    let language: Language = Language::new();

    let items = vec![
        // ("file", &language.file),
        ("new_file", &language.new_file),
        ("open_file", &language.open_file),
        ("save", &language.save),
        ("save_as", &language.save_as),
        ("quit", &language.quit),
        // ("language", &language.language),
    ];

    for (id, item) in items {
        println!("Setting title for id: {}", id);
        println!("Setting title for lang: {}", lang);
        let title = item.get_name(lang);
        print!("{}", title);
        let _ = menu_handle.get_item(id).set_title(title);
    }
}
pub fn handle_menu_event(event: WindowMenuEvent) {
    let main_window = event.window();
    let menu_handle = main_window.menu_handle();
    match event.menu_item_id() {
        "quit" => {
            event.window().close().unwrap();
        }
        "zh_menu" => {
            let mut lang = CURRENT_LANG.lock().unwrap();
            *lang = "zh".to_string();
            set_menu_language(menu_handle, lang.as_str());
        }
        "en_menu" => {
            let mut lang = CURRENT_LANG.lock().unwrap();
            *lang = "en".to_string();
            set_menu_language(menu_handle, lang.as_str());
        }
        _ => {}
    }
}
