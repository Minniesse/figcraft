//! A build without `collab` and `plugins` (the KatWork build) shows no UI of what it leaves out:
//! no Share button or people, no online files, join field or sharing tip on the Home screen, no
//! sharing server or invite-link setting, no Plugins menu and no Plugins & widgets tab. Each
//! check has a control: something beside it that every build shows.

use egui::Shape;

use crate::live_tests::{Harness, has_text};

fn shown(setup: impl FnOnce(&mut crate::FigApp)) -> Vec<Shape> {
    let (mut h, _) = Harness::new();
    setup(&mut h.app);
    h.frames(3)
}

#[test]
fn the_editor_has_no_share_button() {
    let s = shown(|_| {});
    assert!(has_text(&s, "Design"), "control: the right panel shows");
    assert!(!has_text(&s, "Share"));
}

#[test]
fn home_has_no_online_files_join_field_or_sharing_tip() {
    let s = shown(|app| app.ui.home = true);
    assert!(has_text(&s, "Recents") && has_text(&s, "Templates") && has_text(&s, "Draw a frame"), "control: the Home screen shows");
    assert!(!has_text(&s, "Online"));
    assert!(!has_text(&s, "Join with a code"));
    assert!(!has_text(&s, "Share with a code"));
}

#[test]
fn preferences_have_no_sharing_server_or_invite_links() {
    let s = shown(|app| app.ui.preferences = true);
    assert!(has_text(&s, "Your name"), "control: Preferences show");
    assert!(!has_text(&s, "Server for online sharing"));
    assert!(!has_text(&s, "Open invite links in FigCraft"));
}

#[test]
fn the_main_menu_has_no_plugins_menu() {
    let s = shown(|app| app.ui.main_menu = true);
    assert!(has_text(&s, "File") && has_text(&s, "Libraries"), "control: the main menu shows");
    assert!(!has_text(&s, "Plugins"));
}

#[test]
fn the_actions_palette_has_no_plugins_tab() {
    let s = shown(|app| app.ui.palette = Some(String::new()));
    assert!(has_text(&s, "Assets"), "control: the palette shows");
    assert!(!has_text(&s, "Plugins & widgets"));
}
