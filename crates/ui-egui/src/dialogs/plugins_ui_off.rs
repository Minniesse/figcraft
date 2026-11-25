//! A build without `plugins` runs no plugin: the Plugins menu lists none, no plugin window or
//! panel opens, and a plugin command asked for by name answers that plugins aren't available.
//! The state types match the full module's, so the rest of the UI is the same in both builds.

use egui::{Context, Ui};
use serde_json::{Map, Value};

use crate::FigApp;
use crate::i18n::tr;
use crate::theme::Tokens;

/// The parameters form of a plugin command, with the values entered so far.
#[derive(Clone, Debug, PartialEq)]
pub struct PluginForm {
    pub id: String,
    pub command: String,
    pub values: Map<String, Value>,
}

/// The Manage plugins window's tabs.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PluginsTab {
    #[default]
    Installed,
    Browse,
}

/// The Manage plugins window's state.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PluginsWindow {
    pub tab: PluginsTab,
    pub server: Option<String>,
    pub query: String,
    pub confirm: Option<String>,
}

pub(crate) fn field_value(_: &FigApp, _: &str, _: &str) -> Option<String> {
    None
}

pub(crate) fn field_window(_: &FigApp, _: &Context, _: &str) -> Option<egui::Rect> {
    None
}

pub(crate) fn set_field(_: &mut FigApp, _: &str, _: &str, _: &str) {}

pub fn start(app: &mut FigApp, _: &str, _: &str) {
    app.toast(tr("Plugins aren't available in this build"));
}

pub(crate) fn save_file(app: &mut FigApp, _: &Value) {
    app.toast(tr("Plugins aren't available in this build"));
}

pub fn import_dialog(app: &mut FigApp) {
    app.toast(tr("Plugins aren't available in this build"));
}

pub fn menu(_: &mut FigApp, _: &mut Ui, _: &Tokens) -> bool {
    false
}

pub fn show(app: &mut FigApp, _: &Context) {
    app.ui.plugins_window = false;
    app.ui.plugin_form = None;
}
