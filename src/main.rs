include!("../couchbase-sys/src/lib.rs");

// use ffi;
use std::ffi::CString;

use std::ffi::CStr;
use std::str;
// use libc;
use std::mem::{
    transmute,
    transmute_copy,
};
use std::option::Option;

// pub type lcb_store_callback = Option<extern "C" fn(
//     instance: lcb_t,
//     cookie: *const c_void,
//     operation: lcb_storage_t,
//     error: lcb_error_t,
//     resp: *const lcb_store_resp_t
// ) -> ()>;

extern "C" fn storage_callback(
    instance: ffi::lcb_t,
    cookie: *const libc::c_void,
    operation: ffi::lcb_storage_t,
    error: ffi::lcb_error_t,
    resp: *const ffi::lcb_store_resp_t
) -> () {
    match error {
        ffi::lcb_error_t::LCB_SUCCESS => println!("SET operation success!"),
        _ => println!("Couldn't store!"),
    }
}

// pub type lcb_get_callback = Option<extern "C" fn(
//     instance: lcb_t,
//     cookie: *const c_void,
//     error: lcb_error_t,
//     resp: *const lcb_get_resp_t
// ) -> ()>;

extern "C" fn get_callback(
    instance: ffi::lcb_t,
    cookie: *const libc::c_void,
    error: ffi::lcb_error_t,
    resp: *const ffi::lcb_get_resp_t
) -> () {
    unsafe {
        match error {
            ffi::lcb_error_t::LCB_SUCCESS => {
                println!("GET operation success!");
                println!("get response version: {}", (*resp).version);
                let mut a = (*resp).v;
                let getResp0 = a.v0();
                println!(
                    "key: {}\nvalue: {}\nflags: {}\ncas: {}\ndatatype: {}",
                    str::from_utf8(CStr::from_ptr((*getResp0).key as *const libc::c_char).to_bytes()).unwrap(),
                    str::from_utf8(CStr::from_ptr((*getResp0).bytes as *const libc::c_char).to_bytes()).unwrap(),
                    (*getResp0).flags,
                    (*getResp0).cas,
                    (*getResp0).datatype
                );
            },
            _ => println!("Couldn't get!"),
        }
    }
}

fn main() {
    println!("Hello, world!");

    unsafe {
        let mut cropts = ffi::lcb_create_st { ..Default::default() };
        let connstr = "couchbase://127.0.0.1:8091/default";
        let create3 = ffi::lcb_create_st3 {
            connstr: CString::new(connstr.as_bytes()).unwrap().as_ptr(),
            ..Default::default()
        };
        cropts.version = 3;
        cropts.v._data_ = transmute_copy(&create3);
        println!("create version: {}", cropts.version);
        let create32 = &*cropts.v.v3();
        let slice = CStr::from_ptr(create32.connstr);
        println!("connstr: {}", str::from_utf8(slice.to_bytes()).unwrap());
        let mut err: ffi::lcb_error_t;
        let mut instance: ffi::lcb_t = std::mem::uninitialized();
        err = ffi::lcb_create(&mut instance, &mut cropts);
        match err {
            ffi::lcb_error_t::LCB_SUCCESS => println!("Instance created!"),
            _ => println!("Couldn't create instance!"),
        }
        ffi::lcb_connect(instance);
        ffi::lcb_wait(instance);
        err = lcb_get_bootstrap_status(instance);
        match err {
            ffi::lcb_error_t::LCB_SUCCESS => println!("Bootstrapped!"),
            _ => println!("Couldn't bootstrap!"),
        }

        ffi::lcb_set_store_callback(instance, storage_callback);

        ffi::lcb_set_get_callback(instance, get_callback);

        let mut scmd = ffi::lcb_store_cmd_t { ..Default::default() };
        let storecmdv0 = ffi::lcb_STORECMDv0 {
            key: CString::new("Hello".as_bytes()).unwrap().as_ptr() as *const libc::c_void,
            nkey: 5,
            bytes: CString::new("World".as_bytes()).unwrap().as_ptr() as *const libc::c_void,
            nbytes: 5,
            operation: ffi::lcb_storage_t::LCB_SET,
            ..Default::default()
        };
        scmd.version = 0;
        scmd.v._data_ = transmute_copy(&storecmdv0);
        println!("store version: {}", scmd.version);
        let store0 = &*scmd.v.v0();
        println!("key: {}\nvalue: {}\ncommand: {:?}", str::from_utf8(CStr::from_ptr(store0.key as *const libc::c_char).to_bytes()).unwrap(), str::from_utf8(CStr::from_ptr(store0.bytes as *const libc::c_char).to_bytes()).unwrap(), store0.operation);
        let mut scmdlist:Vec<*const ffi::lcb_store_cmd_t> = vec!(&scmd as *const ffi::lcb_store_cmd_t);
        err = ffi::lcb_store(instance, CString::new("abc".as_bytes()).unwrap().as_ptr() as *const libc::c_void, 1, scmdlist.as_ptr() as *const *const ffi::lcb_store_cmd_t);
        match err {
            ffi::lcb_error_t::LCB_SUCCESS => println!("SET operation scheduled!"),
            _ => println!("Couldn't schedule SET operation!"),
        }
        ffi::lcb_wait(instance);

        let mut gcmd = ffi::lcb_get_cmd_t { ..Default::default() };
        let getcmdv0 = ffi::lcb_GETCMDv0 {
            key: CString::new("Hello".as_bytes()).unwrap().as_ptr() as *const libc::c_void,
            nkey: 5,
            ..Default::default()
        };
        gcmd.version = 0;
        gcmd.v._data_ = transmute_copy(&getcmdv0);
        println!("get version: {}", scmd.version);
        let get0 = &*gcmd.v.v0();
        println!("key: {}", str::from_utf8(CStr::from_ptr(get0.key as *const libc::c_char).to_bytes()).unwrap());
        let mut gcmdlist:Vec<*const ffi::lcb_get_cmd_t> = vec!(&gcmd as *const ffi::lcb_get_cmd_t);
        err = ffi::lcb_get(instance, CString::new("abc".as_bytes()).unwrap().as_ptr() as *const libc::c_void, 1, gcmdlist.as_ptr() as *const *const ffi::lcb_get_cmd_t);
        match err {
            ffi::lcb_error_t::LCB_SUCCESS => println!("GET operation scheduled!"),
            _ => println!("Couldn't schedule GET operation!"),
        }
        ffi::lcb_wait(instance);
        ffi::lcb_destroy(instance);
    }
}