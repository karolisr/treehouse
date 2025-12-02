use muda::MenuItem;

use riced::RawWindowHandle;
use riced::Task;
use riced::WindowId;
use riced::run;

use treeview::TvContextMenuListing;

use crate::AppMsg;

use super::AppMenuAction;

#[allow(missing_debug_implementations)]
#[derive(Default, Clone)]
pub struct ContextMenu {
    muda_menu: muda::Menu,
}

impl ContextMenu {
    pub fn new() -> Self {
        let muda_menu = muda::Menu::new();
        Self { muda_menu }
    }

    pub fn with_muda_menu(muda_menu: muda::Menu) -> Self {
        Self { muda_menu }
    }
}

pub fn show_context_menu(
    tree_view_context_menu_listing: TvContextMenuListing,
    winid: WindowId,
) -> Task<AppMsg> {
    run(winid, |h| {
        if let Ok(handle) = h.window_handle() {
            let context_menu: ContextMenu =
                tree_view_context_menu_listing.into();
            let muda_menu: muda::Menu = context_menu.into();

            #[cfg(target_os = "macos")]
            unsafe {
                if let RawWindowHandle::AppKit(handle_raw) = handle.as_raw() {
                    _ = muda::ContextMenu::show_context_menu_for_nsview(
                        &muda_menu,
                        handle_raw.ns_view.as_ptr(),
                        None,
                    );
                }
            }
            #[cfg(target_os = "windows")]
            unsafe {
                if let RawWindowHandle::Win32(handle_raw) = handle.as_raw() {
                    _ = muda::ContextMenu::show_context_menu_for_hwnd(
                        &muda_menu,
                        handle_raw.hwnd.into(),
                        None,
                    );
                }
            }
        }
    })
    .discard()
}

impl From<ContextMenu> for muda::Menu {
    fn from(context_menu: ContextMenu) -> Self {
        context_menu.muda_menu
    }
}

impl From<TvContextMenuListing> for ContextMenu {
    fn from(tv_context_menu_listing: TvContextMenuListing) -> Self {
        let muda_menu = muda::Menu::new();
        tv_context_menu_listing.items().iter().enumerate().for_each(
            |(idx, item)| {
                let mii = AppMenuAction::ContextMenuIndex(idx);
                let mmi = MenuItem::with_id(
                    mii,
                    item.label.clone(),
                    item.enabled,
                    None,
                );
                let _ = muda_menu.append(&mmi);
            },
        );
        ContextMenu::with_muda_menu(muda_menu)
    }
}
