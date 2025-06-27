/// Creates a vec of menu items
///
/// [`Item`]: crate::menu::Item
///
/// Syntax:
/// ```ignore
/// menu_items!(
///     (widget)
///     (widget)
///     (widget, menu)
///     (widget)
///     (widget, menu)
///     (widget)
///     ...
/// )
/// ```
#[macro_export]
macro_rules! menu_items {
    ($($x:tt)+) => {
        {
            macro_rules! wrap_item {
                (($i:expr , $m:expr)) => (
                    $crate::MenuItem::with_menu($i, $m)
                );
                (($i:expr)) => (
                    $crate::MenuItem::new($i)
                );
            }

            vec![ $( wrap_item!($x) ),+ ]
        }
    }
}

/// Creates a [`Menu`] with the given items.
///
/// [`Menu`]: crate::menu::Menu
///
/// Syntax:
/// ```ignore
/// menu!(
///     (widget)
///     (widget)
///     (widget, menu)
///     (widget)
///     (widget, menu)
///     (widget)
///     ...
/// )
/// ```
#[macro_export]
macro_rules! menu {
    ($($x:tt)+) => {
        $crate::menu::Menu::new( $crate::menu_items!( $($x)+ ) )
    }
}

/// Creates a [`MenuBar`] with the given children.
///
/// [`MenuBar`]: crate::menu::MenuBar
///
/// Syntax:
/// ```ignore
/// menu_bar!(
///     (widget, menu)
///     (widget, menu)
///     (widget, menu)
///     ...
/// )
/// ```
#[macro_export]
macro_rules! menu_bar {
    ($(($x:expr, $m:expr))+) => (
        $crate::MenuBar::new(vec![ $( MenuItem::with_menu($x, $m) ),+ ])
    );
}
