// This is needed to prevent a command shell to open on windows.
// It still opens when in debug mode.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use fltk::{
    app::*, button::*, dialog::*, enums::*, group::*, image::*, input::*, menu::*, prelude::*,
    window::*, *,
};

// I like to have some initial values as constants since they'll never
// change over the course of the application running. They also help to
// reduce memory footprint of the application due to compiler
// optimizations.
const INITIAL_WIDTH: i32 = 864;
const INITIAL_HEIGHT: i32 = 360;
const INITIAL_MENU_HEIGHT: i32 = 22;

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
    ConvertToTagOrType,
    Copy,
    CopyScreenShotToClipboard,
    CopyURLOfLocalThought,
    CopyURLOfWebThought,
    CloseTab,
    CloseWindow,
    CreateChild,
    CreateLink,
    CreateParent,
    CreateTag,
    CreateType,
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
    MarkAsDeceased,
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
    OrderCerebrisByName,
    OrderCerebrisByDateOfModification,
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
    RefreshCerebris,
    RemovePin,
    RemoveTag,
    RemoveType,
    RenameCerebro,
    Save,
    SaveAs,
    SearchReplace,
    SelectAll,
    SelectPathInBetween,
    SelectRelatedThoughts,
    SetAsRoot,
    SetTag,
    SetType,
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
    ViewHistory,
    ViewProperties,
    Wander,
    WebSearch,
    ZoomIntoCerebro,
    ZoomOutOfCerebro,
}

struct MyButton {
    btn: Button,
}

impl MyButton {
    pub fn new(x: i32, y: i32, w: i32, h: i32, label: &'static str) -> Self {
        let mut btn = Button::new(x, y, w, h, label);
        btn.set_frame(FrameType::UpBox);
        btn.set_label_size(0);
        //btn.draw2(|b| {
        //  draw::set_draw_hex_color(0x464646);
        //  draw::set_line_style(draw::LineStyle::JoinRound, 2);
        //  draw::draw_line(
        //      b.x(),
        //      b.y() + b.height() + 5,
        //      b.x() + b.width(),
        //      b.y() + b.height() + 5,
        //  )*
        //});
        MyButton { btn }
    }
}

impl std::ops::Deref for MyButton {
    type Target = Button;

    fn deref(&self) -> &Self::Target {
        &self.btn
    }
}

impl std::ops::DerefMut for MyButton {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.btn
    }
}

fn init_tab(mut tab: Tabs) -> Tabs {
    // Check wether there is an initial Tab or not.
    // If not, create one.
    let mut i = 0;
    for t in tab.clone().into_iter() {
        if t.as_group().is_some() {
            i += 1;
        }
    }

    if i == 0 {
        tab = new_tab(tab);
    }

    tab
}

fn new_tab(mut tab: Tabs) -> Tabs {
    let new_group = Group::new(
        1,
        2 * INITIAL_MENU_HEIGHT,
        INITIAL_WIDTH - 2,
        INITIAL_HEIGHT - 2 * INITIAL_MENU_HEIGHT,
        "Cerebris",
    );

    let (s, _r) = channel::<Message>();

    //TODO: Fill with life - display all cerebris of the user (either local
    //      from a given directory or from the web source).
    //      Add images to the buttons!
    let mut hpck_tab_btns = Pack::new(
        2,
        2 * INITIAL_MENU_HEIGHT + 4,
        4 * INITIAL_MENU_HEIGHT + 4,
        INITIAL_MENU_HEIGHT,
        "",
    );
    let mut btn_new_cerebro = MyButton::new(0, 0, INITIAL_MENU_HEIGHT, 0, "");
    btn_new_cerebro.set_tooltip("Create a new cerebro");
    btn_new_cerebro.emit(s, Message::NewCerebro);
    let mut img_new_cerebro = PngImage::load("assets/material-design-icons/png/file/create_new_folder/materialicons/18dp/2x/baseline_create_new_folder_black_18dp.png").unwrap();
    img_new_cerebro.scale(INITIAL_MENU_HEIGHT - 2, INITIAL_MENU_HEIGHT - 2, true, true);
    btn_new_cerebro.set_image(Some(img_new_cerebro));

    let mut btn_refresh_list = MyButton::new(0, 0, INITIAL_MENU_HEIGHT, 0, "");
    btn_refresh_list.set_tooltip("Refresh list of cerebris");
    btn_refresh_list.emit(s, Message::RefreshCerebris);
    let mut img_refresh_list = PngImage::load("assets/material-design-icons/png/action/autorenew/materialicons/18dp/2x/baseline_autorenew_black_18dp.png").unwrap();
    img_refresh_list.scale(INITIAL_MENU_HEIGHT - 2, INITIAL_MENU_HEIGHT - 2, true, true);
    btn_refresh_list.set_image(Some(img_refresh_list));

    let mut btn_sort_by_name = MyButton::new(0, 0, INITIAL_MENU_HEIGHT, 0, "");
    btn_sort_by_name.set_tooltip("Sort cerebris by name");
    btn_sort_by_name.emit(s, Message::OrderCerebrisByName);
    let mut img_sort_by_name = PngImage::load("assets/material-design-icons/png/action/list/materialicons/18dp/2x/baseline_list_black_18dp.png").unwrap();
    img_sort_by_name.scale(INITIAL_MENU_HEIGHT, INITIAL_MENU_HEIGHT, true, true);
    btn_sort_by_name.set_image(Some(img_sort_by_name));

    let mut btn_sort_by_mod_date = MyButton::new(0, 0, INITIAL_MENU_HEIGHT, 0, "");
    btn_sort_by_mod_date.set_tooltip("Sort cerebris by modification date");
    btn_sort_by_mod_date.emit(s, Message::OrderCerebrisByDateOfModification);
    let mut img_sort_by_mod_date = PngImage::load("assets/material-design-icons/png/action/date_range/materialicons/18dp/2x/baseline_date_range_black_18dp.png").unwrap();
    img_sort_by_mod_date.scale(INITIAL_MENU_HEIGHT, INITIAL_MENU_HEIGHT, true, true);
    btn_sort_by_mod_date.set_image(Some(img_sort_by_mod_date));

    hpck_tab_btns.end();
    hpck_tab_btns.set_type(PackType::Horizontal);

    let mut hpck_filter = Pack::new(
        INITIAL_WIDTH - 8 * INITIAL_MENU_HEIGHT - 2,
        2 * INITIAL_MENU_HEIGHT + 4,
        8 * INITIAL_MENU_HEIGHT,
        INITIAL_MENU_HEIGHT,
        "",
    );

    let mut filter = Input::new(0, 0, 8 * INITIAL_MENU_HEIGHT, 0, "");
    filter.set_tooltip("Filter cerebris by name");
    filter.set_text_color(Color::Light1);
    filter.set_value("Type name of cerebro here");

    hpck_filter.end();
    hpck_filter.set_type(PackType::Horizontal);

    new_group.end();

    tab.add(&new_group);

    tab
}

fn main() {
    // The app variable is immutable. This is the entry point of the
    // application.
    // Loads with Gleam scheme of FLTK - will later be changeable via JSON
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
    let mut menu = SysMenuBar::new(1, 0, INITIAL_WIDTH - 2, INITIAL_MENU_HEIGHT, "cerebro menu");

    // Create tabs area which will be altered by functions on the fly.
    let mut tab = Tabs::new(
        1,
        INITIAL_MENU_HEIGHT,
        INITIAL_WIDTH - 2,
        INITIAL_HEIGHT - 2 * INITIAL_MENU_HEIGHT,
        "",
    );
    tab.end();

    tab = init_tab(tab);

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

    // Core thought menu entries
    menu.add_emit(
        "&Core thought/Create child",
        Shortcut::None,
        MenuFlag::Normal,
        s,
        Message::CreateChild,
    );

    menu.add_emit(
        "&Core thought/Create parent",
        Shortcut::None,
        MenuFlag::Normal,
        s,
        Message::CreateParent,
    );

    menu.add_emit(
        "&Core thought/Create link",
        Shortcut::None,
        MenuFlag::MenuDivider,
        s,
        Message::CreateLink,
    );

    menu.add_emit(
        "&Core thought/Type/Create type",
        Shortcut::None,
        MenuFlag::Normal,
        s,
        Message::CreateType,
    );

    menu.add_emit(
        "&Core thought/Type/Remove type",
        Shortcut::None,
        MenuFlag::Normal,
        s,
        Message::RemoveType,
    );

    menu.add_emit(
        "&Core thought/Type/Set type",
        Shortcut::None,
        MenuFlag::Normal,
        s,
        Message::SetType,
    );

    menu.add_emit(
        "&Core thought/Tag/Create tag",
        Shortcut::None,
        MenuFlag::Normal,
        s,
        Message::CreateTag,
    );

    menu.add_emit(
        "&Core thought/Tag/Remove tag",
        Shortcut::None,
        MenuFlag::Normal,
        s,
        Message::RemoveTag,
    );

    menu.add_emit(
        "&Core thought/Tag/Set tag",
        Shortcut::None,
        MenuFlag::Normal,
        s,
        Message::SetTag,
    );

    menu.add_emit(
        "&Core thought/Mark as deceased",
        Shortcut::None,
        MenuFlag::Normal,
        s,
        Message::MarkAsDeceased,
    );

    menu.add_emit(
        "&Core thought/Remove pin",
        Shortcut::None,
        MenuFlag::Normal,
        s,
        Message::RemovePin,
    );

    menu.add_emit(
        "&Core thought/Set as root",
        Shortcut::None,
        MenuFlag::MenuDivider,
        s,
        Message::SetAsRoot,
    );

    menu.add_emit(
        "&Core thought/Convert to tag or type",
        Shortcut::None,
        MenuFlag::Normal,
        s,
        Message::ConvertToTagOrType,
    );

    menu.add_emit(
        "&Core thought/View history",
        Shortcut::None,
        MenuFlag::Normal,
        s,
        Message::ViewHistory,
    );

    menu.add_emit(
        "&Core thought/View properties",
        Shortcut::None,
        MenuFlag::Normal,
        s,
        Message::ViewProperties,
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

    // We need some application icon of some sort - subject to change.
    // TODO: Find or create a fitting icon for that matter.
    let w_img = PngImage::load("assets/material-design-icons/png/maps/badge/materialicons/48dp/2x/baseline_badge_black_48dp.png").unwrap();
    w.set_icon(Some(w_img));

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
            Some(Message::CloseTab) => {
                println!("CloseTab");
                let rm_group = tab.value();
                if let Some(active) = rm_group {
                    tab.remove(&active);
                    tab = init_tab(tab);
                    w.redraw();
                }
            }
            Some(Message::CloseWindow) => println!("CloseWindow"),
            Some(Message::ConvertToTagOrType) => println!("ConvertToTagOrType"),
            Some(Message::Copy) => println!("Copy"),
            Some(Message::CopyScreenShotToClipboard) => println!("CopyScreenShotToClipboard"),
            Some(Message::CopyURLOfLocalThought) => println!("CopyURLOfLocalThought"),
            Some(Message::CopyURLOfWebThought) => println!("CopyURLOfWebThought"),
            Some(Message::CreateChild) => println!("CreateChild"),
            Some(Message::CreateLink) => println!("CreateLink"),
            Some(Message::CreateParent) => println!("CreateParent"),
            Some(Message::CreateTag) => println!("CreateTag"),
            Some(Message::CreateType) => println!("CreateType"),
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
            Some(Message::MarkAsDeceased) => println!("MarkAsDeceased"),
            Some(Message::MindmapView) => println!("MindmapView"),
            Some(Message::MoveTabToNewWindow) => println!("MoveTabToNewWindow"),
            Some(Message::NewCerebro) => println!("NewCerebro"),
            Some(Message::NewTab) => {
                println!("NewTab");
                tab = new_tab(tab);
                tab.redraw();
            }
            Some(Message::NewWindow) => println!("NewWindow"),
            Some(Message::NextCerebro) => println!("NextCerebro"),
            Some(Message::OfflineActivation) => println!("OfflineActivation"),
            Some(Message::Open) => println!("Open"),
            Some(Message::OpenCerebroArchive) => println!("OpenCerebroArchive"),
            Some(Message::OpenThoughtInWebClient) => println!("OpenThoughtInWebClient"),
            Some(Message::OpenProtocolDirectory) => println!("OpenProtocolDirectory"),
            Some(Message::OrderCerebrisByDateOfModification) => {
                println!("OrderCerebrisByDateOfModification")
            }
            Some(Message::OrderCerebrisByName) => println!("OrderCerebrisByName"),
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
            Some(Message::RefreshCerebris) => println!("RefreshCerebris"),
            Some(Message::RemovePin) => println!("RemovePin"),
            Some(Message::RemoveTag) => println!("RemoveTag"),
            Some(Message::RemoveType) => println!("RemoveType"),
            Some(Message::RenameCerebro) => println!("RenameCerebro"),
            Some(Message::Save) => println!("Save"),
            Some(Message::SaveAs) => println!("SaveAs"),
            Some(Message::SearchReplace) => println!("SearchReplace"),
            Some(Message::SelectAll) => println!("SelectAll"),
            Some(Message::SelectPathInBetween) => println!("SelectPathInBetween"),
            Some(Message::SelectRelatedThoughts) => println!("SelectRelatedThoughts"),
            Some(Message::SetAsRoot) => println!("SetAsRoot"),
            Some(Message::SetTag) => println!("SetTag"),
            Some(Message::SetType) => println!("SetType"),
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
            Some(Message::ViewHistory) => println!("ViewHistory"),
            Some(Message::ViewProperties) => println!("ViewProperties"),
            Some(Message::Wander) => println!("Wander"),
            Some(Message::WebSearch) => println!("WebSearch"),
            Some(Message::ZoomIntoCerebro) => println!("ZoomIntoCerebro"),
            Some(Message::ZoomOutOfCerebro) => println!("ZoomOutOfCerebro"),
            None => (),
        }
    }
}
