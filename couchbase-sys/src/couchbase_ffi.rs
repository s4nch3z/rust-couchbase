/***************
* Enumerations *
****************/
// lcb_DUMPFLAGS : couchbase.h
pub enum lcb_DUMPFLAGS {
    LCB_DUMP_VBCONFIG =  0x01,
    LCB_DUMP_PKTINFO = 0x02,
    LCB_DUMP_BUFINFO = 0x04,
    LCB_DUMP_ALL = 0xff,
}

// lcb_GETNODETYPE : couchbase.h
pub enum lcb_GETNODETYPE {
    LCB_NODE_HTCONFIG = 0x01,
    LCB_NODE_DATA = 0x02,
    LCB_NODE_VIEWS = 0x04,
    LCB_NODE_CONNECTED = 0x08,
    LCB_NODE_NEVERNULL = 0x10,
    LCB_NODE_HTCONFIG_CONNECTED = 0x09,
    LCB_NODE_HTCONFIG_ANY = 0x11,
}

// lcb_http_method_t : couchbase.h
#[derive(Copy)]
pub enum lcb_http_method_t {
    LCB_HTTP_METHOD_GET = 0,
    LCB_HTTP_METHOD_POST = 1,
    LCB_HTTP_METHOD_PUT = 2,
    LCB_HTTP_METHOD_DELETE = 3,
    LCB_HTTP_METHOD_MAX = 4,
}
impl Clone for lcb_http_method_t {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_http_method_t {
    fn default() -> Self { unsafe { zeroed() } }
}

// lcb_http_type_t : couchbase.h
pub enum lcb_http_type_t {
    LCB_HTTP_TYPE_VIEW = 0,
    LCB_HTTP_TYPE_MANAGEMENT = 1,
    LCB_HTTP_TYPE_RAW = 2,
    LCB_HTTP_TYPE_N1QL = 3,
    LCB_HTTP_TYPE_MAX,
}

// lcb_io_ops_type_t : couchbase.h
#[derive(Copy)]
pub enum lcb_io_ops_type_t {
    LCB_IO_OPS_INVALID = 0x00,
    LCB_IO_OPS_DEFAULT = 0x01,
    LCB_IO_OPS_LIBEVENT = 0x02,
    LCB_IO_OPS_WINSOCK = 0x03,
    LCB_IO_OPS_LIBEV = 0x04,
    LCB_IO_OPS_SELECT = 0x05,
    LCB_IO_OPS_WINIOCP = 0x06,
    LCB_IO_OPS_LIBUV = 0x07,
}
impl Clone for lcb_io_ops_type_t {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_io_ops_type_t {
    fn default() -> Self { unsafe { zeroed() } }
}

// lcb_observe_options_t : couchbase.h
#[derive(Copy)]
pub enum lcb_observe_options_t {
    LCB_OBSERVE_MASTER_ONLY = 0x01,
}
impl Clone for lcb_observe_options_t {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_observe_options_t {
    fn default() -> Self { unsafe { zeroed() } }
}

// lcb_observe_t : couchbase.h
#[derive(Copy)]
pub enum lcb_observe_t {
    LCB_OBSERVE_FOUND = 0x00,
    LCB_OBSERVE_PERSISTED = 0x01,
    LCB_OBSERVE_NOT_FOUND = 0x80,
    LCB_OBSERVE_LOGICALLY_DELETED = 0x81,
    LCB_OBSERVE_MAX = 0x82
}
impl Clone for lcb_observe_t {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_observe_t {
    fn default() -> Self { unsafe { zeroed() } }
}

// lcb_replica_t : couchbase.h
#[derive(Copy)]
pub enum lcb_replica_t {
    LCB_REPLICA_FIRST = 0x00,
    LCB_REPLICA_ALL = 0x01,
    LCB_REPLICA_SELECT = 0x02,
}
impl Clone for lcb_replica_t {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_replica_t {
    fn default() -> Self { unsafe { zeroed() } }
}

// lcb_storage_t : couchbase.h
#[derive(Copy)]
pub enum lcb_storage_t {
    LCB_ADD = 0x01,
    LCB_REPLACE = 0x02,
    LCB_SET = 0x03,
    LCB_APPEND = 0x04,
    LCB_PREPEND = 0x05,
}
impl Clone for lcb_storage_t {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_storage_t {
    fn default() -> Self { unsafe { zeroed() } }
}

// lcb_timeunit_t : couchbase.h
pub enum lcb_timeunit_t {
    LCB_TIMEUNIT_NSEC = 0,
    LCB_TIMEUNIT_USEC = 1,
    LCB_TIMEUNIT_MSEC = 2,
    LCB_TIMEUNIT_SEC = 3,
}

// lcb_type_t : couchbase.h
#[derive(Copy)]
pub enum lcb_type_t {
    LCB_TYPE_BUCKET = 0x00,
    LCB_TYPE_CLUSTER = 0x01,
}
impl Clone for lcb_type_t {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_type_t {
    fn default() -> Self { unsafe { zeroed() } }
}


// lcb_VALUEFLAGS : couchbase.h
pub enum lcb_VALUEFLAGS {
    LCB_VALUE_RAW = 0x00,
    LCB_VALUE_F_JSON = 0x01,
    LCB_VALUE_F_SNAPPYCOMP = 0x02,
}

// lcb_verbosity_level_t : couchbase.h
#[derive(Copy)]
pub enum lcb_verbosity_level_t {
    LCB_VERBOSITY_DETAIL = 0x00,
    LCB_VERBOSITY_DEBUG = 0x01,
    LCB_VERBOSITY_INFO = 0x02,
    LCB_VERBOSITY_WARNING = 0x03,
}
impl Clone for lcb_verbosity_level_t {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_verbosity_level_t {
    fn default() -> Self { unsafe { zeroed() } }
}

// lcb_WAITFLAGS : couchbase.h
pub enum lcb_WAITFLAGS {
    LCB_WAIT_DEFAULT = 0x00,
    LCB_WAIT_NOCHECK = 0x01,
}

// lcb_config_transport_t : couchbase.h
#[derive(Copy)]
pub enum lcb_config_transport_t {
    LCB_CONFIG_TRANSPORT_LIST_END = 0,
    LCB_CONFIG_TRANSPORT_HTTP = 1,
    LCB_CONFIG_TRANSPORT_CCCP,
    LCB_CONFIG_TRANSPORT_MAX,
}
impl Clone for lcb_config_transport_t {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_config_transport_t {
    fn default() -> Self { unsafe { zeroed() } }
}

/*******************
* Type definitions *
********************/
// lcb_arithmetic_callback : couchbase.h
pub type lcb_arithmetic_callback = Option<extern "C" fn(
    instance: lcb_t,
    cookie: *const c_void,
    error: lcb_error_t,
    resp: *const lcb_arithmetic_resp_t
) -> ()>;

// lcb_bootstrap_callback : couchbase.h
pub type lcb_bootstrap_callback = Option<extern "C" fn(
    instance: lcb_t,
    err: lcb_error_t
) -> ()>;

// lcb_destroy_callback : couchbase.h
pub type lcb_destroy_callback = Option<extern "C" fn(
    cookie: *const c_void
) -> ()>;

// lcb_durability_callback : couchbase.h
pub type lcb_durability_callback = Option<extern "C" fn(
    instance: lcb_t,
    cookie: *const c_void,
    err: lcb_error_t,
    res: *const lcb_durability_resp_t
) -> ()>;

// // lcb_errmap_callback : couchbase.h
pub type lcb_errmap_callback = Option<extern "C" fn(
    instance: lcb_t,
    bincode: lcb_U16
) -> lcb_error_t>;

// // lcb_flush_callback : couchbase.h
pub type lcb_flush_callback = Option<extern "C" fn(
    instance: lcb_t,
    cookie: *const c_void,
    error: lcb_error_t,
    resp: *const lcb_flush_resp_t
) -> ()>;

// lcb_get_callback : couchbase.h
pub type lcb_get_callback = Option<extern "C" fn(
    instance: lcb_t,
    cookie: *const c_void,
    error: lcb_error_t,
    resp: *const lcb_get_resp_t
) -> ()>;

// lcb_http_res_callback : couchbase.h
pub type lcb_http_res_callback = Option<extern "C" fn(
    request: lcb_http_request_t,
    instance: lcb_t,
    cookie: *const c_void,
    error: lcb_error_t,
    resp: *const lcb_http_resp_t
) -> ()>;

// lcb_observe_callback : couchbase.h
pub type lcb_observe_callback = Option<extern "C" fn(
    instance: lcb_t,
    cookie: *const c_void,
    error: lcb_error_t,
    resp: *const lcb_observe_resp_t
) -> ()>;

// lcb_remove_callback : couchbase.h
pub type lcb_remove_callback = Option<extern "C" fn(
    instance: lcb_t,
    cookie: *const c_void,
    error: lcb_error_t,
    resp: *const lcb_remove_resp_t
) -> ()>;

// lcb_stat_callback : couchbase.h
pub type lcb_stat_callback = Option<extern "C" fn(
    instance: lcb_t,
    cookie: *const c_void,
    error: lcb_error_t,
    resp: *const lcb_server_stat_resp_t
) -> ()>;

// lcb_store_callback : couchbase.h
pub type lcb_store_callback = Option<extern "C" fn(
    instance: lcb_t,
    cookie: *const c_void,
    operation: lcb_storage_t,
    error: lcb_error_t,
    resp: *const lcb_store_resp_t
) -> ()>;

// lcb_timings_callback : couchbase.h
pub type lcb_timings_callback = Option<extern "C" fn(
    instance: lcb_t,
    cookie: *const c_void,
    timeunit: lcb_timeunit_t,
    min: lcb_U32, max: lcb_U32,
    total: lcb_U32, maxtotal: lcb_U32
) -> ()>;

// lcb_touch_callback : couchbase.h
pub type lcb_touch_callback = Option<extern "C" fn(
    instance: lcb_t,
    cookie: *const c_void,
    error: lcb_error_t,
    resp: *const lcb_touch_resp_t
) -> ()>;

// lcb_unlock_callback : couchbase.h
pub type lcb_unlock_callback = Option<extern "C" fn(
    instance: lcb_t,
    cookie: *const c_void,
    error: lcb_error_t,
    resp: *const lcb_unlock_resp_t
) -> ()>;

// lcb_verbosity_callback : couchbase.h
pub type lcb_verbosity_callback = Option<extern "C" fn(
    instance: lcb_t,
    cookie: *const c_void,
    error: lcb_error_t,
    resp: *const lcb_verbosity_resp_t
) -> ()>;

// lcb_version_callback : couchbase.h
pub type lcb_version_callback = Option<extern "C" fn(
    instance: lcb_t,
    cookie: *const c_void,
    error: lcb_error_t,
    resp:
    *const lcb_server_version_resp_t
) -> ()>;

#[repr(C)]
#[derive(Copy)]
pub struct lcb_server_version_resp_st {
    pub version: c_int,
    pub v: lcb_server_version_resp_st_u,
}
impl Clone for lcb_server_version_resp_st {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_server_version_resp_st {
    fn default() -> Self { unsafe { zeroed() } }
}
pub type lcb_server_version_resp_t = lcb_server_version_resp_st;

#[repr(C)]
#[derive(Copy)]
pub struct lcb_server_version_resp_st_u {
    pub _data_: [u64; 3usize],
}
impl lcb_server_version_resp_st_u {
    pub unsafe fn v0(&mut self) -> *mut lcb_server_version_resp_st_u_st {
        let raw: *mut u8 = transmute(&self._data_);
        transmute(raw.offset(0))
    }
}
impl Clone for lcb_server_version_resp_st_u {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_server_version_resp_st_u {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_server_version_resp_st_u_st {
    pub server_endpoint: *const c_char,
    pub vstring: *const c_char,
    pub nvstring: lcb_SIZE,
}
impl Clone for lcb_server_version_resp_st_u_st {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_server_version_resp_st_u_st {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_verbosity_resp_st {
    pub version: c_int,
    pub v: lcb_verbosity_resp_st_u,
}
impl Clone for lcb_verbosity_resp_st {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_verbosity_resp_st {
    fn default() -> Self { unsafe { zeroed() } }
}
pub type lcb_verbosity_resp_t = lcb_verbosity_resp_st;

#[repr(C)]
#[derive(Copy)]
pub struct lcb_verbosity_resp_st_u {
    pub _data_: [u64; 1usize],
}
impl lcb_verbosity_resp_st_u {
    pub unsafe fn v0(&mut self) -> *mut lcb_verbosity_resp_st_u_st {
        let raw: *mut u8 = transmute(&self._data_);
        transmute(raw.offset(0))
    }
}
impl Clone for lcb_verbosity_resp_st_u {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_verbosity_resp_st_u {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_verbosity_resp_st_u_st {
    pub server_endpoint: *const c_char,
}
impl Clone for lcb_verbosity_resp_st_u_st {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_verbosity_resp_st_u_st {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_unlock_resp_t {
    pub version: c_int,
    pub v: lcb_unlock_resp_t_u,
}
impl Clone for lcb_unlock_resp_t {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_unlock_resp_t {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_unlock_resp_t_u {
    pub _data_: [u64; 2usize],
}
impl lcb_unlock_resp_t_u {
    pub unsafe fn v0(&mut self) -> *mut lcb_UNLOCKRESPv0 {
        let raw: *mut u8 = transmute(&self._data_);
        transmute(raw.offset(0))
    }
}
impl Clone for lcb_unlock_resp_t_u {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_unlock_resp_t_u {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_UNLOCKRESPv0 {
    pub key: *const c_void,
    pub nkey: lcb_SIZE,
}
impl Clone for lcb_UNLOCKRESPv0 {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_UNLOCKRESPv0 {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_touch_resp_t {
    pub version: c_int,
    pub v: lcb_touch_resp_t_u,
}
impl Clone for lcb_touch_resp_t {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_touch_resp_t {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_touch_resp_t_u {
    pub _data_: [u64; 3usize],
}
impl lcb_touch_resp_t_u {
    pub unsafe fn v0(&mut self) -> *mut lcb_TOUCHRESPv0 {
        let raw: *mut u8 = transmute(&self._data_);
        transmute(raw.offset(0))
    }
}
impl Clone for lcb_touch_resp_t_u {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_touch_resp_t_u {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_TOUCHRESPv0 {
    pub key: *const c_void,
    pub nkey: lcb_SIZE,
    pub cas: lcb_cas_t,
}
impl Clone for lcb_TOUCHRESPv0 {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_TOUCHRESPv0 {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_store_resp_t {
    pub version: c_int,
    pub v: lcb_store_resp_t_u,
}
impl Clone for lcb_store_resp_t {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_store_resp_t {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_store_resp_t_u {
    pub _data_: [u64; 4usize],
}
impl lcb_store_resp_t_u {
    pub unsafe fn v0(&mut self) -> *mut lcb_STORERESPv0 {
        let raw: *mut u8 = transmute(&self._data_);
        transmute(raw.offset(0))
    }
}
impl Clone for lcb_store_resp_t_u {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_store_resp_t_u {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_STORERESPv0 {
    pub key: *const c_void,
    pub nkey: lcb_SIZE,
    pub cas: lcb_cas_t,
    pub synctoken: *const lcb_SYNCTOKEN,
}
impl Clone for lcb_STORERESPv0 {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_STORERESPv0 {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_server_stat_resp_t {
    pub version: c_int,
    pub v: lcb_server_stat_resp_t_u,
}
impl Clone for lcb_server_stat_resp_t {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_server_stat_resp_t {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_server_stat_resp_t_u {
    pub _data_: [u64; 5usize],
}
impl lcb_server_stat_resp_t_u {
    pub unsafe fn v0(&mut self) -> *mut lcb_STATSRESPv0 {
        let raw: *mut u8 = transmute(&self._data_);
        transmute(raw.offset(0))
    }
}
impl Clone for lcb_server_stat_resp_t_u {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_server_stat_resp_t_u {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_STATSRESPv0 {
    pub server_endpoint: *const c_void,
    pub key: *const c_void,
    pub nkey: lcb_SIZE,
    pub bytes: *const c_void,
    pub nbytes: lcb_SIZE,
}
impl Clone for lcb_STATSRESPv0 {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_STATSRESPv0 {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_remove_resp_t {
    pub version: c_int,
    pub v: lcb_remove_resp_t_u,
}
impl Clone for lcb_remove_resp_t {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_remove_resp_t {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_remove_resp_t_u {
    pub _data_: [u64; 4usize],
}
impl lcb_remove_resp_t_u {
    pub unsafe fn v0(&mut self) -> *mut lcb_REMOVERESPv0 {
        let raw: *mut u8 = transmute(&self._data_);
        transmute(raw.offset(0))
    }
}
impl Clone for lcb_remove_resp_t_u {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_remove_resp_t_u {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_REMOVERESPv0 {
    pub key: *const c_void,
    pub nkey: lcb_SIZE,
    pub cas: lcb_cas_t,
    pub synctoken: *const lcb_SYNCTOKEN,
}
impl Clone for lcb_REMOVERESPv0 {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_REMOVERESPv0 {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_observe_resp_t {
    pub version: c_int,
    pub v: lcb_observe_resp_t_u,
}
impl Clone for lcb_observe_resp_t {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_observe_resp_t {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_observe_resp_t_u {
    pub _data_: [u64; 7usize],
}
impl lcb_observe_resp_t_u {
    pub unsafe fn v0(&mut self) -> *mut lcb_OBSERVERESPv0 {
        let raw: *mut u8 = transmute(&self._data_);
        transmute(raw.offset(0))
    }
}
impl Clone for lcb_observe_resp_t_u {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_observe_resp_t_u {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_OBSERVERESPv0 {
    pub key: *const c_void,
    pub nkey: lcb_SIZE,
    pub cas: lcb_cas_t,
    pub status: lcb_observe_t,
    pub from_master: c_int,
    pub ttp: lcb_time_t,
    pub ttr: lcb_time_t,
}
impl Clone for lcb_OBSERVERESPv0 {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_OBSERVERESPv0 {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_http_resp_t {
    pub version: c_int,
    pub v: lcb_http_resp_t_u,
}
impl Clone for lcb_http_resp_t {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_http_resp_t {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_http_resp_t_u {
    pub _data_: [u64; 6usize],
}
impl lcb_http_resp_t_u {
    pub unsafe fn v0(&mut self) -> *mut lcb_HTTPRESPv0 {
        let raw: *mut u8 = transmute(&self._data_);
        transmute(raw.offset(0))
    }
}
impl Clone for lcb_http_resp_t_u {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_http_resp_t_u {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_HTTPRESPv0 {
    pub status: lcb_http_status_t,
    pub path: *const c_char,
    pub npath: lcb_SIZE,
    pub headers: *const c_char,
    pub bytes: *const c_void,
    pub nbytes: lcb_SIZE,
}
impl Clone for lcb_HTTPRESPv0 {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_HTTPRESPv0 {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_http_request_st;
impl Clone for lcb_http_request_st {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_http_request_st {
    fn default() -> Self { unsafe { zeroed() } }
}
pub type lcb_http_request_t = *mut lcb_http_request_st;

#[repr(C)]
#[derive(Copy)]
pub struct lcb_get_resp_t {
    pub version: c_int,
    pub v: lcb_get_resp_t_u,
}
impl Clone for lcb_get_resp_t {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_get_resp_t {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_get_resp_t_u {
    pub _data_: [u64; 7usize],
}
impl lcb_get_resp_t_u {
    pub unsafe fn v0(&mut self) -> *mut lcb_GETRESPv0 {
        let raw: *mut u8 = transmute(&self._data_);
        transmute(raw.offset(0))
    }
}
impl Clone for lcb_get_resp_t_u {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_get_resp_t_u {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_GETRESPv0 {
    pub key: *const c_void,
    pub nkey: lcb_SIZE,
    pub bytes: *const c_void,
    pub nbytes: lcb_SIZE,
    pub flags: lcb_U32,
    pub cas: lcb_cas_t,
    pub datatype: lcb_U8,
}
impl Clone for lcb_GETRESPv0 {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_GETRESPv0 {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_flush_resp_t {
    pub version: c_int,
    pub v: lcb_flush_resp_t_u,
}
impl Clone for lcb_flush_resp_t {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_flush_resp_t {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_flush_resp_t_u {
    pub _data_: [u64; 1usize],
}
impl lcb_flush_resp_t_u {
    pub unsafe fn v0(&mut self) -> *mut lcb_flush_resp_t_u_st {
        let raw: *mut u8 = transmute(&self._data_);
        transmute(raw.offset(0))
    }
}
impl Clone for lcb_flush_resp_t_u {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_flush_resp_t_u {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_flush_resp_t_u_st {
    pub server_endpoint: *const c_char,
}
impl Clone for lcb_flush_resp_t_u_st {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_flush_resp_t_u_st {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_durability_resp_st {
    pub version: c_int,
    pub v: lcb_durability_resp_st_u,
}
impl Clone for lcb_durability_resp_st {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_durability_resp_st {
    fn default() -> Self { unsafe { zeroed() } }
}
pub type lcb_durability_resp_t = lcb_durability_resp_st;

#[repr(C)]
#[derive(Copy)]
pub struct lcb_durability_resp_st_u {
    pub _data_: [u64; 9usize],
}
impl lcb_durability_resp_st_u {
    pub unsafe fn v0(&mut self) -> *mut lcb_DURABILITYRESPv0 {
        let raw: *mut u8 = transmute(&self._data_);
        transmute(raw.offset(0))
    }
}
impl Clone for lcb_durability_resp_st_u {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_durability_resp_st_u {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_DURABILITYRESPv0 {
    pub key: *const c_void,
    pub nkey: lcb_SIZE,
    pub err: lcb_error_t,
    pub cas: lcb_cas_t,
    pub persisted_master: c_uchar,
    pub exists_master: c_uchar,
    pub npersisted: c_uchar,
    pub nreplicated: c_uchar,
    pub nresponses: c_ushort,
}
impl Clone for lcb_DURABILITYRESPv0 {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_DURABILITYRESPv0 {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_arithmetic_resp_t {
    pub version: c_int,
    pub v: lcb_arithmetic_resp_t_u,
}
impl Clone for lcb_arithmetic_resp_t {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_arithmetic_resp_t {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_ARITHRESPv0 {
    pub key: *mut c_void,
    pub nkey: lcb_SIZE,
    pub cas: lcb_cas_t,
    pub synctoken: *const lcb_SYNCTOKEN,
}
impl Clone for lcb_ARITHRESPv0 {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_ARITHRESPv0 {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_SYNCTOKEN {
    pub uuid_: lcb_U64,
    pub seqno_: lcb_U64,
    pub vbid_: lcb_U16,
}
impl Clone for lcb_SYNCTOKEN {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_SYNCTOKEN {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_arithmetic_resp_t_u {
    pub _data_: [u64; 4usize],
}
impl lcb_arithmetic_resp_t_u {
    pub unsafe fn v0(&mut self) -> *mut lcb_ARITHRESPv0 {
        let raw: *mut u8 = transmute(&self._data_);
        transmute(raw.offset(0))
    }
}
impl Clone for lcb_arithmetic_resp_t_u {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_arithmetic_resp_t_u {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_st;
impl Clone for lcb_st {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_st {
    fn default() -> Self { unsafe { zeroed() } }
}
pub type lcb_t = *mut lcb_st;

#[repr(C)]
#[derive(Copy)]
pub struct lcb_create_st0 {
    pub host: *const c_char,
    pub user: *const c_char,
    pub passwd: *const c_char,
    pub bucket: *const c_char,
    pub io: *mut lcb_io_opt_st,
}
impl Clone for lcb_create_st0 {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_create_st0 {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_create_st1 {
    pub host: *const c_char,
    pub user: *const c_char,
    pub passwd: *const c_char,
    pub bucket: *const c_char,
    pub io: *mut lcb_io_opt_st,
    pub _type: lcb_type_t,
}
impl Clone for lcb_create_st1 {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_create_st1 {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_create_st2 {
    pub host: *const c_char,
    pub user: *const c_char,
    pub passwd: *const c_char,
    pub bucket: *const c_char,
    pub io: *mut lcb_io_opt_st,
    pub _type: lcb_type_t,
    pub mchosts: *const c_char,
    pub transports: *const lcb_config_transport_t,
}
impl Clone for lcb_create_st2 {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_create_st2 {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_create_st3 {
    pub connstr: *const c_char,
    pub username: *const c_char,
    pub passwd: *const c_char,
    pub _pad_bucket: *mut c_void,
    pub io: *mut lcb_io_opt_st,
    pub _type: lcb_type_t,
}
impl Clone for lcb_create_st3 {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_create_st3 {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_CRST_u {
    pub _data_: [u64; 8usize],
}
impl lcb_CRST_u {
    pub unsafe fn v0(&mut self) -> *mut lcb_create_st0 {
        let raw: *mut u8 = transmute(&self._data_);
        transmute(raw.offset(0))
    }
    pub unsafe fn v1(&mut self) -> *mut lcb_create_st1 {
        let raw: *mut u8 = transmute(&self._data_);
        transmute(raw.offset(0))
    }
    pub unsafe fn v2(&mut self) -> *mut lcb_create_st2 {
        let raw: *mut u8 = transmute(&self._data_);
        transmute(raw.offset(0))
    }
    pub unsafe fn v3(&mut self) -> *mut lcb_create_st3 {
        let raw: *mut u8 = transmute(&self._data_);
        transmute(raw.offset(0))
    }
}
impl Clone for lcb_CRST_u {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_CRST_u {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_create_st {
    pub version: c_int,
    pub v: lcb_CRST_u,
}
impl Clone for lcb_create_st {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_create_st {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_arithmetic_cmd_st {
    pub version: c_int,
    pub v: lcb_arithmetic_cmd_st_u,
}
impl Clone for lcb_arithmetic_cmd_st {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_arithmetic_cmd_st {
    fn default() -> Self { unsafe { zeroed() } }
}
pub type lcb_arithmetic_cmd_t = lcb_arithmetic_cmd_st;

#[repr(C)]
#[derive(Copy)]
pub struct lcb_arithmetic_cmd_st_u {
    pub _data_: [u64; 8usize],
}
impl lcb_arithmetic_cmd_st_u {
    pub unsafe fn v0(&mut self) -> *mut lcb_ARITHCMDv0 {
        let raw: *mut u8 = transmute(&self._data_);
        transmute(raw.offset(0))
    }
}
impl Clone for lcb_arithmetic_cmd_st_u {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_arithmetic_cmd_st_u {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_ARITHCMDv0 {
    pub key: *const c_void,
    pub nkey: lcb_SIZE,
    pub exptime: lcb_time_t,
    pub create: c_int,
    pub delta: lcb_S64,
    pub initial: lcb_U64,
    pub hashkey: *const c_void,
    pub nhashkey: lcb_SIZE,
}
impl Clone for lcb_ARITHCMDv0 {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_ARITHCMDv0 {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_unlock_cmd_st {
    pub version: c_int,
    pub v: lcb_unlock_cmd_st_u,
}
impl Clone for lcb_unlock_cmd_st {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_unlock_cmd_st {
    fn default() -> Self { unsafe { zeroed() } }
}
pub type lcb_unlock_cmd_t = lcb_unlock_cmd_st;

#[repr(C)]
#[derive(Copy)]
pub struct lcb_unlock_cmd_st_u {
    pub _data_: [u64; 5usize],
}
impl lcb_unlock_cmd_st_u {
    pub unsafe fn v0(&mut self) -> *mut lcb_UNLOCKCMDv0 {
        let raw: *mut u8 = transmute(&self._data_);
        transmute(raw.offset(0))
    }
}
impl Clone for lcb_unlock_cmd_st_u {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_unlock_cmd_st_u {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_UNLOCKCMDv0 {
    pub key: *const c_void,
    pub nkey: lcb_SIZE,
    pub cas: lcb_cas_t,
    pub hashkey: *const c_void,
    pub nhashkey: lcb_SIZE,
}
impl Clone for lcb_UNLOCKCMDv0 {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_UNLOCKCMDv0 {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_get_cmd_st {
    pub version: c_int,
    pub v: lcb_get_cmd_st_u,
}
impl Clone for lcb_get_cmd_st {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_get_cmd_st {
    fn default() -> Self { unsafe { zeroed() } }
}
pub type lcb_get_cmd_t = lcb_get_cmd_st;
pub type lcb_touch_cmd_t = lcb_get_cmd_t;

#[repr(C)]
#[derive(Copy)]
pub struct lcb_get_cmd_st_u {
    pub _data_: [u64; 6usize],
}
impl lcb_get_cmd_st_u {
    pub unsafe fn v0(&mut self) -> *mut lcb_GETCMDv0 {
        let raw: *mut u8 = transmute(&self._data_);
        transmute(raw.offset(0))
    }
}
impl Clone for lcb_get_cmd_st_u {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_get_cmd_st_u {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_GETCMDv0 {
    pub key: *const c_void,
    pub nkey: lcb_SIZE,
    pub exptime: lcb_time_t,
    pub lock: c_int,
    pub hashkey: *const c_void,
    pub nhashkey: lcb_SIZE,
}
impl Clone for lcb_GETCMDv0 {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_GETCMDv0 {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_store_cmd_st {
    pub version: c_int,
    pub v: lcb_store_cmd_st_u,
}
impl Clone for lcb_store_cmd_st {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_store_cmd_st {
    fn default() -> Self { unsafe { zeroed() } }
}
pub type lcb_store_cmd_t = lcb_store_cmd_st;

#[repr(C)]
#[derive(Copy)]
pub struct lcb_store_cmd_st_u {
    pub _data_: [u64; 11usize],
}
impl lcb_store_cmd_st_u {
    pub unsafe fn v0(&mut self) -> *mut lcb_STORECMDv0 {
        let raw: *mut u8 = transmute(&self._data_);
        transmute(raw.offset(0))
    }
}
impl Clone for lcb_store_cmd_st_u {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_store_cmd_st_u {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_STORECMDv0 {
    pub key: *const c_void,
    pub nkey: lcb_SIZE,
    pub bytes: *const c_void,
    pub nbytes: lcb_SIZE,
    pub flags: lcb_U32,
    pub cas: lcb_cas_t,
    pub datatype: lcb_U8,
    pub exptime: lcb_time_t,
    pub operation: lcb_storage_t,
    pub hashkey: *const c_void,
    pub nhashkey: lcb_SIZE,
}
impl Clone for lcb_STORECMDv0 {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_STORECMDv0 {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_verbosity_cmd_st {
    pub version: c_int,
    pub v: lcb_verbosity_cmd_st_u,
}
impl Clone for lcb_verbosity_cmd_st {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_verbosity_cmd_st {
    fn default() -> Self { unsafe { zeroed() } }
}
pub type lcb_verbosity_cmd_t = lcb_verbosity_cmd_st;

#[repr(C)]
#[derive(Copy)]
pub struct lcb_verbosity_cmd_st_u {
    pub _data_: [u64; 2usize],
}
impl lcb_verbosity_cmd_st_u {
    pub unsafe fn v0(&mut self) -> *mut lcb_VERBOSITYCMDv0 {
        let raw: *mut u8 = transmute(&self._data_);
        transmute(raw.offset(0))
    }
}
impl Clone for lcb_verbosity_cmd_st_u {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_verbosity_cmd_st_u {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_VERBOSITYCMDv0 {
    pub server: *const c_char,
    pub level: lcb_verbosity_level_t,
}
impl Clone for lcb_VERBOSITYCMDv0 {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_VERBOSITYCMDv0 {
    fn default() -> Self { unsafe { zeroed() } }
}

pub type lcb_http_data_callback = lcb_http_res_callback;
pub type lcb_http_complete_callback = lcb_http_res_callback;

#[repr(C)]
#[derive(Copy)]
pub struct lcb_server_version_cmd_st {
    pub version: c_int,
    pub v: lcb_server_version_cmd_st_u,
}
impl Clone for lcb_server_version_cmd_st {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_server_version_cmd_st {
    fn default() -> Self { unsafe { zeroed() } }
}
pub type lcb_server_version_cmd_t = lcb_server_version_cmd_st;

#[repr(C)]
#[derive(Copy)]
pub struct lcb_server_version_cmd_st_u {
    pub _data_: [u64; 1usize],
}
impl lcb_server_version_cmd_st_u {
    pub unsafe fn v0(&mut self) -> *mut lcb_server_version_cmd_st_u_st {
        let raw: *mut u8 = transmute(&self._data_);
        transmute(raw.offset(0))
    }
}
impl Clone for lcb_server_version_cmd_st_u {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_server_version_cmd_st_u {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_server_version_cmd_st_u_st {
    pub notused: *const c_void,
}
impl Clone for lcb_server_version_cmd_st_u_st {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_server_version_cmd_st_u_st {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_server_stats_cmd_st {
    pub version: c_int,
    pub v: lcb_server_stats_cmd_st_u,
}
impl Clone for lcb_server_stats_cmd_st {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_server_stats_cmd_st {
    fn default() -> Self { unsafe { zeroed() } }
}
pub type lcb_server_stats_cmd_t = lcb_server_stats_cmd_st;

#[repr(C)]
#[derive(Copy)]
pub struct lcb_server_stats_cmd_st_u {
    pub _data_: [u64; 2usize],
}
impl lcb_server_stats_cmd_st_u {
    pub unsafe fn v0(&mut self) -> *mut lcb_STATSCMDv0 {
        let raw: *mut u8 = transmute(&self._data_);
        transmute(raw.offset(0))
    }
}
impl Clone for lcb_server_stats_cmd_st_u {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_server_stats_cmd_st_u {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_STATSCMDv0 {
    pub name: *const c_void,
    pub nname: lcb_SIZE,
}
impl Clone for lcb_STATSCMDv0 {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_STATSCMDv0 {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_remove_cmd_st {
    pub version: c_int,
    pub v: lcb_remove_cmd_st_u,
}
impl Clone for lcb_remove_cmd_st {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_remove_cmd_st {
    fn default() -> Self { unsafe { zeroed() } }
}
pub type lcb_remove_cmd_t = lcb_remove_cmd_st;

#[repr(C)]
#[derive(Copy)]
pub struct lcb_remove_cmd_st_u {
    pub _data_: [u64; 5usize],
}
impl lcb_remove_cmd_st_u {
    pub unsafe fn v0(&mut self) -> *mut lcb_REMOVECMDv0 {
        let raw: *mut u8 = transmute(&self._data_);
        transmute(raw.offset(0))
    }
}
impl Clone for lcb_remove_cmd_st_u {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_remove_cmd_st_u {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_REMOVECMDv0 {
    pub key: *const c_void,
    pub nkey: lcb_SIZE,
    pub cas: lcb_cas_t,
    pub hashkey: *const c_void,
    pub nhashkey: lcb_SIZE,
}
impl Clone for lcb_REMOVECMDv0 {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_REMOVECMDv0 {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_observe_cmd_st {
    pub version: c_int,
    pub v: lcb_observe_cmd_st_u,
}
impl Clone for lcb_observe_cmd_st {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_observe_cmd_st {
    fn default() -> Self { unsafe { zeroed() } }
}
pub type lcb_observe_cmd_t = lcb_observe_cmd_st;

#[repr(C)]
#[derive(Copy)]
pub struct lcb_observe_cmd_st_u {
    pub _data_: [u64; 5usize],
}
impl lcb_observe_cmd_st_u {
    pub unsafe fn v0(&mut self) -> *mut lcb_OBSERVECMDv0 {
        let raw: *mut u8 = transmute(&self._data_);
        transmute(raw.offset(0))
    }
    pub unsafe fn v1(&mut self) -> *mut lcb_OBSERVECMDv1 {
        let raw: *mut u8 = transmute(&self._data_);
        transmute(raw.offset(0))
    }
}
impl Clone for lcb_observe_cmd_st_u {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_observe_cmd_st_u {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_OBSERVECMDv0 {
    pub key: *const c_void,
    pub nkey: lcb_SIZE,
    pub hashkey: *const c_void,
    pub nhashkey: lcb_SIZE,
}
impl Clone for lcb_OBSERVECMDv0 {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_OBSERVECMDv0 {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_OBSERVECMDv1 {
    pub key: *const c_void,
    pub nkey: lcb_SIZE,
    pub hashkey: *const c_void,
    pub nhashkey: lcb_SIZE,
    pub options: lcb_observe_options_t,
}
impl Clone for lcb_OBSERVECMDv1 {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_OBSERVECMDv1 {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_http_cmd_st {
    pub version: c_int,
    pub v: lcb_http_cmd_st_u,
}
impl Clone for lcb_http_cmd_st {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_http_cmd_st {
    fn default() -> Self { unsafe { zeroed() } }
}
pub type lcb_http_cmd_t = lcb_http_cmd_st;

#[repr(C)]
#[derive(Copy)]
pub struct lcb_http_cmd_st_u {
    pub _data_: [u64; 10usize],
}
impl lcb_http_cmd_st_u {
    pub unsafe fn v0(&mut self) -> *mut lcb_HTTPCMDv0 {
        let raw: *mut u8 = transmute(&self._data_);
        transmute(raw.offset(0))
    }
    pub unsafe fn v1(&mut self) -> *mut lcb_HTTPCMDv1 {
        let raw: *mut u8 = transmute(&self._data_);
        transmute(raw.offset(0))
    }
}
impl Clone for lcb_http_cmd_st_u {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_http_cmd_st_u {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_HTTPCMDv0 {
    pub path: *const c_char,
    pub npath: lcb_SIZE,
    pub body: *const c_void,
    pub nbody: lcb_SIZE,
    pub method: lcb_http_method_t,
    pub chunked: c_int,
    pub content_type: *const c_char,
}
impl Clone for lcb_HTTPCMDv0 {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_HTTPCMDv0 {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_HTTPCMDv1 {
    pub path: *const c_char,
    pub npath: lcb_SIZE,
    pub body: *const c_void,
    pub nbody: lcb_SIZE,
    pub method: lcb_http_method_t,
    pub chunked: c_int,
    pub content_type: *const c_char,
    pub host: *const c_char,
    pub username: *const c_char,
    pub password: *const c_char,
}
impl Clone for lcb_HTTPCMDv1 {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_HTTPCMDv1 {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_get_replica_cmd_st {
    pub version: c_int,
    pub v: lcb_get_replica_cmd_st_u,
}
impl Clone for lcb_get_replica_cmd_st {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_get_replica_cmd_st {
    fn default() -> Self { unsafe { zeroed() } }
}
pub type lcb_get_replica_cmd_t = lcb_get_replica_cmd_st;

#[repr(C)]
#[derive(Copy)]
pub struct lcb_get_replica_cmd_st_u {
    pub _data_: [u64; 6usize],
}
impl lcb_get_replica_cmd_st_u {
    pub unsafe fn v0(&mut self) -> *mut lcb_GETREPLICACMDv0 {
        let raw: *mut u8 = transmute(&self._data_);
        transmute(raw.offset(0))
    }
    pub unsafe fn v1(&mut self) -> *mut lcb_GETREPLICACMDv1 {
        let raw: *mut u8 = transmute(&self._data_);
        transmute(raw.offset(0))
    }
}
impl Clone for lcb_get_replica_cmd_st_u {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_get_replica_cmd_st_u {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_GETREPLICACMDv0 {
    pub key: *const c_void,
    pub nkey: lcb_SIZE,
    pub hashkey: *const c_void,
    pub nhashkey: lcb_SIZE,
}
impl Clone for lcb_GETREPLICACMDv0 {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_GETREPLICACMDv0 {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_GETREPLICACMDv1 {
    pub key: *const c_void,
    pub nkey: lcb_SIZE,
    pub hashkey: *const c_void,
    pub nhashkey: lcb_SIZE,
    pub strategy: lcb_replica_t,
    pub index: c_int,
}
impl Clone for lcb_GETREPLICACMDv1 {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_GETREPLICACMDv1 {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_flush_cmd_st {
    pub version: c_int,
    pub v: lcb_flush_cmd_st_u,
}
impl Clone for lcb_flush_cmd_st {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_flush_cmd_st {
    fn default() -> Self { unsafe { zeroed() } }
}
pub type lcb_flush_cmd_t = lcb_flush_cmd_st;

#[repr(C)]
#[derive(Copy)]
pub struct lcb_flush_cmd_st_u {
    pub _data_: [u64; 1usize],
}
impl lcb_flush_cmd_st_u {
    pub unsafe fn v0(&mut self) -> *mut lcb_flush_cmd_st_u_st {
        let raw: *mut u8 = transmute(&self._data_);
        transmute(raw.offset(0))
    }
}
impl Clone for lcb_flush_cmd_st_u {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_flush_cmd_st_u {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_flush_cmd_st_u_st {
    pub unused: c_int,
}
impl Clone for lcb_flush_cmd_st_u_st {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_flush_cmd_st_u_st {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_durability_cmd_st {
    pub version: c_int,
    pub v: lcb_durability_cmd_st_u,
}
impl Clone for lcb_durability_cmd_st {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_durability_cmd_st {
    fn default() -> Self { unsafe { zeroed() } }
}
pub type lcb_durability_cmd_t = lcb_durability_cmd_st;

#[repr(C)]
#[derive(Copy)]
pub struct lcb_durability_cmd_st_u {
    pub _data_: [u64; 6usize],
}
impl lcb_durability_cmd_st_u {
    pub unsafe fn v0(&mut self) -> *mut lcb_DURABILITYCMDv0 {
        let raw: *mut u8 = transmute(&self._data_);
        transmute(raw.offset(0))
    }
}
impl Clone for lcb_durability_cmd_st_u {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_durability_cmd_st_u {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_DURABILITYCMDv0 {
    pub key: *const c_void,
    pub nkey: lcb_SIZE,
    pub hashkey: *const c_void,
    pub nhashkey: lcb_SIZE,
    pub cas: lcb_cas_t,
    pub synctok: *const lcb_SYNCTOKEN,
}
impl Clone for lcb_DURABILITYCMDv0 {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_DURABILITYCMDv0 {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_durability_opts_st {
    pub version: c_int,
    pub v: lcb_durability_opts_st_u,
}
impl Clone for lcb_durability_opts_st {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_durability_opts_st {
    fn default() -> Self { unsafe { zeroed() } }
}
pub type lcb_durability_opts_t = lcb_durability_opts_st;

#[repr(C)]
#[derive(Copy)]
pub struct lcb_durability_opts_st_u {
    pub _data_: [u64; 7usize],
}
impl lcb_durability_opts_st_u {
    pub unsafe fn v0(&mut self) -> *mut lcb_DURABILITYOPTSv0 {
        let raw: *mut u8 = transmute(&self._data_);
        transmute(raw.offset(0))
    }
}
impl Clone for lcb_durability_opts_st_u {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_durability_opts_st_u {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_DURABILITYOPTSv0 {
    pub timeout: lcb_U32,
    pub interval: lcb_U32,
    pub persist_to: lcb_U16,
    pub replicate_to: lcb_U16,
    pub check_delete: lcb_U8,
    pub cap_max: lcb_U8,
    pub pollopts: lcb_U8,
}
impl Clone for lcb_DURABILITYOPTSv0 {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_DURABILITYOPTSv0 {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_create_io_ops_st {
    pub version: c_int,
    pub v: lcb_create_io_ops_st_u,
}
impl Clone for lcb_create_io_ops_st {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_create_io_ops_st {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_create_io_ops_st_u {
    pub _data_: [u64; 2usize],
}
impl lcb_create_io_ops_st_u {
    pub unsafe fn v0(&mut self) -> *mut lcb_IOCREATEOPTS_BUILTIN {
        let raw: *mut u8 = transmute(&self._data_);
        transmute(raw.offset(0))
    }
    // These look like depricated
    /*
    pub unsafe fn v1(&mut self) -> *mut lcb_IOCREATEOPTS_DSO {
        let raw: *mut u8 = transmute(&self._data_);
        transmute(raw.offset(0))
    }
    pub unsafe fn v2(&mut self) -> *mut lcb_IOCREATEOPS_FUNCTIONPOINTER {
        let raw: *mut u8 = transmute(&self._data_);
        transmute(raw.offset(0))
    }
    */
}
impl Clone for lcb_create_io_ops_st_u {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_create_io_ops_st_u {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_IOCREATEOPTS_BUILTIN {
    pub _type: lcb_io_ops_type_t,
    pub cookie: *mut c_void,
}
impl Clone for lcb_IOCREATEOPTS_BUILTIN {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_IOCREATEOPTS_BUILTIN {
    fn default() -> Self { unsafe { zeroed() } }
}

/************
* Variables *
*************/
#[link(name = "couchbase")]
extern "C" {
    pub static lcb_version_g: lcb_U32;
}

/************
* Functions *
*************/
#[link(name = "couchbase")]
extern "C" {

    // lcb_arithmetic() : couchbase.h
    pub fn lcb_arithmetic(
        instance: lcb_t,
        command_cookie: *const c_void,
        num: lcb_SIZE,
        commands: *const *const lcb_arithmetic_cmd_t
    ) -> lcb_error_t;

    // lcb_breakout() : couchbase.h
    pub fn lcb_breakout(instance: lcb_t) -> ();

    // lcb_cancel_http_request() : couchbase.h
    pub fn lcb_cancel_http_request(
        instance: lcb_t,
        request: lcb_http_request_t
    ) -> ();

    // lcb_cntl() : couchbase.h
    pub fn lcb_cntl(
        instance: lcb_t,
        mode: c_int,
        cmd: c_int,
        arg: *mut c_void
    ) -> lcb_error_t;

    // lcb_cntl_exists() : couchbase.h
    pub fn lcb_cntl_exists(ctl: c_int) -> c_int;

    // lcb_cntl_getu32() : couchbase.h
    pub fn lcb_cntl_getu32(
        instance: lcb_t,
        cmd: c_int
    ) -> lcb_U32;

    // lcb_cntl_setu32() : couchbase.h
    pub fn lcb_cntl_setu32(
        instance: lcb_t,
        cmd: c_int,
        arg: lcb_U32
    ) -> lcb_error_t;

    // lcb_cntl_string() : couchbase.h
    pub fn lcb_cntl_string(
        instance: lcb_t,
        key: *const c_char,
        value: *const c_char
    ) -> lcb_error_t;

    // lcb_connect() : couchbase.h
    pub fn lcb_connect(instance: lcb_t) -> lcb_error_t;

    // lcb_create() : couchbase.h
    pub fn lcb_create(
        instance: *mut lcb_t,
        options: *const lcb_create_st
    ) -> lcb_error_t;

    // lcb_create_io_ops() : couchbase.h
    pub fn lcb_create_io_ops(
        op: *mut lcb_io_opt_t,
        options: *const lcb_create_io_ops_st
    ) -> lcb_error_t;

    // lcb_destroy() : couchbase.h
    pub fn lcb_destroy(instance: lcb_t) -> ();

    // lcb_destroy_async() : couchbase.h
    pub fn lcb_destroy_async(
        instance: lcb_t,
        arg: *const c_void
    ) -> ();

    // lcb_destroy_io_ops() : couchbase.h
    pub fn lcb_destroy_io_ops(op: lcb_io_opt_t) -> lcb_error_t;

    // lcb_disable_timings() : couchbase.h
    pub fn lcb_disable_timings(instance: lcb_t) -> lcb_error_t;

    // lcb_dump() : couchbase.h
    pub fn lcb_dump(
        instance: lcb_t,
        fp: *mut FILE,
        flags: lcb_U32
    ) -> ();

    // lcb_durability_poll() : couchbase.h
    pub fn lcb_durability_poll(
        instance: lcb_t,
        cookie: *const c_void,
        options: *const lcb_durability_opts_t,
        ncmds: lcb_SIZE,
        cmds: *const *const lcb_durability_cmd_t
    ) -> lcb_error_t;

    // lcb_enable_timings() : couchbase.h
    pub fn lcb_enable_timings(instance: lcb_t) -> lcb_error_t;

    // lcb_errmap_default() : couchbase.h
    pub fn lcb_errmap_default(
        instance: lcb_t,
        code: lcb_U16
    ) -> lcb_error_t;

    // lcb_flush() : couchbase.h
    pub fn lcb_flush(
        instance: lcb_t,
        cookie: *const c_void,
        num: lcb_SIZE,
        commands: *const *const lcb_flush_cmd_t
    ) -> lcb_error_t;

    // lcb_get() : couchbase.h
    pub fn lcb_get(
        instance: lcb_t,
        command_cookie: *const c_void,
        num: lcb_SIZE,
        commands: *const *const lcb_get_cmd_t
    ) -> lcb_error_t;

    // lcb_get_bootstrap_status() : couchbase.h
    pub fn lcb_get_bootstrap_status(instance: lcb_t) -> lcb_error_t;

    // lcb_get_cookie() : couchbase.h
    pub fn lcb_get_cookie(instance: lcb_t) -> *const c_void;

    // lcb_get_node() : couchbase.h
    pub fn lcb_get_node(
        instance: lcb_t,
        _type: lcb_GETNODETYPE,
        index: c_uint
    ) -> *const c_char;

    // lcb_get_num_nodes() : couchbase.h
    pub fn lcb_get_num_nodes(instance: lcb_t) -> lcb_S32;

    // lcb_get_num_replicas() : couchbase.h
    pub fn lcb_get_num_replicas(instance: lcb_t) -> lcb_S32;

    // lcb_get_replica() : couchbase.h
    pub fn lcb_get_replica(
        instance: lcb_t,
        command_cookie: *const c_void,
        num: lcb_SIZE,
        commands: *const *const lcb_get_replica_cmd_t
    ) -> lcb_error_t;

    // lcb_get_server_list() : couchbase.h
    pub fn lcb_get_server_list(instance: lcb_t) -> *const *const c_char;

    // lcb_get_timings() : couchbase.h
    pub fn lcb_get_timings(
        instance: lcb_t,
        cookie: *const c_void,
        callback: lcb_timings_callback
    ) -> lcb_error_t;

    // lcb_get_version() : couchbase.h
    pub fn lcb_get_version(version: *mut lcb_U32) -> *const c_char;

    // lcb_is_waiting() : couchbase.h
    pub fn lcb_is_waiting(instance: lcb_t) -> c_int;

    // lcb_make_http_request() : couchbase.h
    pub fn lcb_make_http_request(
        instance: lcb_t,
        command_cookie: *const c_void,
        _type: lcb_http_type_t,
        cmd: *const lcb_http_cmd_t,
        request: *mut lcb_http_request_t
    ) -> lcb_error_t;

    // lcb_mem_alloc() : couchbase.h
    pub fn lcb_mem_alloc(size: lcb_SIZE) -> *mut c_void;

    // lcb_mem_free() : couchbase.h
    pub fn lcb_mem_free(ptr: *mut c_void) -> ();

    // lcb_observe() : couchbase.h
    pub fn lcb_observe(
        instance: lcb_t,
        command_cookie: *const c_void,
        num: lcb_SIZE,
        commands: *const *const lcb_observe_cmd_t
    ) -> lcb_error_t;

    // lcb_refresh_config() : couchbase.h
    pub fn lcb_refresh_config(instance: lcb_t) -> ();

    // lcb_remove() : couchbase.h
    pub fn lcb_remove(
        instance: lcb_t,
        command_cookie: *const c_void,
        num: lcb_SIZE,
        commands: *const *const lcb_remove_cmd_t
    ) -> lcb_error_t;

    // lcb_run_loop() : couchbase.h
    pub fn lcb_run_loop(instance: lcb_t) -> ();

    // lcb_stop_loop() : couchbase.h
    pub fn lcb_stop_loop(instance: lcb_t) -> ();

    // lcb_server_stats() : couchbase.h
    pub fn lcb_server_stats(
        instance: lcb_t,
        command_cookie: *const c_void,
        num: lcb_SIZE,
        commands: *const *const lcb_server_stats_cmd_t
    ) -> lcb_error_t;

    // lcb_server_versions() : couchbase.h
    pub fn lcb_server_versions(
        instance: lcb_t,
        command_cookie: *const c_void,
        num: lcb_SIZE,
        commands: *const *const lcb_server_version_cmd_t
    ) -> lcb_error_t;

    // lcb_set_arithmetic_callback() : couchbase.h
    pub fn lcb_set_arithmetic_callback(
        arg1: lcb_t,
        arg2: lcb_arithmetic_callback
    ) -> lcb_arithmetic_callback;

    // lcb_set_bootstrap_callback() : couchbase.h
    pub fn lcb_set_bootstrap_callback(
        instance: lcb_t,
        callback: lcb_bootstrap_callback
    ) -> lcb_bootstrap_callback;

    // lcb_set_cookie() : couchbase.h
    pub fn lcb_set_cookie(
        instance: lcb_t,
        cookie: *const c_void
    ) -> ();

    // lcb_set_destroy_callback() : couchbase.h
    pub fn lcb_set_destroy_callback(
        arg1: lcb_t,
        arg2: lcb_destroy_callback
    ) -> lcb_destroy_callback;

    // lcb_set_errmap_callback() : couchbase.h
    pub fn lcb_set_errmap_callback(
        arg1: lcb_t,
        arg2: lcb_errmap_callback
    ) -> lcb_errmap_callback;

    // lcb_set_get_callback() : couchbase.h
    pub fn lcb_set_get_callback(
        arg1: lcb_t,
        callback: lcb_get_callback
    ) -> lcb_get_callback;

    // lcb_set_http_complete_callback() : couchbase.h
    pub fn lcb_set_http_complete_callback(
        arg1: lcb_t,
        arg2: lcb_http_complete_callback
    ) -> lcb_http_complete_callback;

    // lcb_set_http_data_callback() : couchbase.h
    pub fn lcb_set_http_data_callback(
        arg1: lcb_t,
        arg2: lcb_http_data_callback
    ) -> lcb_http_data_callback;

    // lcb_set_store_callback() : couchbase.h
    pub fn lcb_set_store_callback(
        arg1: lcb_t,
        callback: lcb_store_callback
    ) -> lcb_store_callback;

    // lcb_set_touch_callback() : couchbase.h
    pub fn lcb_set_touch_callback(
        arg1: lcb_t,
        arg2: lcb_touch_callback
    ) -> lcb_touch_callback;

    // lcb_set_unlock_callback() : couchbase.h
    pub fn lcb_set_unlock_callback(
        arg1: lcb_t,
        arg2: lcb_unlock_callback
    ) -> lcb_unlock_callback;

    // lcb_set_verbosity() : couchbase.h
    pub fn lcb_set_verbosity(
        instance: lcb_t,
        command_cookie: *const c_void,
        num: lcb_SIZE,
        commands: *const *const lcb_verbosity_cmd_t
    ) -> lcb_error_t;

    // lcb_store() : couchbase.h
    pub fn lcb_store(
        instance: lcb_t,
        command_cookie: *const c_void,
        num: lcb_SIZE,
        commands: *const *const lcb_store_cmd_t
    ) -> lcb_error_t;

    // lcb_supports_feature() : couchbase.h
    pub fn lcb_supports_feature(n: c_int) -> c_int;

    // lcb_tick_nowait() : couchbase.h
    pub fn lcb_tick_nowait(instance: lcb_t) -> lcb_error_t;

    // lcb_touch() : couchbase.h
    pub fn lcb_touch(
        instance: lcb_t,
        cookie: *const c_void,
        num: lcb_SIZE,
        commands: *const *const lcb_touch_cmd_t
    ) -> lcb_error_t;

    // lcb_unlock() : couchbase.h
    pub fn lcb_unlock(
        instance: lcb_t,
        command_cookie: *const c_void,
        num: lcb_SIZE,
        commands: *const *const lcb_unlock_cmd_t
    ) -> lcb_error_t;

    // lcb_wait() : couchbase.h
    pub fn lcb_wait(instance: lcb_t) -> lcb_error_t;

    // lcb_wait3() : couchbase.h
    pub fn lcb_wait3(
        instance: lcb_t,
        flags: lcb_WAITFLAGS
    ) -> ();

    // lcb_nstime() : couchbase.h
    pub fn lcb_nstime() -> lcb_U64;

    // lcb_set_observe_callback() : couchbase.h
    pub fn lcb_set_observe_callback(
        arg1: lcb_t,
        arg2: lcb_observe_callback
    ) -> lcb_observe_callback;

    // lcb_set_remove_callback() : couchbase.h
    pub fn lcb_set_remove_callback(
        arg1: lcb_t,
        arg2: lcb_remove_callback
    ) -> lcb_remove_callback;

    // lcb_set_durability_callback() : couchbase.h
    pub fn lcb_set_durability_callback(
        arg1: lcb_t,
        arg2: lcb_durability_callback
    ) -> lcb_durability_callback;

    // lcb_set_obserlcb_set_stat_callbackve_callback() : couchbase.h
    pub fn lcb_set_stat_callback(
        arg1: lcb_t,
        arg2: lcb_stat_callback
    ) -> lcb_stat_callback;

    // lcb_set_version_callback() : couchbase.h
    pub fn lcb_set_version_callback(
        arg1: lcb_t,
        arg2: lcb_version_callback
    ) -> lcb_version_callback;

    // lcb_set_verbosity_callback() : couchbase.h
    pub fn lcb_set_verbosity_callback(
        arg1: lcb_t,
        arg2: lcb_verbosity_callback
    ) -> lcb_verbosity_callback;

    // lcb_set_flush_callback() : couchbase.h
    pub fn lcb_set_flush_callback(
        arg1: lcb_t,
        arg2: lcb_flush_callback
    ) -> lcb_flush_callback;
}