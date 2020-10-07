// TODO:
//      1. Layout as TheBrain 11 is
//      2. Change to horizontal layout dynamically.
//      3. Fill slowly with function.
//      4. Release early, release often.

#![windows_subsystem = "windows"]

use fltk::{app::*, dialog::*, menu::*, window::*};

#[derive(Copy, Clone)]
pub enum Message {
    About,
    Backup,
    Changed,
    CheckForUpdate,
    Copy,
    Cut,
    Export,
    Import,
    NewCerebro,
    NewTab,
    NewWindow,
    Open,
    Paste,
    Quit,
    RenameCerebro,
    Save,
    SaveAs,
    Statistics,
    Utilities,
}

fn main() {
    const INITIAL_WIDTH: i32 = 864;
    const INITIAL_HEIGHT: i32 = 360;
    const INITIAL_MENU_HEIGHT: i32 = 21;

    // The app variable is immutable. This is the entry point of the
    // application.
    // Loads with Base scheme of FLTK - will later be changeable via JSON
    // based config file.
    // Loads all fonts known to the system for convenience later on.
    let app = App::default()
        .with_scheme(AppScheme::Base)
        .load_system_fonts();

    // Collect the cli arguments.
    // TODO:    Document each of them!
    let args: Vec<String> = std::env::args().collect();

    let (s, r) = channel::<Message>();

    let mut w = Window::default()
        .with_size(INITIAL_WIDTH, INITIAL_HEIGHT)
        .center_screen()
        .with_label("cerebro - a note taking and knowledge management tool");

    let mut menu = SysMenuBar::new(0, 0, INITIAL_WIDTH, INITIAL_MENU_HEIGHT, "cerebro menu");

    menu.add_emit(
        "&File/New...\t",
        Shortcut::Ctrl | 'n',
        MenuFlag::Normal,
        s,
        Message::NewCerebro,
    );

    menu.add_emit(
        "&File/Open...\t",
        Shortcut::Ctrl | 'o',
        MenuFlag::Normal,
        s,
        Message::Open,
    );

    menu.add_emit(
        "&File/Save\t",
        Shortcut::Ctrl | 's',
        MenuFlag::Normal,
        s,
        Message::Save,
    );

    menu.add_emit(
        "&File/Save as...\t",
        Shortcut::Ctrl | 'w',
        MenuFlag::MenuDivider,
        s,
        Message::SaveAs,
    );

    menu.add_emit(
        "&File/Quit\t",
        Shortcut::Ctrl | 'q',
        MenuFlag::Normal,
        s,
        Message::Quit,
    );

    menu.add_emit(
        "&Edit/Cut\t",
        Shortcut::Ctrl | 'x',
        MenuFlag::Normal,
        s,
        Message::Cut,
    );

    menu.add_emit(
        "&Edit/Copy\t",
        Shortcut::Ctrl | 'c',
        MenuFlag::Normal,
        s,
        Message::Copy,
    );

    menu.add_emit(
        "&Edit/Paste\t",
        Shortcut::Ctrl | 'v',
        MenuFlag::Normal,
        s,
        Message::Paste,
    );

    menu.add_emit(
        "&Help/About\t",
        Shortcut::None,
        MenuFlag::Normal,
        s,
        Message::About,
    );

    menu.add_emit(
        "&Help/Check for updates\t",
        Shortcut::None,
        MenuFlag::Normal,
        s,
        Message::CheckForUpdate,
    );

    w.make_resizable(true);
    w.end();
    w.show();
    app.run().unwrap();
}
