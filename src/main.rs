// TODO:
//      1. Layout as TheBrain 11 is
//      2. Change to horizontal layout dynamically.
//      3. Fill slowly with function.
//      4. Release early, release often.

#![windows_subsystem = "windows"]

use fltk::{app::*, dialog::*, menu::*, window::*};

// Adding something here, needs to be added in the match tree of
// app.wait() too!
#[derive(Copy, Clone)]
pub enum Message {
    About,
    AddUTF8Signature,
    Backup,
    Changed,
    CheckForUpdate,
    Copy,
    CopyURLOfLocalThought,
    CopyURLOfWebThought,
    CloseTab,
    CloseWindow,
    Cut,
    DataIntegrityScan,
    DoubleThought,
    Export,
    HideAllParents,
    Import,
    ManageEndPoints,
    NewCerebro,
    NewTab,
    NewWindow,
    None,
    Open,
    Paste,
    Quit,
    RenameCerebro,
    Save,
    SaveAs,
    SearchReplace,
    SelectAll,
    ShowAllParents,
    Statistics,
    Undo,
    Redo,
}

fn main() {
    const INITIAL_WIDTH: i32 = 864;
    const INITIAL_HEIGHT: i32 = 360;
    const INITIAL_MENU_HEIGHT: i32 = 23;

    // The app variable is immutable. This is the entry point of the
    // application.
    // Loads with Base scheme of FLTK - will later be changeable via JSON
    // based config file.
    // Loads all fonts known to the system for convenience later on.
    let app = App::default()
        .with_scheme(AppScheme::Gleam)
        .load_system_fonts();

    // Collect the cli arguments.
    // TODO:    Implement and document each of them!
    //let args: Vec<String> = std::env::args().collect();

    let (s, r) = channel::<Message>();

    let mut w = Window::default()
        .with_size(INITIAL_WIDTH, INITIAL_HEIGHT)
        .center_screen()
        .with_label("cerebro - a note taking and knowledge management tool");

    let mut menu = SysMenuBar::new(0, 0, INITIAL_WIDTH, INITIAL_MENU_HEIGHT, "cerebro menu");

    menu.add_emit(
        "&File/New Tab\t",
        Shortcut::Ctrl | 't',
        MenuFlag::Normal,
        s,
        Message::NewTab,
    );

    menu.add_emit(
        "&File/New Window\t",
        Shortcut::Ctrl | Shortcut::Shift | 'n',
        MenuFlag::Normal,
        s,
        Message::NewTab,
    );

    menu.add_emit(
        "&File/New Cerebro\t",
        Shortcut::Ctrl | 'n',
        MenuFlag::Normal,
        s,
        Message::NewCerebro,
    );

    menu.add_emit(
        "&File/Backup to cerebro archive\t",
        Shortcut::None,
        MenuFlag::Normal,
        s,
        Message::Backup,
    );

    menu.add_emit(
        "&File/Import\t",
        Shortcut::None,
        MenuFlag::Normal,
        s,
        Message::Import,
    );

    menu.add_emit(
        "&File/Export\t",
        Shortcut::None,
        MenuFlag::Normal,
        s,
        Message::Export,
    );

    menu.add_emit(
        "&File/Rename cerebro\t",
        Shortcut::None,
        MenuFlag::Normal,
        s,
        Message::RenameCerebro,
    );

    menu.add_emit(
        "&File/Statistics\t",
        Shortcut::None,
        MenuFlag::Normal,
        s,
        Message::Statistics,
    );

    menu.add_emit(
        "&File/Utilities/Manage endpoints\t",
        Shortcut::None,
        MenuFlag::Normal,
        s,
        Message::ManageEndPoints,
    );

    menu.add_emit(
        "&File/Utilities/Search and Replace\t",
        Shortcut::None,
        MenuFlag::Normal,
        s,
        Message::SearchReplace,
    );

    menu.add_emit(
        "&File/Utilities/Data integrity scan\t",
        Shortcut::None,
        MenuFlag::Normal,
        s,
        Message::DataIntegrityScan,
    );

    menu.add_emit(
        "&File/Utilities/Add UTF-8 signature to notes\t",
        Shortcut::None,
        MenuFlag::Normal,
        s,
        Message::AddUTF8Signature,
    );

    menu.add_emit(
        "&File/Utilities/Show all parents\t",
        Shortcut::None,
        MenuFlag::Normal,
        s,
        Message::ShowAllParents,
    );

    menu.add_emit(
        "&File/Utilities/Hide all parents\t",
        Shortcut::None,
        MenuFlag::Normal,
        s,
        Message::HideAllParents,
    );

    menu.add_emit(
        "&File/Close Tab\t",
        Shortcut::Ctrl | 'w',
        MenuFlag::Normal,
        s,
        Message::CloseTab,
    );

    menu.add_emit(
        "&File/Close Window\t",
        Shortcut::Alt | 'w',
        MenuFlag::MenuDivider,
        s,
        Message::CloseWindow,
    );

    menu.add_emit(
        "&File/Quit\t",
        Shortcut::Ctrl | 'q',
        MenuFlag::Normal,
        s,
        Message::Quit,
    );

    menu.add_emit(
        "&Edit/Undo\t",
        Shortcut::Ctrl | 'z',
        MenuFlag::Normal,
        s,
        Message::Undo,
    );

    menu.add_emit(
        "&Edit/Redo\t",
        Shortcut::Ctrl | 'y',
        MenuFlag::MenuDivider,
        s,
        Message::Redo,
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
        "&Edit/Copy URL of local thought\t",
        Shortcut::None,
        MenuFlag::Normal,
        s,
        Message::CopyURLOfLocalThought,
    );

    menu.add_emit(
        "&Edit/Copy URL of web thought\t",
        Shortcut::None,
        MenuFlag::Normal,
        s,
        Message::CopyURLOfWebThought,
    );

    menu.add_emit(
        "&Edit/Double thought\t",
        Shortcut::None,
        MenuFlag::MenuDivider,
        s,
        Message::DoubleThought,
    );

    menu.add_emit(
        "&Edit/Paste\t",
        Shortcut::Ctrl | 'v',
        MenuFlag::Normal,
        s,
        Message::Paste,
    );

    menu.add_emit(
        "&Edit/Select all\t",
        Shortcut::Ctrl | 'a',
        MenuFlag::MenuDivider,
        s,
        Message::SelectAll,
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
    while app.wait() {
        match r.recv() {
            // TODO: Fill all of them with life!
            Some(Message::About) => println!("About"),
            Some(Message::AddUTF8Signature) => println!("AddUTF8Signature"),
            Some(Message::Backup) => println!("Backup"),
            Some(Message::Changed) => println!("Changed"),
            Some(Message::CheckForUpdate) => println!("CheckForUpdate"),
            Some(Message::Copy) => println!("Copy"),
            Some(Message::CopyURLOfLocalThought) => println!("CopyURLOfLocalThought"),
            Some(Message::CopyURLOfWebThought) => println!("CopyURLOfWebThought"),
            Some(Message::CloseTab) => println!("CloseTab"),
            Some(Message::CloseWindow) => println!("CloseWindow"),
            Some(Message::Cut) => println!("Cut"),
            Some(Message::DataIntegrityScan) => println!("DataIntegrityScan"),
            Some(Message::DoubleThought) => println!("DoubleThought"),
            Some(Message::Export) => println!("Export"),
            Some(Message::HideAllParents) => println!("HideAllParents"),
            Some(Message::Import) => println!("Import"),
            Some(Message::ManageEndPoints) => println!("ManageEndPoints"),
            Some(Message::NewCerebro) => println!("NewCerebro"),
            Some(Message::NewTab) => println!("NewTab"),
            Some(Message::NewWindow) => println!("NewWindow"),
            Some(Message::None) => println!("None"),
            Some(Message::Open) => println!("Open"),
            Some(Message::Paste) => println!("Paste"),
            Some(Message::Quit) => app.quit(),
            Some(Message::RenameCerebro) => println!("RenameCerebro"),
            Some(Message::Save) => println!("Save"),
            Some(Message::SaveAs) => println!("SaveAs"),
            Some(Message::SearchReplace) => println!("SearchReplace"),
            Some(Message::SelectAll) => println!("SelectAll"),
            Some(Message::ShowAllParents) => println!("ShowAllParents"),
            Some(Message::Statistics) => println!("Statistics"),
            Some(Message::Undo) => println!("Undo"),
            Some(Message::Redo) => println!("Redo"),
            None => (),
        }
    }
}
