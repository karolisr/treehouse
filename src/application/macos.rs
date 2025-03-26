// Handler fo macOS "application:openFiles:"
// Code adapted from: https://github.com/neovide/neovide/pull/2395

use objc2::{
    MainThreadMarker,
    declare::ClassBuilder,
    msg_send,
    rc::Retained,
    runtime::{AnyClass, AnyObject},
    sel,
};
use objc2_app_kit::NSApplication;
use objc2_foundation::{NSArray, NSString};
use std::ffi::CString;

pub fn macos_register_open_files_handler() {
    use objc2::rc::autoreleasepool;

    extern "C" fn handle_open_files(
        _this: &mut AnyObject,
        _sel: objc2::runtime::Sel,
        _sender: &objc2::runtime::AnyObject,
        files: &mut NSArray<NSString>,
    ) {
        autoreleasepool(|pool| {
            for file in files.iter() {
                let path = unsafe { file.to_str(pool).to_owned() };
                // -------------------------------------
                // piggybacking on muda::MenuEvent::send
                muda::MenuEvent::send(muda::MenuEvent {
                    id: muda::MenuId(path),
                });
                // -------------------------------------
            }
        });
    }

    unsafe {
        let mtm = MainThreadMarker::new_unchecked();
        let app = NSApplication::sharedApplication(mtm);
        let delegate = app.delegate().unwrap();

        // Find out class of the NSApplicationDelegate
        let class: &AnyClass = msg_send![&delegate, class];

        // register subclass of whatever was in delegate
        let adstr = CString::new("hClusterApplicationDelegate").ok().unwrap();
        let mut my_class = ClassBuilder::new(adstr.as_ref(), class).unwrap();
        my_class.add_method(
            sel!(application:openFiles:),
            handle_open_files as unsafe extern "C" fn(_, _, _, _) -> _,
        );
        let class = my_class.register();

        // this should be safe as:
        //  * our class is a subclass
        //  * no new ivars
        //  * overriden methods are compatible with old (we implement protocol method)
        let delegate_obj = Retained::cast_unchecked::<AnyObject>(delegate);
        AnyObject::set_class(&delegate_obj, class);
    }
}
