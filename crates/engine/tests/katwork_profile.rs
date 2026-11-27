//! The KatWork build profile: the engine built without `collab`, `plugins` and `font-download`
//! (`--no-default-features`) has no live-session, plugin or font-download command, and a file
//! whose layers name a plugin still opens and exports. The tests need all three features off: none
//! of them is built otherwise.
#![cfg(not(any(feature = "collab", feature = "plugins", feature = "font-download")))]
#![allow(clippy::unwrap_used, clippy::panic)]

use figcraft_engine::{EngineError, Session};
use serde_json::{Value, json};

/// The command ids each gate removes (every `specs()` of the modules it leaves out).
const COLLAB: &[&str] = &[
    "collab.diff",
    "collab.host",
    "collab.share",
    "collab.stop",
    "collab.join",
    "collab.reconnect",
    "collab.leave",
    "collab.publish",
    "collab.serverInfo",
    "collab.manageFile",
    "collab.report",
    "collab.peers",
    "collab.follow",
    "collab.removePerson",
    "collab.cursor",
    "collab.cursorChat",
    "collab.spotlight",
    "collab.files",
    "collab.openFile",
    "collab.forgetFile",
    "collab.recoverFile",
    "collab.registerLinks",
];
const PLUGINS: &[&str] = &[
    "plugin.list",
    "plugin.import",
    "plugin.remove",
    "plugin.run",
    "plugin.ui",
    "plugin.event",
    "plugin.close",
    "plugins.runLast",
    "plugins.manage",
    "plugin.directory.list",
    "plugin.directory.install",
    "plugin.directory.publish",
    "plugin.directory.unpublish",
    "widget.insert",
    "widget.click",
    "widget.setText",
    "widget.menu",
    "widget.hover",
    "widget.list",
];
const FONT_DOWNLOAD: &[&str] = &["font.download", "font.downloads"];

fn unknown(s: &mut Session, id: &str, params: Value) {
    match s.execute(id, &params) {
        Err(EngineError::UnknownCommand(c)) => assert_eq!(c, id),
        other => panic!("`{id}` should be an unknown command, got {other:?}"),
    }
}

/// The gated ids are neither run, listed, nor in the menus.
fn gone(ids: &[&str], prefixes: &[&str]) {
    let mut s = Session::new();
    for id in ids {
        unknown(&mut s, id, json!({}));
    }
    for c in figcraft_engine::all_commands() {
        assert!(!prefixes.iter().any(|p| c.id.starts_with(p)), "`{}` is still registered", c.id);
    }
    for m in figcraft_engine::catalog::MENU {
        assert!(!ids.contains(&m.id), "the menus still name `{}`", m.id);
    }
}

#[test]
fn collab_and_plugins_are_unknown() {
    let mut s = Session::new();
    // The plan's two: sharing the file and importing a plugin, with real-looking parameters.
    unknown(&mut s, "collab.share", json!({"server": "wss://collab.example.com/"}));
    unknown(&mut s, "plugin.import", json!({"path": "plugin/figcraft-plugin.json"}));
    // Control: the same session still runs a known command.
    assert!(s.execute("shape.rectangle", &json!({"x": 0, "y": 0, "width": 10, "height": 10})).is_ok());
    gone(COLLAB, &["collab."]);
    gone(PLUGINS, &["plugin.", "plugins.", "widget."]);
}

#[test]
fn font_download_is_unknown() {
    gone(FONT_DOWNLOAD, &["font.download"]);
    assert!(!figcraft_engine::fonts::downloads_allowed());
    assert!(figcraft_engine::fonts::allow_downloads().is_err(), "downloads can't be turned on");
    assert_eq!(figcraft_engine::fonts::source("Lato"), None, "no font has a download source");
}

/// A layer saved by a widget (its plugin's id in its layer data) is a plain frame here: the file
/// opens, the layer exports, and nothing calls a plugin.
#[test]
fn a_widget_layer_from_a_file_stays_a_plain_layer() {
    let mut s = Session::new();
    let frame = s.execute("frame.create", &json!({"x": 0, "y": 0, "width": 40, "height": 20, "name": "Counter"})).unwrap();
    let data = json!({"figcraft.widget": "org.example.counter", "figcraft.widget.state": "{\"n\":1}"});
    s.execute("node.set", &json!({"ids": [frame["id"]], "pluginData": data})).unwrap();
    unknown(&mut s, "widget.click", json!({"id": frame["id"], "target": "inc"}));
    let svg = s.execute("file.exportSvg", &json!({"ids": [frame["id"]]}));
    assert!(svg.is_ok(), "the layer exports: {svg:?}");
}
