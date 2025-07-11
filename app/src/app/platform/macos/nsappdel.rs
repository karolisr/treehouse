// Handler fo macOS "application:openFiles:"
// Code adapted from: https://github.com/neovide/neovide/pull/2395
// https://developer.apple.com/documentation/appkit/nsapplicationdelegate

use std::{ffi::CString, path::PathBuf};

use objc2::{
    MainThreadMarker,
    declare::ClassBuilder,
    msg_send,
    rc::{Retained, autoreleasepool},
    runtime::{AnyClass, AnyObject},
    sel,
};
use objc2_app_kit::NSApplication;
use objc2_foundation::{NSArray, NSString};

use super::send_os_event;
use crate::app::AppMsg;

pub fn register_ns_application_delegate_handlers() {
    extern "C" fn handle_application_open_files(
        _this: &mut AnyObject,
        _sel: objc2::runtime::Sel,
        _sender: &objc2::runtime::AnyObject,
        files: &mut NSArray<NSString>,
    ) {
        autoreleasepool(|pool| {
            for file in files.iter() {
                let path = unsafe { file.to_str(pool).to_owned() };
                if !path.is_empty() {
                    send_os_event(AppMsg::PathToOpen(Some(PathBuf::from(
                        path,
                    ))));
                }
            }
        });
    }

    unsafe {
        let mtm = MainThreadMarker::new_unchecked();
        let app = NSApplication::sharedApplication(mtm);
        let delegate = app.delegate().unwrap();
        let class: &AnyClass = msg_send![&delegate, class];
        let adstr = CString::new("ApplicationDelegate").ok().unwrap();
        let mut my_class = ClassBuilder::new(adstr.as_ref(), class).unwrap();

        my_class.add_method(
            sel!(application:openFiles:),
            handle_application_open_files
                as unsafe extern "C" fn(_, _, _, _) -> _,
        );

        let class = my_class.register();

        let delegate_obj = Retained::cast_unchecked::<AnyObject>(delegate);
        _ = AnyObject::set_class(&delegate_obj, class);
    }
}
