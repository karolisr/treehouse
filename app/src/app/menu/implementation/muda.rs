mod context_menu;
mod events;

use super::super::AppMenuItemId;
use super::super::app_menu_bar::app_menu_bar;
use super::super::menu_model::Accelerator;
use super::super::menu_model::KeyCode;
use super::super::menu_model::Menu;
use super::super::menu_model::MenuItem;
use super::super::menu_model::MenuItemId;
use super::super::menu_model::Modifier;
pub use context_menu::show_tv_context_menu;
pub use events::menu_events;
use muda::MenuItemKind as MIK;

#[derive(Default, Clone)]
pub(crate) struct AppMenu {
    muda_menu: Option<muda::Menu>,
}

impl AppMenu {
    pub(crate) fn new() -> Self {
        // println!("app::menu::AppMenu::new");
        let muda_menu = Self::prepare_app_menu();
        #[cfg(target_os = "macos")]
        muda_menu.init_for_nsapp();
        Self { muda_menu: Some(muda_menu) }
    }

    #[cfg(target_os = "windows")]
    pub fn init_for_hwnd(&self, hwnd: u64) {
        unsafe {
            if let Some(muda_menu) = &self.muda_menu {
                _ = muda_menu.init_for_hwnd(hwnd as isize);
            }
        };
    }

    pub(crate) fn enable(&self, app_menu_item_id: AppMenuItemId) {
        // println!("app::menu::AppMenu -> enable({app_menu_item_id})");
        self.set_enabled(&app_menu_item_id, true);
    }

    pub(crate) fn disable(&self, app_menu_item_id: AppMenuItemId) {
        // println!("app::menu::AppMenu -> disable({app_menu_item_id})");
        self.set_enabled(&app_menu_item_id, false);
    }

    fn set_enabled(&self, app_menu_item_id: &AppMenuItemId, state: bool) {
        if let Some(muda_menu) = &self.muda_menu {
            Self::set_enabled_recursive(
                &muda_menu.items(),
                app_menu_item_id,
                state,
            );
        }
    }

    fn set_enabled_recursive(
        muda_items: &[MIK],
        app_menu_item_id: &AppMenuItemId,
        state: bool,
    ) {
        for mik in muda_items {
            if mik.id() == app_menu_item_id.to_string() {
                match mik {
                    MIK::MenuItem(itm) => itm.set_enabled(state),
                    MIK::Submenu(itm) => itm.set_enabled(state),
                    MIK::Predefined(_) => (),
                    MIK::Check(itm) => itm.set_enabled(state),
                    MIK::Icon(itm) => itm.set_enabled(state),
                }
            } else {
                match mik {
                    MIK::Submenu(muda_submenu) => {
                        Self::set_enabled_recursive(
                            &muda_submenu.items(),
                            app_menu_item_id,
                            state,
                        );
                    }
                    _ => (),
                }
            }
        }
    }

    pub(crate) fn update(&self, app_menu_item_id: AppMenuItemId) {
        // println!("app::menu::AppMenu -> update({app_menu_item_id})");
        if let Some(muda_menu) = &self.muda_menu {
            Self::update_recursive(&muda_menu.items(), &app_menu_item_id);
        }
    }

    fn update_recursive(muda_items: &[MIK], app_menu_item_id: &AppMenuItemId) {
        for mik in muda_items {
            if mik.id() == app_menu_item_id.to_string() {
                match mik {
                    MIK::MenuItem(_) => (),
                    MIK::Submenu(_) => (),
                    MIK::Predefined(_) => (),
                    MIK::Check(_) => (),
                    MIK::Icon(_) => (),
                }
            } else {
                match mik {
                    MIK::Submenu(muda_submenu) => Self::update_recursive(
                        &muda_submenu.items(),
                        app_menu_item_id,
                    ),
                    _ => (),
                }
            }
        }
    }

    fn prepare_app_menu() -> muda::Menu {
        let muda_menu = muda::Menu::default();
        let menu_bar = app_menu_bar();

        menu_bar.items().iter().for_each(|mi| {
            let mik: MIK = mi.clone().into();
            match mik {
                MIK::MenuItem(itm) => _ = muda_menu.append(&itm),
                MIK::Submenu(itm) => _ = muda_menu.append(&itm),
                MIK::Predefined(itm) => _ = muda_menu.append(&itm),
                MIK::Check(itm) => _ = muda_menu.append(&itm),
                MIK::Icon(itm) => _ = muda_menu.append(&itm),
            }
        });

        muda_menu
    }
}

impl From<Menu> for muda::Submenu {
    fn from(menu: Menu) -> Self {
        let submenu_items: Vec<MIK> = menu
            .items()
            .iter()
            .map(|menu_item| menu_item.clone().into())
            .collect();

        let muda_submenu = muda::Submenu::new("", false);

        submenu_items.iter().for_each(|mik| match mik {
            MIK::MenuItem(itm) => _ = muda_submenu.append(itm),
            MIK::Submenu(itm) => _ = muda_submenu.append(itm),
            MIK::Predefined(itm) => _ = muda_submenu.append(itm),
            MIK::Check(itm) => _ = muda_submenu.append(itm),
            MIK::Icon(itm) => _ = muda_submenu.append(itm),
        });

        muda_submenu
    }
}

impl From<MenuItem> for muda::MenuItemKind {
    fn from(itm: MenuItem) -> Self {
        match itm {
            MenuItem::Item { label, enabled, id, accelerator } => {
                let app_menu_item_id: AppMenuItemId = id.clone().into();

                let muda_accelerator = match accelerator {
                    Some(Accelerator { modifier, key }) => {
                        let muda_modifier = modifier.map(|modif| match modif {
                            Modifier::CmdOrCtrl => {
                                muda::accelerator::CMD_OR_CTRL
                            }
                            Modifier::Alt => muda::accelerator::Modifiers::ALT,
                        });

                        let muda_key_code = match key {
                            KeyCode::KeyE => muda::accelerator::Code::KeyE,
                            KeyCode::KeyF => muda::accelerator::Code::KeyF,
                            KeyCode::KeyO => muda::accelerator::Code::KeyO,
                            KeyCode::KeyP => muda::accelerator::Code::KeyP,
                            KeyCode::KeyQ => muda::accelerator::Code::KeyQ,
                            KeyCode::KeyS => muda::accelerator::Code::KeyS,
                            KeyCode::F4 => muda::accelerator::Code::F4,
                        };

                        Some(muda::accelerator::Accelerator::new(
                            muda_modifier, muda_key_code,
                        ))
                    }
                    None => None,
                };

                match app_menu_item_id {
                    AppMenuItemId::About => muda::MenuItemKind::Predefined(
                        muda::PredefinedMenuItem::about(None, None),
                    ),
                    id => {
                        muda::MenuItemKind::MenuItem(muda::MenuItem::with_id(
                            id, label, enabled, muda_accelerator,
                        ))
                    }
                }
            }

            MenuItem::Submenu { label, enabled, id: _, menu } => {
                let muda_submenu: muda::Submenu = menu.into();
                muda_submenu.set_enabled(enabled);
                muda_submenu.set_text(label);
                muda::MenuItemKind::Submenu(muda_submenu)
            }

            MenuItem::Separator => muda::MenuItemKind::Predefined(
                muda::PredefinedMenuItem::separator(),
            ),
        }
    }
}

impl From<muda::MenuItem> for MenuItemId {
    fn from(value: muda::MenuItem) -> Self {
        value.id().0.clone().into()
    }
}

impl From<muda::CheckMenuItem> for MenuItemId {
    fn from(value: muda::CheckMenuItem) -> Self {
        value.id().0.clone().into()
    }
}

impl From<muda::Submenu> for MenuItemId {
    fn from(value: muda::Submenu) -> Self {
        value.id().0.clone().into()
    }
}

impl From<muda::PredefinedMenuItem> for MenuItemId {
    fn from(value: muda::PredefinedMenuItem) -> Self {
        value.id().0.clone().into()
    }
}
