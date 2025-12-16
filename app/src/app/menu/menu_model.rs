use std::{collections::HashMap, fmt::Display};

use riced::Key as RicedKey;
use riced::Modifiers as RicedModifiers;

impl From<RicedKey> for KeyCode {
    fn from(riced_key: RicedKey) -> Self {
        if let RicedKey::Character(k) = riced_key {
            let k: &str = k.as_str();
            match k {
                "e" => KeyCode::KeyE,
                "f" => KeyCode::KeyF,
                "o" => KeyCode::KeyO,
                "p" => KeyCode::KeyP,
                "q" => KeyCode::KeyQ,
                "s" => KeyCode::KeyS,
                _ => KeyCode::Other,
            }
        } else if let RicedKey::Named(named) = riced_key {
            match named {
                riced::KeyName::F4 => KeyCode::F4,
                _ => KeyCode::Other,
            }
        } else {
            KeyCode::Other
        }
    }
}

impl From<RicedModifiers> for Modifier {
    fn from(riced_modifiers: RicedModifiers) -> Self {
        if riced_modifiers == RicedModifiers::COMMAND {
            Modifier::CmdOrCtrl
        } else if riced_modifiers == RicedModifiers::ALT {
            Modifier::Alt
        } else {
            Modifier::Other
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MenuItemId {
    pub id: String,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Hash)]
pub enum Modifier {
    CmdOrCtrl,
    Alt,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Hash)]
pub enum KeyCode {
    KeyE,
    KeyF,
    KeyO,
    KeyP,
    KeyQ,
    KeyS,
    F4,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Hash)]
pub struct Accelerator {
    pub modifier: Option<Modifier>,
    pub key: KeyCode,
}

impl Display for Modifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mod_str = match self {
            Modifier::CmdOrCtrl => {
                #[cfg(any(target_os = "windows", target_os = "linux"))]
                {
                    "Ctrl"
                }
                #[cfg(target_os = "macos")]
                {
                    "Cmd"
                }
            }
            Modifier::Alt => "Alt",
            Modifier::Other => "Other",
        };

        write!(f, "{}", mod_str)
    }
}

impl Display for KeyCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let key_code_str = match self {
            KeyCode::KeyE => "E",
            KeyCode::KeyF => "F",
            KeyCode::KeyO => "O",
            KeyCode::KeyP => "P",
            KeyCode::KeyQ => "Q",
            KeyCode::KeyS => "S",
            KeyCode::F4 => "F4",
            KeyCode::Other => "Other",
        };

        write!(f, "{}", key_code_str)
    }
}

impl Display for Accelerator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mod_str = if let Some(modifier) = self.modifier {
            let mut tmp = modifier.to_string();
            tmp.push('+');
            tmp
        } else {
            "".to_string()
        };
        let key_code_str = self.key.to_string();
        write!(f, "{}{}", mod_str, key_code_str)
    }
}

#[derive(Default, Debug, Clone)]
pub struct Menu {
    items: Vec<MenuItem>,
    accelerator_id_map: HashMap<Accelerator, MenuItemId>,
}

impl Menu {
    pub fn new() -> Self {
        Menu { items: vec![], accelerator_id_map: HashMap::new() }
    }

    pub fn with_items(items: Vec<MenuItem>) -> Self {
        let accelerator_id_map =
            Self::make_accelerator_id_map_recursive(&items);
        Menu { items, accelerator_id_map }
    }

    pub fn append(&mut self, item: MenuItem) {
        Self::make_accelerator_id_map_recursive(std::slice::from_ref(&item))
            .iter()
            .for_each(|(accel, id)| {
                _ = self.accelerator_id_map.insert(*accel, id.clone());
            });

        self.items.push(item);
    }

    fn make_accelerator_id_map_recursive(
        items: &[MenuItem],
    ) -> HashMap<Accelerator, MenuItemId> {
        let mut accelerator_id_map: HashMap<Accelerator, MenuItemId> =
            HashMap::new();
        items.iter().for_each(|itm| match itm {
            MenuItem::Item { accelerator, id, .. } => {
                if let Some(accel) = accelerator {
                    _ = accelerator_id_map.insert(*accel, id.clone());
                }
            }
            MenuItem::Submenu { menu, .. } => {
                Self::make_accelerator_id_map_recursive(menu.items())
                    .iter()
                    .for_each(|(accel, id)| {
                        _ = accelerator_id_map.insert(*accel, id.clone());
                    });
            }
            MenuItem::Separator => {}
        });
        accelerator_id_map
    }

    pub fn menu_item_id_for_accelerator(
        &self,
        accelerator: Accelerator,
    ) -> Option<MenuItemId> {
        let menu_item_id = self.accelerator_id_map.get(&accelerator).cloned();
        if let Some(id) = menu_item_id
            && self.is_enabled(id.clone())
        {
            Some(id)
        } else {
            None
        }
    }

    pub fn items(&self) -> &[MenuItem] {
        &self.items
    }

    pub fn set_enabled(&mut self, menu_item_id: MenuItemId, state: bool) {
        Self::set_enabled_recursive(&mut self.items, menu_item_id, state);
    }

    fn set_enabled_recursive(
        items: &mut [MenuItem],
        menu_item_id: MenuItemId,
        state: bool,
    ) {
        items.iter_mut().for_each(|itm| match itm {
            MenuItem::Item { id, enabled, .. } => {
                if *id == menu_item_id {
                    *enabled = state;
                }
            }
            MenuItem::Submenu { menu, .. } => {
                Self::set_enabled_recursive(
                    &mut menu.items,
                    menu_item_id.clone(),
                    state,
                );
            }
            MenuItem::Separator => {}
        });
    }

    pub fn is_enabled(&self, menu_item_id: MenuItemId) -> bool {
        Self::is_enabled_recursive(&self.items, menu_item_id)
    }

    fn is_enabled_recursive(
        items: &[MenuItem],
        menu_item_id: MenuItemId,
    ) -> bool {
        let mut rv: bool = false;
        for itm in items {
            if rv {
                break;
            }
            match itm {
                MenuItem::Item { id, enabled, .. } => {
                    if *id == menu_item_id {
                        rv = *enabled;
                        break;
                    }
                }
                MenuItem::Submenu { menu, .. } => {
                    rv = Self::is_enabled_recursive(
                        &menu.items,
                        menu_item_id.clone(),
                    );
                }
                MenuItem::Separator => {}
            }
        }
        rv
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
            menu: Menu::with_items(items),
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
