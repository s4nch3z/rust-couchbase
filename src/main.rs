include!("../couchbase-sys/src/lib.rs");

// use ffi;
use std::ffi::CString;

fn main() {
    println!("Hello, world!");

    let err: ffi::lcb_error_t;
    let instance: ffi::lcb_t;
    let mut create_options = ffi::lcb_create_st { version: 0, v: ffi::lcb_CRST_u { _data_: [0; 8]} };
    let scmd = ffi::lcb_store_cmd_t { version: 0, v: ffi::lcb_store_cmd_st_u { _data_: [0; 11] } };
    let scmdlist: *const ffi::lcb_store_cmd_t;
    let gcmd = ffi::lcb_get_cmd_t { version: 0, v: ffi::lcb_get_cmd_st_u { _data_: [0; 6] } };
    create_options.version = 3;

    let mut opts = ffi::lcb_create_st3 {
        connstr: CString::new("couchbase://127.0.0.1:8091/default").unwrap().as_ptr(),
        passwd: CString::new("").unwrap().as_ptr(),
        username: CString::new("").unwrap().as_ptr(),
        _pad_bucket: 0,
        io: 0,
        _type: 0,
    };

    create_options.v.v3() = &opts;
}

// static void
// die(lcb_t instance, const char *msg, lcb_error_t err)
// {
//     fprintf(stderr, "%s. Received code 0x%X (%s)\n",
//         msg, err, lcb_strerror(instance, err));
//     exit(EXIT_FAILURE);
// }
// static void
// store_callback(lcb_t instance, const void *cookie,
//     lcb_storage_t operation, lcb_error_t error, const lcb_store_resp_t *item)
// {
//     if (error == LCB_SUCCESS) {
//         fprintf(stderr, "=== STORED ===\n");
//         fprintf(stderr, "KEY: %.*s\n", (int)item->v.v0.nkey, item->v.v0.key);
//         fprintf(stderr, "CAS: 0x%"PRIx64"\n", item->v.v0.cas);
//     } else {
//         die(instance, "Couldn't store item", error);
//     }
//     (void)operation;
// }
// static void
// get_callback(lcb_t instance, const void *cookie, lcb_error_t error,
//     const lcb_get_resp_t *item)
// {
//     if (error == LCB_SUCCESS) {
//         fprintf(stderr, "=== RETRIEVED ===\n");
//         fprintf(stderr, "KEY: %.*s\n", (int)item->v.v0.nkey, item->v.v0.key);
//         fprintf(stderr, "VALUE: %.*s\n", (int)item->v.v0.nbytes, item->v.v0.bytes);
//         fprintf(stderr, "CAS: 0x%"PRIx64"\n", item->v.v0.cas);
//         fprintf(stderr, "FLAGS: 0x%x\n", item->v.v0.flags);
//     } else {
//         die(instance, "Couldn't retrieve", error);
//     }
//     (void)cookie;
// }
// int main(int argc, char *argv[])
// {
//     lcb_error_t err;
//     lcb_t instance;
//     struct lcb_create_st create_options = { 0 };
//     lcb_store_cmd_t scmd = { 0 };
//     const lcb_store_cmd_t *scmdlist[1];
//     lcb_get_cmd_t gcmd = { 0 };
//     const lcb_get_cmd_t *gcmdlist[1];
//     create_options.version = 3;
//     if (argc < 2) {
//         fprintf(stderr, "Usage: %s couchbase://host/bucket [ password ]\n", argv[0]);
//         exit(EXIT_FAILURE);
//     }
//     create_options.v.v3.connstr = argv[1];
//     if (argc >= 3) {
//         create_options.v.v3.passwd = argv[2];
//     }
//     err = lcb_create(&instance, &create_options);
//     if (err != LCB_SUCCESS) {
//         die(NULL, "Couldn't create couchbase handle", err);
//     }
//     err = lcb_connect(instance);
//     if (err != LCB_SUCCESS) {
//         die(instance, "Couldn't schedule connection", err);
//     }
//     lcb_wait(instance);
//     err = lcb_get_bootstrap_status(instance);
//     if (err != LCB_SUCCESS) {
//         die(instance, "Couldn't bootstrap from cluster", err);
//     }
//     /* Assign the handlers to be called for the operation types */
//     lcb_set_get_callback(instance, get_callback);
//     lcb_set_store_callback(instance, store_callback);
//     scmd.v.v0.operation = LCB_SET;
//     scmd.v.v0.key = "foo"; scmd.v.v0.nkey = 3;
//     scmd.v.v0.bytes = "bar"; scmd.v.v0.nbytes = 3;
//     scmdlist[0] = &scmd;
//     err = lcb_store(instance, NULL, 1, scmdlist);
//     if (err != LCB_SUCCESS) {
//         die(instance, "Couldn't schedule storage operation", err);
//     }
//     /* The store_callback is invoked from lcb_wait() */
//     fprintf(stderr, "Will wait for storage operation to complete..\n");
//     lcb_wait(instance);
//     /* Now fetch the item back */
//     gcmd.v.v0.key = "foo";
//     gcmd.v.v0.nkey = 3;
//     gcmdlist[0] = &gcmd;
//     err = lcb_get(instance, NULL, 1, gcmdlist);
//     if (err != LCB_SUCCESS) {
//         die(instance, "Couldn't schedule retrieval operation", err);
//     }
//     /* Likewise, the get_callback is invoked from here */
//     fprintf(stderr, "Will wait to retrieve item..\n");
//     lcb_wait(instance);
//     /* Now that we're all done, close down the connection handle */
//     lcb_destroy(instance);
//     return 0;
// }