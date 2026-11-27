#[cfg(feature = "plugins")]
pub mod plugin_directory;
#[cfg(feature = "plugins")]
pub mod plugins;
#[cfg(feature = "plugins")]
pub mod widgets;

/// A build without `plugins` runs no plugin: a server's plugin directory is out of reach.
#[cfg(not(feature = "plugins"))]
pub mod plugin_directory {
    pub const NO_DIRECTORY: &str = "plugins aren't available in this build";
}

/// A build without `plugins` runs no plugin, so no layer is a widget: the layers of a widget saved
/// in a file stay plain layers (locked, inside their frame), and nothing calls a plugin back.
#[cfg(not(feature = "plugins"))]
pub mod widgets {
    use figcraft_doc::{Document, Node, NodeId};
    use figcraft_geom::Point;

    use crate::{Result, Session};

    pub fn is_widget(_: &Node) -> bool {
        false
    }
    pub fn target_at(_: &Document, _: NodeId, _: Point) -> Option<(NodeId, String)> {
        None
    }
    pub fn field_at(_: &Document, _: NodeId, _: Point) -> Option<NodeId> {
        None
    }
    pub fn field_of(_: &Document, _: NodeId) -> Option<(NodeId, String)> {
        None
    }
    pub fn hovered_at(_: &Document, _: NodeId, _: Point) -> Vec<NodeId> {
        Vec::new()
    }

    impl Session {
        pub fn widget_hover(&mut self, _: &[NodeId]) -> Result<bool> {
            Ok(false)
        }
    }
}
