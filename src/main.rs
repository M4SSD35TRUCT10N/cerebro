// This is needed to prevent a command shell to open on windows.
// It still opens when in debug mode.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use fltk::{app::*, dialog::*, menu::*, window::*};

// Message enum for the menu bar and other functionalities.
// Adding something here, needs to be added in the match tree of
// app.wait() too!
#[derive(Copy, Clone)]
pub enum Message {
    About,
    AddUTF8Signature,
    AddSelection,
    AutomaticSynchronization,
    Backup,
    CancelSelection,
    Cerebris,
    CerebroDefaults,
    CerebroPermissions,
    CerebroTheme,
    Changed,
    CheckForUpdate,
    CheckSpellingWhileTyping,
    Copy,
    CopyScreenShotToClipboard,
    CopyURLOfLocalThought,
    CopyURLOfWebThought,
    CloseTab,
    CloseWindow,
    Cut,
    DataIntegrityScan,
    DefaultView,
    DisplayAccountInformations,
    DisplayReport,
    DisplayTimeline,
    DoubleThought,
    DuplicateTab,
    ExpandAllThoughts,
    Export,
    FoldAllThoughts,
    HideAllParents,
    HidePrivateThoughts,
    Import,
    JoinAll,
    ManageEndPoints,
    MindmapView,
    MoveTabToNewWindow,
    NewCerebro,
    NewTab,
    NewWindow,
    NextCerebro,
    OfflineActivation,
    Open,
    OpenCerebroArchive,
    OpenProtocolDirectory,
    OpenThoughtInWebClient,
    OrderThoughtsByDateOfCreation,
    OrderThoughtsByDateOfCreationOldestFirst,
    OrderThoughtsByDateOfModification,
    OrderThoughtsByDateOfModificationOldestFirst,
    OrderThoughtsByDateOfVisit,
    OrderThoughtsByDateOfVisitOldestFirst,
    OrderThoughtsByName,
    OrderThoughtsByType,
    OutlineView,
    Paste,
    PresentationMode,
    PreviousCerebro,
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
    ShowDeceasedThoughts,
    Statistics,
    SynchronizeCerebro,
    TippsAndTricks,
    Tutorials,
    Undo,
    UpgradeToPro,
    Redo,
    Wander,
    WebSearch,
    ZoomIntoCerebro,
    ZoomOutOfCerebro,
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
        "&File/New Tab",
        Shortcut::Ctrl | 't',
        MenuFlag::Normal,
        s,
        Message::NewTab,
    );

    menu.add_emit(
        "&File/New Window",
        Shortcut::Ctrl | Shortcut::Shift | 'n',
        MenuFlag::Normal,
        s,
        Message::NewTab,
    );

    menu.add_emit(
        "&File/New Cerebro",
        Shortcut::Ctrl | 'n',
        MenuFlag::Normal,
        s,
        Message::NewCerebro,
    );

    menu.add_emit(
        "&File/Backup to cerebro archive",
        Shortcut::None,
        MenuFlag::Normal,
        s,
        Message::Backup,
    );

    menu.add_emit(
        "&File/Import",
        Shortcut::None,
        MenuFlag::Normal,
        s,
        Message::Import,
    );

    menu.add_emit(
        "&File/Export",
        Shortcut::None,
        MenuFlag::Normal,
        s,
        Message::Export,
    );

    menu.add_emit(
        "&File/Rename cerebro",
        Shortcut::None,
        MenuFlag::Normal,
        s,
        Message::RenameCerebro,
    );

    menu.add_emit(
        "&File/Statistics",
        Shortcut::None,
        MenuFlag::Normal,
        s,
        Message::Statistics,
    );

    menu.add_emit(
        "&File/Utilities/Manage endpoints",
        Shortcut::None,
        MenuFlag::Normal,
        s,
        Message::ManageEndPoints,
    );

    menu.add_emit(
        "&File/Utilities/Search and Replace",
        Shortcut::None,
        MenuFlag::Normal,
        s,
        Message::SearchReplace,
    );

    menu.add_emit(
        "&File/Utilities/Data integrity scan",
        Shortcut::None,
        MenuFlag::Normal,
        s,
        Message::DataIntegrityScan,
    );

    menu.add_emit(
        "&File/Utilities/Add UTF-8 signature to notes",
        Shortcut::None,
        MenuFlag::Normal,
        s,
        Message::AddUTF8Signature,
    );

    menu.add_emit(
        "&File/Utilities/Show all parents",
        Shortcut::None,
        MenuFlag::Normal,
        s,
        Message::ShowAllParents,
    );

    menu.add_emit(
        "&File/Utilities/Hide all parents",
        Shortcut::None,
        MenuFlag::Normal,
        s,
        Message::HideAllParents,
    );

    menu.add_emit(
        "&File/Close Tab",
        Shortcut::Ctrl | 'w',
        MenuFlag::Normal,
        s,
        Message::CloseTab,
    );

    menu.add_emit(
        "&File/Close Window",
        Shortcut::Alt | 'w',
        MenuFlag::MenuDivider,
        s,
        Message::CloseWindow,
    );

    menu.add_emit(
        "&File/Quit",
        Shortcut::Ctrl | 'q',
        MenuFlag::Normal,
        s,
        Message::Quit,
    );

    // Edit menu entries
    menu.add_emit(
        "&Edit/Undo",
        Shortcut::Ctrl | 'z',
        MenuFlag::Normal,
        s,
        Message::Undo,
    );

    menu.add_emit(
        "&Edit/Redo",
        Shortcut::Ctrl | 'y',
        MenuFlag::MenuDivider,
        s,
        Message::Redo,
    );

    menu.add_emit(
        "&Edit/Cut",
        Shortcut::Ctrl | 'x',
        MenuFlag::Normal,
        s,
        Message::Cut,
    );

    menu.add_emit(
        "&Edit/Copy",
        Shortcut::Ctrl | 'c',
        MenuFlag::Normal,
        s,
        Message::Copy,
    );

    menu.add_emit(
        "&Edit/Copy URL of local thought",
        Shortcut::None,
        MenuFlag::Normal,
        s,
        Message::CopyURLOfLocalThought,
    );

    menu.add_emit(
        "&Edit/Copy URL of web thought",
        Shortcut::None,
        MenuFlag::Normal,
        s,
        Message::CopyURLOfWebThought,
    );

    menu.add_emit(
        "&Edit/Double thought",
        Shortcut::None,
        MenuFlag::MenuDivider,
        s,
        Message::DoubleThought,
    );

    menu.add_emit(
        "&Edit/Paste",
        Shortcut::Ctrl | 'v',
        MenuFlag::Normal,
        s,
        Message::Paste,
    );

    menu.add_emit(
        "&Edit/Select all",
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

    // View menu entries
    menu.add_emit(
        "&View/Default",
        Shortcut::Ctrl | Shortcut::Shift | '1',
        MenuFlag::Toggle,
        s,
        Message::DefaultView,
    );

    menu.add_emit(
        "&View/Outline",
        Shortcut::Ctrl | Shortcut::Shift | '2',
        MenuFlag::Toggle,
        s,
        Message::OutlineView,
    );

    menu.add_emit(
        "&View/Mindmap",
        Shortcut::Ctrl | Shortcut::Shift | '3',
        MenuFlag::Toggle,
        s,
        Message::MindmapView,
    );

    menu.add_emit(
        "&View/Expand all thoughts",
        Shortcut::Ctrl | '0',
        MenuFlag::Normal,
        s,
        Message::ExpandAllThoughts,
    );

    menu.add_emit(
        "&View/Fold all thoughts",
        Shortcut::Ctrl | '9',
        MenuFlag::MenuDivider,
        s,
        Message::FoldAllThoughts,
    );

    menu.add_emit(
        "&View/Zoom into cerebro",
        Shortcut::Ctrl | '=',
        MenuFlag::Normal,
        s,
        Message::ZoomIntoCerebro,
    );

    menu.add_emit(
        "&View/Zoom out of cerebro",
        Shortcut::Ctrl | '-',
        MenuFlag::MenuDivider,
        s,
        Message::ZoomOutOfCerebro,
    );

    menu.add_emit(
        "&View/Open a cerebro archive",
        Shortcut::None,
        MenuFlag::MenuDivider,
        s,
        Message::OpenCerebroArchive,
    );

    menu.add_emit(
        "&View/Report",
        Shortcut::None,
        MenuFlag::Normal,
        s,
        Message::DisplayReport,
    );

    menu.add_emit(
        "&View/Timeline",
        Shortcut::None,
        MenuFlag::MenuDivider,
        s,
        Message::DisplayTimeline,
    );

    menu.add_emit(
        "&View/Presentation mode",
        Shortcut::None,
        MenuFlag::Normal,
        s,
        Message::PresentationMode,
    );

    // Options menu entries
    menu.add_emit(
        "&Options/Order thoughts by/Name",
        Shortcut::None,
        MenuFlag::Toggle,
        s,
        Message::OrderThoughtsByName,
    );

    menu.add_emit(
        "&Options/Order thoughts by/Type",
        Shortcut::None,
        MenuFlag::Toggle,
        s,
        Message::OrderThoughtsByType,
    );

    menu.add_emit(
        "&Options/Order thoughts by/Date of modification",
        Shortcut::None,
        MenuFlag::Toggle,
        s,
        Message::OrderThoughtsByDateOfModification,
    );

    menu.add_emit(
        "&Options/Order thoughts by/Date of creation",
        Shortcut::None,
        MenuFlag::Toggle,
        s,
        Message::OrderThoughtsByDateOfCreation,
    );

    menu.add_emit(
        "&Options/Order thoughts by/Date of visit",
        Shortcut::None,
        MenuFlag::Toggle,
        s,
        Message::OrderThoughtsByDateOfVisit,
    );

    menu.add_emit(
        "&Options/Order thoughts by/Date of modification (oldest first)",
        Shortcut::None,
        MenuFlag::Toggle,
        s,
        Message::OrderThoughtsByDateOfModificationOldestFirst,
    );

    menu.add_emit(
        "&Options/Order thoughts by/Date of creation (oldest first)",
        Shortcut::None,
        MenuFlag::Toggle,
        s,
        Message::OrderThoughtsByDateOfCreationOldestFirst,
    );

    menu.add_emit(
        "&Options/Order thoughts by/Date of visit (oldest first)",
        Shortcut::None,
        MenuFlag::Toggle,
        s,
        Message::OrderThoughtsByDateOfVisitOldestFirst,
    );

    menu.add_emit(
        "&Options/Web search",
        Shortcut::Ctrl | Shortcut::Shift | 'f',
        MenuFlag::MenuDivider,
        s,
        Message::WebSearch,
    );

    menu.add_emit(
        "&Options/Show deceased thoughts",
        Shortcut::None,
        MenuFlag::Normal,
        s,
        Message::ShowDeceasedThoughts,
    );

    menu.add_emit(
        "&Options/Hide private thoughts",
        Shortcut::None,
        MenuFlag::Normal,
        s,
        Message::HidePrivateThoughts,
    );

    menu.add_emit(
        "&Options/Wander",
        Shortcut::None,
        MenuFlag::MenuDivider,
        s,
        Message::Wander,
    );

    menu.add_emit(
        "&Options/Check spelling while typing",
        Shortcut::None,
        MenuFlag::MenuDivider,
        s,
        Message::CheckSpellingWhileTyping,
    );

    menu.add_emit(
        "&Options/Cerebro theme",
        Shortcut::None,
        MenuFlag::Normal,
        s,
        Message::CerebroTheme,
    );

    menu.add_emit(
        "&Options/Cerebro defaults",
        Shortcut::Ctrl | ',',
        MenuFlag::Normal,
        s,
        Message::CerebroDefaults,
    );

    // Online menu entries
    menu.add_emit(
        "&Online/Synchronize cerebro",
        Shortcut::None,
        MenuFlag::Normal,
        s,
        Message::SynchronizeCerebro,
    );

    menu.add_emit(
        "&Online/Automatic synchronization",
        Shortcut::None,
        MenuFlag::MenuDivider,
        s,
        Message::AutomaticSynchronization,
    );

    menu.add_emit(
        "&Online/Open thought in web client",
        Shortcut::None,
        MenuFlag::Normal,
        s,
        Message::OpenThoughtInWebClient,
    );

    menu.add_emit(
        "&Online/Copy URL of web thought",
        Shortcut::None,
        MenuFlag::MenuDivider,
        s,
        Message::CopyURLOfWebThought,
    );

    menu.add_emit(
        "&Online/Cerebro permissions",
        Shortcut::None,
        MenuFlag::MenuDivider,
        s,
        Message::CerebroPermissions,
    );

    menu.add_emit(
        "&Online/Display account informations",
        Shortcut::None,
        MenuFlag::Normal,
        s,
        Message::DisplayAccountInformations,
    );

    // Window menu entries
    menu.add_emit(
        "&Window/Previous cerebro",
        Shortcut::Ctrl | Shortcut::Shift | Key::Tab,
        MenuFlag::Normal,
        s,
        Message::PreviousCerebro,
    );

    menu.add_emit(
        "&Window/Next cerebro",
        Shortcut::Ctrl | Key::Tab,
        MenuFlag::Normal,
        s,
        Message::NextCerebro,
    );

    menu.add_emit(
        "&Window/Move current tab to new window",
        Shortcut::None,
        MenuFlag::Normal,
        s,
        Message::MoveTabToNewWindow,
    );

    menu.add_emit(
        "&Window/Join all",
        Shortcut::None,
        MenuFlag::Normal,
        s,
        Message::JoinAll,
    );

    menu.add_emit(
        "&Window/Duplicate tab",
        Shortcut::None,
        MenuFlag::MenuDivider,
        s,
        Message::DuplicateTab,
    );

    menu.add_emit(
        "&Window/Cerebris",
        Shortcut::None,
        MenuFlag::Normal,
        s,
        Message::Cerebris,
    );

    // Help menu entries
    menu.add_emit(
        "&Help/About",
        Shortcut::None,
        MenuFlag::Normal,
        s,
        Message::About,
    );

    menu.add_emit(
        "&Help/Check for updates",
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

    // menu entry stub - for copy only!
    /* menu.add_emit(
        "",
        Shortcut::None,
        MenuFlag::Normal,
        s,
        Message::,
    ); */

    w.make_resizable(true);
    w.end();
    w.show();

    while app.wait() {
        match r.recv() {
            // TODO: Fill all of them with life!
            Some(Message::About) => println!("About"),
            Some(Message::AddSelection) => println!("AddSelection"),
            Some(Message::AddUTF8Signature) => println!("AddUTF8Signature"),
            Some(Message::AutomaticSynchronization) => println!("AutomaticSynchronization"),
            Some(Message::Backup) => println!("Backup"),
            Some(Message::CancelSelection) => println!("CancelSelection"),
            Some(Message::Cerebris) => println!("Cerebris"),
            Some(Message::CerebroDefaults) => println!("CerebroDefaults"),
            Some(Message::CerebroPermissions) => println!("CerebroPermissions"),
            Some(Message::CerebroTheme) => println!("CerebroTheme"),
            Some(Message::Changed) => println!("Changed"),
            Some(Message::CheckForUpdate) => println!("CheckForUpdate"),
            Some(Message::CheckSpellingWhileTyping) => println!("CheckSpellingWhileTyping"),
            Some(Message::Copy) => println!("Copy"),
            Some(Message::CopyScreenShotToClipboard) => println!("CopyScreenShotToClipboard"),
            Some(Message::CopyURLOfLocalThought) => println!("CopyURLOfLocalThought"),
            Some(Message::CopyURLOfWebThought) => println!("CopyURLOfWebThought"),
            Some(Message::CloseTab) => println!("CloseTab"),
            Some(Message::CloseWindow) => println!("CloseWindow"),
            Some(Message::Cut) => println!("Cut"),
            Some(Message::DataIntegrityScan) => println!("DataIntegrityScan"),
            Some(Message::DefaultView) => println!("DefaultView"),
            Some(Message::DisplayAccountInformations) => println!("DisplayAccountInformations"),
            Some(Message::DisplayReport) => println!("DisplayReport"),
            Some(Message::DisplayTimeline) => println!("DisplayTimeline"),
            Some(Message::DoubleThought) => println!("DoubleThought"),
            Some(Message::DuplicateTab) => println!("DuplicateTab"),
            Some(Message::ExpandAllThoughts) => println!("ExpandAllThoughts"),
            Some(Message::Export) => println!("Export"),
            Some(Message::FoldAllThoughts) => println!("FoldAllThoughts"),
            Some(Message::HideAllParents) => println!("HideAllParents"),
            Some(Message::HidePrivateThoughts) => println!("HidePrivateThoughts"),
            Some(Message::Import) => println!("Import"),
            Some(Message::JoinAll) => println!("JoinAll"),
            Some(Message::ManageEndPoints) => println!("ManageEndPoints"),
            Some(Message::MindmapView) => println!("MindmapView"),
            Some(Message::MoveTabToNewWindow) => println!("MoveTabToNewWindow"),
            Some(Message::NewCerebro) => println!("NewCerebro"),
            Some(Message::NewTab) => println!("NewTab"),
            Some(Message::NewWindow) => println!("NewWindow"),
            Some(Message::NextCerebro) => println!("NextCerebro"),
            Some(Message::OfflineActivation) => println!("OfflineActivation"),
            Some(Message::Open) => println!("Open"),
            Some(Message::OpenCerebroArchive) => println!("OpenCerebroArchive"),
            Some(Message::OpenThoughtInWebClient) => println!("OpenThoughtInWebClient"),
            Some(Message::OpenProtocolDirectory) => println!("OpenProtocolDirectory"),
            Some(Message::OrderThoughtsByDateOfCreation) => {
                println!("OrderThoughtsByDateOfCreation")
            }
            Some(Message::OrderThoughtsByDateOfCreationOldestFirst) => {
                println!("OrderThoughtsByDateOfCreationOldestFirst")
            }
            Some(Message::OrderThoughtsByDateOfModification) => {
                println!("OrderThoughtsByDateOfModification")
            }
            Some(Message::OrderThoughtsByDateOfModificationOldestFirst) => {
                println!("OrderThoughtsByDateOfModificationOldestFirst")
            }
            Some(Message::OrderThoughtsByDateOfVisit) => println!("OrderThoughtsByDateOfVisit"),
            Some(Message::OrderThoughtsByDateOfVisitOldestFirst) => {
                println!("OrderThoughtsByDateOfVisitOldestFirst")
            }
            Some(Message::OrderThoughtsByName) => println!("OrderThoughtsByName"),
            Some(Message::OrderThoughtsByType) => println!("OrderThoughtsByType"),
            Some(Message::OutlineView) => println!("OutlineView"),
            Some(Message::Paste) => println!("Paste"),
            Some(Message::PresentationMode) => println!("PresentationMode"),
            Some(Message::PreviousCerebro) => println!("PreviousCerebro"),
            Some(Message::Quit) => app.quit(), // TODO: emit autosave message
            Some(Message::RenameCerebro) => println!("RenameCerebro"),
            Some(Message::Save) => println!("Save"),
            Some(Message::SaveAs) => println!("SaveAs"),
            Some(Message::SearchReplace) => println!("SearchReplace"),
            Some(Message::SelectAll) => println!("SelectAll"),
            Some(Message::SelectPathInBetween) => println!("SelectPathInBetween"),
            Some(Message::SelectRelatedThoughts) => println!("SelectRelatedThoughts"),
            Some(Message::ShowAllParents) => println!("ShowAllParents"),
            Some(Message::ShowDeceasedThoughts) => println!("ShowDeceasedThoughts"),
            Some(Message::ShowCommonChildren) => println!("ShowCommonChildren"),
            Some(Message::Statistics) => println!("Statistics"),
            Some(Message::SynchronizeCerebro) => println!("SynchronizeCerebro"),
            Some(Message::TippsAndTricks) => println!("TippsAndTricks"),
            Some(Message::Tutorials) => println!("Tutorials"),
            Some(Message::Undo) => println!("Undo"),
            Some(Message::UpgradeToPro) => println!("UpgradeToPro"),
            Some(Message::QuickstartCerebro) => println!("QuickstartCerebro"),
            Some(Message::Redo) => println!("Redo"),
            Some(Message::Wander) => println!("Wander"),
            Some(Message::WebSearch) => println!("WebSearch"),
            Some(Message::ZoomIntoCerebro) => println!("ZoomIntoCerebro"),
            Some(Message::ZoomOutOfCerebro) => println!("ZoomOutOfCerebro"),
            None => (),
        }
    }
}
