use lazy_static::lazy_static;
use std::sync::{ Mutex};
use tauri::{
    menu::{CheckMenuItemBuilder, Menu, MenuBuilder, SubmenuBuilder},
    App, AppHandle, Wry,
};

lazy_static! {
    pub static ref CURRENT_LANG: Mutex<String> = Mutex::new("en".to_string());
}
#[derive(Clone)]
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

#[derive(Clone)]
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

fn create_menu(
    handle: &AppHandle,
    language: Language,
    lang_str: &str,
) -> Result<Menu<Wry>, tauri::Error> {
    let file_menu = SubmenuBuilder::new(handle, language.file.get_name(lang_str))
        .text("new_file", language.new_file.get_name(lang_str))
        .text("open_file", language.open_file.get_name(lang_str))
        .text("save", language.save.get_name(lang_str))
        .text("save_as", language.save_as.get_name(lang_str))
        .text("quit", language.quit.get_name(lang_str));

    let language_sub_en = CheckMenuItemBuilder::new("English")
        .id("en")
        .checked(lang_str == "en");

    let language_sub_zh = CheckMenuItemBuilder::new("中文")
        .id("zh")
        .checked(lang_str == "zh");

    let language_menu = SubmenuBuilder::new(handle, language.language.get_name(lang_str))
        .item(&language_sub_en.build(handle)?)
        .item(&language_sub_zh.build(handle)?);

    let menu = MenuBuilder::new(handle)
        .item(&file_menu.build()?)
        .item(&language_menu.build()?)
        .build()?;
    Ok(menu)
}

pub fn window_menu(app: &mut App) -> Result<(), tauri::Error> {
    let lang = CURRENT_LANG.lock().unwrap().clone();
    let lang_str = lang.as_str().to_string();
    let handle = app.handle();
    let language = Language::new();

    let menu = create_menu(&handle, language.clone(), &lang_str)?;
    app.set_menu(menu)?;

    let handle_clone = handle.clone();
    let language_clone = language.clone();

    app.on_menu_event(move |app_handle: &tauri::AppHandle, event| {
        println!("event {:?} ", event.id());
        if event.id() == "en" {
            let mut lang = CURRENT_LANG.lock().unwrap();
            *lang = "en".to_string();
            if let Ok(menu) = create_menu(&handle_clone, language_clone.clone(), &lang) {
                let _ = app_handle.set_menu(menu);
            }
        }
        if event.id() == "zh" {
            let mut lang = CURRENT_LANG.lock().unwrap();
            *lang = "zh".to_string();
            if let Ok(menu) = create_menu(&handle_clone, language_clone.clone(), &lang) {
                let _ = app_handle.set_menu(menu);
            }
        }
    });
    Ok(())
}
