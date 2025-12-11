use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MenuItemId {
    pub id: String,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Modifier {
    CmdOrCtrl,
    Alt,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum KeyCode {
    KeyE,
    KeyF,
    KeyO,
    KeyP,
    KeyQ,
    KeyS,
    F4,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Accelerator {
    pub modifier: Option<Modifier>,
    pub key: KeyCode,
}

#[derive(Default, Debug, Clone)]
pub struct Menu {
    items: Vec<MenuItem>,
}

impl Menu {
    pub fn new() -> Self {
        Menu { items: vec![] }
    }

    pub fn with_items(items: Vec<MenuItem>) -> Self {
        Menu { items }
    }

    pub fn append(&mut self, item: MenuItem) {
        self.items.push(item);
    }

    pub fn items(&self) -> Vec<MenuItem> {
        self.items.clone()
    }
}

#[derive(Debug, Clone)]
pub enum MenuItem {
    Item {
        label: String,
        enabled: bool,
        id: MenuItemId,
        accelerator: Option<Accelerator>,
    },
    Submenu {
        label: String,
        enabled: bool,
        id: MenuItemId,
        menu: Menu,
    },
    Separator,
}

impl MenuItem {
    pub fn item(
        label: impl Into<String>,
        enabled: bool,
        id: impl Into<MenuItemId>,
        accelerator: Option<Accelerator>,
    ) -> Self {
        Self::Item { label: label.into(), enabled, id: id.into(), accelerator }
    }

    pub fn submenu(
        label: impl Into<String>,
        enabled: bool,
        id: impl Into<MenuItemId>,
        items: Vec<MenuItem>,
    ) -> Self {
        Self::Submenu {
            label: label.into(),
            enabled,
            id: id.into(),
            menu: Menu { items },
        }
    }

    pub fn separator() -> Self {
        Self::Separator
    }
}

impl Display for MenuItemId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl From<String> for MenuItemId {
    fn from(s: String) -> Self {
        MenuItemId { id: s }
    }
}
