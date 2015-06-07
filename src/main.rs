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

fn main() {
    println!("Hello, world!");

    let mut cropts = ffi::lcb_create_st { ..Default::default() };
    let connstr = "couchbase://127.0.0.1:8091/default";
    let create3 = ffi::lcb_create_st3 {
        connstr: CString::new(connstr.as_bytes()).unwrap().as_ptr(),
        ..Default::default()
    };
    cropts.version = 3;
    unsafe {
        cropts.v._data_ = transmute_copy(&create3);
        println!("version: {}", cropts.version);
       let create32 = &*cropts.v.v3();
       let slice = CStr::from_ptr(create32.connstr);
       println!("v.v3.connstr: {}",
             str::from_utf8(slice.to_bytes()).unwrap());
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
        // ffi::lcb_set_get_callback(instance, get_callback);
        //   lcb_set_storage_callback(instance, storage_callback);
        //   lcb_set_get_callback(instance, get_callback);
          
        //   lcb_store_cmd_t scmd = { 0 };
        //   const lcb_store_cmd_t *scmdlist = &scmd;
        //   scmd.v.v0.key = "Hello";
        //   scmd.v.v0.nkey = 5;
        //   scmd.v.v0.bytes = "World";
        //   scmd.v.v0.nbytes = 5;
        //   scmd.v.v0.operation = LCB_SET;
        //   err = lcb_store(instance, NULL, 1, &scmdlist);
        //   if (err != LCB_SUCCESS) {
        //     printf("Couldn't schedule storage operation!\n");
        //     exit(1);
        //   }
        //   lcb_wait(instance); //storage_callback is invoked here
          
        //   lcb_get_cmd_t gcmd = { 0 };
        //   const lcb_get_cmd_t *gcmdlist = &gcmd;
        //   gcmd.v.v0.key = "Hello";
        //   gcmd.v.v0.nkey = 5;
        //   err = lcb_get(instance, NULL, 1, &gcmdlist);
        //   if (err != LCB_SUCCESS) {
        //     printf("Coudln't schedule get operation!\n");
        //     exit(1);
        //   }
        //   lcb_wait(instance); // get_callback is invoked here
        //   lcb_destroy(instance);
        //   return 0;
    }
}

// static void
// storage_callback(lcb_t instance, const void *cookie, lcb_storage_t op, lcb_error_t err,
//   const lcb_store_resp_t *resp)
// {
//   printf("Stored %.*s\n", (int)resp->v.v0.nkey, resp->v.v0.key);
// }
// static void
// get_callback(lcb_t instance, const void *cookie, lcb_error_t err, lcb_get_resp_t *resp)
// {
//   printf("Retrieved key %.*s\n", (int)resp->v.v0.nkey, resp->v.v0.key);
//   printf("Value is %.*s\n", (int)resp->v.v0.nbytes, resp->v.v0.bytes);
// }

// int main(void)
// {
//   struct lcb_create_st cropts = { 0 };
//   cropts.version = 3;
//   cropts.v.v3.connstr = "couchbase://localhost/default";
//   lcb_error_t err;
//   lcb_t instance;
//   err = lcb_create(&instance, &cropts);
//   if (err != LCB_SUCCESS) {
//     printf("Couldn't create instance!\n");
//     exit(1);
//   }
//   lcb_connect(instance);
//   lcb_wait(instance);
//   if ( (err = lcb_get_bootstrap_status(instance)) != LCB_SUCCESS ) {
//     printf("Couldn't bootstrap!\n");
//     exit(1);
//   }
//   lcb_set_storage_callback(instance, storage_callback);
//   lcb_set_get_callback(instance, get_callback);
  
//   lcb_store_cmd_t scmd = { 0 };
//   const lcb_store_cmd_t *scmdlist = &scmd;
//   scmd.v.v0.key = "Hello";
//   scmd.v.v0.nkey = 5;
//   scmd.v.v0.bytes = "World";
//   scmd.v.v0.nbytes = 5;
//   scmd.v.v0.operation = LCB_SET;
//   err = lcb_store(instance, NULL, 1, &scmdlist);
//   if (err != LCB_SUCCESS) {
//     printf("Couldn't schedule storage operation!\n");
//     exit(1);
//   }
//   lcb_wait(instance); //storage_callback is invoked here
  
//   lcb_get_cmd_t gcmd = { 0 };
//   const lcb_get_cmd_t *gcmdlist = &gcmd;
//   gcmd.v.v0.key = "Hello";
//   gcmd.v.v0.nkey = 5;
//   err = lcb_get(instance, NULL, 1, &gcmdlist);
//   if (err != LCB_SUCCESS) {
//     printf("Coudln't schedule get operation!\n");
//     exit(1);
//   }
//   lcb_wait(instance); // get_callback is invoked here
//   lcb_destroy(instance);
//   return 0;
// }