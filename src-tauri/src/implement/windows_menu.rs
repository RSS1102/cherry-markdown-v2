use lazy_static::lazy_static;
use std::sync::Mutex;
use tauri::{
    menu::{CheckMenuItemBuilder, MenuBuilder, SubmenuBuilder},
    App,
};

lazy_static! {
    pub static ref CURRENT_LANG: Mutex<String> = Mutex::new("en".to_string());
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

fn set_menu_language(app: &tauri::AppHandle, lang: &str) {
    let _ = app;
    let language: Language = Language::new();

    let items = vec![
        ("file", &language.file),
        ("new_file", &language.new_file),
        ("open_file", &language.open_file),
        ("save", &language.save),
        ("save_as", &language.save_as),
        ("quit", &language.quit),
        ("language", &language.language),
    ];

    for (id, item) in items {
        println!("Setting title for id: {}", id);
        println!("Setting title for lang: {}", lang);
        let title = item.get_name(lang);
        print!("{}", title);
    }
}

pub fn window_menu(app: &mut App) -> Result<(), tauri::Error> {
    let lang = CURRENT_LANG.lock().unwrap().clone();
    let lang_str = lang.as_str();
    let language = Language::new();
    let handle = app.handle();

    let file_menu = SubmenuBuilder::new(app, language.file.get_name(lang_str))
        .text("new_file", language.new_file.get_name(lang_str))
        .text("open_file", language.open_file.get_name(lang_str))
        .text("save", language.save.get_name(lang_str))
        .text("save_as", language.save_as.get_name(lang_str))
        .text("quit", language.quit.get_name(lang_str))
        .build()?;

    let language_sub_en = CheckMenuItemBuilder::new("English")
        .id("en")
        .checked(lang_str == "en")
        .build(app)?;
    

    let language_sub_zh = CheckMenuItemBuilder::new("中文")
        .id("zh")
        .checked(lang_str == "zh")
        .build(app)?;

    let language_menu = SubmenuBuilder::new(app, language.language.get_name(lang_str))
        .item(&language_sub_en)
        .item(&language_sub_zh)
        .build()?;

    let menu = MenuBuilder::new(handle)
        .item(&file_menu)
        .item(&language_menu)
        .build()?;

    let _ = app.set_menu(menu);

    app.on_menu_event(move |app, event| {
        println!("event {:?} ", event.id());
        if event.id() == "en" {
            let mut lang = CURRENT_LANG.lock().unwrap();
            *lang = "en".to_string();
        }
        if event.id() == "zh" {
            let mut lang = CURRENT_LANG.lock().unwrap();
            *lang = "en".to_string();
        }
    });
    Ok(())
}
