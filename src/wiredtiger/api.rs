#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!("./", "/bindings.rs"));

pub fn setup_home(home: &str) {
    /* Remove existing test directory and create a new one */
    if std::path::Path::new(home).is_dir() {
        std::fs::remove_dir_all(home).expect("Removal of test dir failed");
    }
    std::fs::create_dir(home).expect("Creation of test dir failed");
}

impl __wt_connection {

    pub unsafe fn close(&mut self, config: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int {
        return (self.close.unwrap())(&mut *self, config);
    }
    pub unsafe fn open_session(
        &mut self,
        event_handler: *mut WT_EVENT_HANDLER,
        config: *const ::std::os::raw::c_char,
        sessionp: *mut *mut WT_SESSION,
    ) -> ::std::os::raw::c_int {
        return (self.open_session.unwrap())(&mut *self, event_handler, config, sessionp);
    }
}

impl __wt_session {
   
    pub unsafe fn close(&mut self, config: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int {
        return (self.close.unwrap())(&mut *self, config);
    }
    pub unsafe fn open_cursor(
        &mut self,
        uri: *const ::std::os::raw::c_char,
        to_dup: *mut WT_CURSOR,
        config: *const ::std::os::raw::c_char,
        cursorp: *mut *mut WT_CURSOR,
    ) -> ::std::os::raw::c_int {
        return (self.open_cursor.unwrap())(&mut *self, uri, to_dup, config, cursorp);
    }
    pub unsafe fn create(
        &mut self,
        name: *const ::std::os::raw::c_char,
        config: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int {
        return (self.create.unwrap())(&mut *self, name, config);
    }
    pub unsafe   fn drop(&mut self,
        name: *const ::std::os::raw::c_char,
        config:*const ::std::os::raw::c_char,
    )-> ::std::os::raw::c_int{
        return (self.drop.unwrap())(&mut *self,name,config);
    }
   
}

impl __wt_cursor {
    /*
    pub unsafe extern "C" fn get_key(&mut self, mut args: ...) -> ::std::os::raw::c_int {
        use std::ffi::VaListImpl;
        let mut ap: VaListImpl;
        let mut aq: VaListImpl;
        ap = args.clone();
        aq = ap.clone();
        return (self.get_key.unwrap()) (&mut *self, aq.as_va_list());
    }
    */
    pub unsafe extern "C" fn get_key(
        &mut self,
        strp: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int {
        return (self.get_key.unwrap())(&mut *self, strp);
    }
    /*
    pub unsafe extern "C" fn get_value(&mut self, mut args: ...) -> ::std::os::raw::c_int {
        use std::ffi::VaListImpl;
        let mut ap: VaListImpl;
        let mut aq: VaListImpl;
        ap = args.clone();
        aq = ap.clone();
        return (self.get_value.unwrap()) (&mut *self, aq.as_va_list());
    }
    */
    pub unsafe extern "C" fn get_value(
        &mut self,
        strp: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int {
        return (self.get_value.unwrap())(&mut *self, strp);
    }
    /*
    pub unsafe extern "C" fn set_key(&mut self, mut args: ...) {
        use std::ffi::VaListImpl;
        let mut ap: VaListImpl;
        let mut aq: VaListImpl;
        ap = args.clone();
        aq = ap.clone();
        return (self.set_key.unwrap()) (&mut *self, aq.as_va_list());
    }
    */
    pub unsafe extern "C" fn set_key(&mut self, strp: *const ::std::os::raw::c_char) {
        return (self.set_key.unwrap())(&mut *self, strp);
    }
    /*
    pub unsafe extern "C" fn set_value(&mut self, mut args: ...) {
        use std::ffi::VaListImpl;
        let mut ap: VaListImpl;
        let mut aq: VaListImpl;
        ap = args.clone();
        aq = ap.clone();
        return (self.set_value.unwrap()) (&mut *self, aq.as_va_list());
    }
    */
    pub unsafe extern "C" fn set_value(&mut self, strp: *const ::std::os::raw::c_char) {
        return (self.set_value.unwrap())(&mut *self, strp);
    }
    pub unsafe fn next(&mut self) -> ::std::os::raw::c_int {
        return (self.next.unwrap())(&mut *self);
    }
    pub unsafe fn prev(&mut self) -> ::std::os::raw::c_int {
        return (self.prev.unwrap())(&mut *self);
    }
    pub unsafe fn reset(&mut self) -> ::std::os::raw::c_int {
        return (self.reset.unwrap())(&mut *self);
    }
    pub unsafe fn search(&mut self) -> ::std::os::raw::c_int {
        return (self.search.unwrap())(&mut *self);
    }
    pub unsafe fn search_near(
        &mut self,
        exactp: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        return (self.search_near.unwrap())(&mut *self, exactp);
    }
    pub unsafe fn insert(&mut self) -> ::std::os::raw::c_int {
        return (self.insert.unwrap())(&mut *self);
    }
    pub unsafe fn update(&mut self) -> ::std::os::raw::c_int {
        return (self.update.unwrap())(&mut *self);
    }
    
    pub unsafe fn remove(&mut self) -> ::std::os::raw::c_int {
        return (self.remove.unwrap())(&mut *self);
    }
    pub unsafe fn close(&mut self) -> ::std::os::raw::c_int {
        return (self.close.unwrap())(&mut *self);
    }
}
