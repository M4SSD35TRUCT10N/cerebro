// TODO:
//      1. Layout as TheBrain 11 is
//      2. Change to horizontal layout dynamically.
//      3. Fill slowly with function.
//      4. Release early, release often.

// This is needed to prevent a command shell to open on windows.
// Uncomment when releasing! TODO: Automate this!
// #![windows_subsystem = "windows"]

use fltk::{app::*, dialog::*, menu::*, window::*};

// Message enum for the menu bar and other functionalities.
// Adding something here, needs to be added in the match tree of
// app.wait() too!
#[derive(Copy, Clone)]
pub enum Message {
    About,
    AddUTF8Signature,
    AddSelection,
    Backup,
    CancelSelection,
    Changed,
    CheckForUpdate,
    Copy,
    CopyScreenShotToClipboard,
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
    QuickstartCerebro,
    Quit,
    RenameCerebro,
    Save,
    SaveAs,
    SearchReplace,
    SelectAll,
    SelectPathInBetween,
    SelectRelatedThoughts,
    ShowAllParents,
    ShowCommonChildren,
    Statistics,
    TippsAndTricks,
    Tutorials,
    Undo,
    UpgradeToPro,
    OfflineActivation,
    OpenProtocolDirectory,
    Redo,
}

fn main() {
    // I like to have some initial values as constants since they'll never
    // change over the course of the application running. They also help to
    // reduce memory footprint of the application due to compiler
    // optimizations.
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

    // Create the main window.
    let mut w = Window::default()
        .with_size(INITIAL_WIDTH, INITIAL_HEIGHT)
        .center_screen()
        .with_label("cerebro - a note taking and knowledge management tool");

    // Be macOS friendly from the beginning: start as system menu bar - no effect on windows
    let mut menu = SysMenuBar::new(0, 0, INITIAL_WIDTH, INITIAL_MENU_HEIGHT, "cerebro menu");

    // Populating menu items
    // File menu entries
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

    // Edit menu entries
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
        "&Edit/Add Selection",
        Shortcut::None,
        MenuFlag::MenuDivider,
        s,
        Message::AddSelection,
    );

    menu.add_emit(
        "&Edit/Select related thoughts",
        Shortcut::None,
        MenuFlag::Normal,
        s,
        Message::SelectRelatedThoughts,
    );

    menu.add_emit(
        "&Edit/Cancel Selection",
        Shortcut::None,
        MenuFlag::MenuDivider,
        s,
        Message::CancelSelection,
    );

    menu.add_emit(
        "&Edit/Show common children",
        Shortcut::None,
        MenuFlag::Normal,
        s,
        Message::ShowCommonChildren,
    );

    menu.add_emit(
        "&Edit/Select path in between",
        Shortcut::None,
        MenuFlag::MenuDivider,
        s,
        Message::SelectPathInBetween,
    );

    menu.add_emit(
        "&Edit/Copy screen shot to clipboard",
        Shortcut::None,
        MenuFlag::Normal,
        s,
        Message::CopyScreenShotToClipboard,
    );

    // Help menu entries
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

    menu.add_emit(
        "&Help/Upgrade to cerebro Professional",
        Shortcut::None,
        MenuFlag::Normal,
        s,
        Message::UpgradeToPro,
    );

    menu.add_emit(
        "&Help/Offline Activation",
        Shortcut::None,
        MenuFlag::MenuDivider,
        s,
        Message::OfflineActivation,
    );

    menu.add_emit(
        "&Help/Tutorials",
        Shortcut::None,
        MenuFlag::Normal,
        s,
        Message::Tutorials,
    );

    menu.add_emit(
        "&Help/Tipps and Tricks",
        Shortcut::None,
        MenuFlag::Normal,
        s,
        Message::TippsAndTricks,
    );

    menu.add_emit(
        "&Help/Generate an examplary cerebro",
        Shortcut::None,
        MenuFlag::MenuDivider,
        s,
        Message::QuickstartCerebro,
    );

    menu.add_emit(
        "&Help/Open directory of protocols",
        Shortcut::None,
        MenuFlag::Normal,
        s,
        Message::OpenProtocolDirectory,
    );

    w.make_resizable(true);
    w.end();
    w.show();
    while app.wait() {
        match r.recv() {
            // TODO: Fill all of them with life!
            Some(Message::About) => println!("About"),
            Some(Message::AddSelection) => println!("AddSelection"),
            Some(Message::AddUTF8Signature) => println!("AddUTF8Signature"),
            Some(Message::Backup) => println!("Backup"),
            Some(Message::CancelSelection) => println!("CancelSelection"),
            Some(Message::Changed) => println!("Changed"),
            Some(Message::CheckForUpdate) => println!("CheckForUpdate"),
            Some(Message::Copy) => println!("Copy"),
            Some(Message::CopyScreenShotToClipboard) => println!("CopyScreenShotToClipboard"),
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
            Some(Message::OfflineActivation) => println!("OfflineActivation"),
            Some(Message::Open) => println!("Open"),
            Some(Message::OpenProtocolDirectory) => println!("OpenProtocolDirectory"),
            Some(Message::Paste) => println!("Paste"),
            Some(Message::Quit) => app.quit(), // TODO: emit autosave message
            Some(Message::RenameCerebro) => println!("RenameCerebro"),
            Some(Message::Save) => println!("Save"),
            Some(Message::SaveAs) => println!("SaveAs"),
            Some(Message::SearchReplace) => println!("SearchReplace"),
            Some(Message::SelectAll) => println!("SelectAll"),
            Some(Message::SelectPathInBetween) => println!("SelectPathInBetween"),
            Some(Message::SelectRelatedThoughts) => println!("SelectRelatedThoughts"),
            Some(Message::ShowAllParents) => println!("ShowAllParents"),
            Some(Message::ShowCommonChildren) => println!("ShowCommonChildren"),
            Some(Message::Statistics) => println!("Statistics"),
            Some(Message::TippsAndTricks) => println!("TippsAndTricks"),
            Some(Message::Tutorials) => println!("Tutorials"),
            Some(Message::Undo) => println!("Undo"),
            Some(Message::UpgradeToPro) => println!("UpgradeToPro"),
            Some(Message::QuickstartCerebro) => println!("QuickstartCerebro"),
            Some(Message::Redo) => println!("Redo"),
            None => (),
        }
    }
}
