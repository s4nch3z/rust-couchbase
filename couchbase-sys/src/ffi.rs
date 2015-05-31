// Enumerations
pub enum lcb_CALLBACKTYPE {
    LCB_CALLBACK_DEFAULT = 0,
    LCB_CALLBACK_GET,
    LCB_CALLBACK_STORE,
    LCB_CALLBACK_COUNTER,
    LCB_CALLBACK_TOUCH,
    LCB_CALLBACK_REMOVE,
    LCB_CALLBACK_UNLOCK,
    LCB_CALLBACK_STATS,
    LCB_CALLBACK_VERSIONS,
    LCB_CALLBACK_VERBOSITY,
    LCB_CALLBACK_FLUSH,
    LCB_CALLBACK_OBSERVE,
    LCB_CALLBACK_GETREPLICA,
    LCB_CALLBACK_ENDURE,
    LCB_CALLBACK_HTTP,
    LCB_CALLBACK_CBFLUSH,
    LCB_CALLBACK_OBSEQNO,
    LCB_CALLBACK__MAX,
}
pub enum lcb_RESPFLAGS {
    LCB_RESP_F_FINAL = 0x01,
    LCB_RESP_F_CLIENTGEN = 0x02,
    LCB_RESP_F_NMVGEN = 0x04
}
pub enum lcb_COMPRESSOPTS {
    LCB_COMPRESS_NONE = 0x00,
    LCB_COMPRESS_IN = 1 << 0,
    LCB_COMPRESS_OUT = 1 << 1,
    LCB_COMPRESS_INOUT = (lcb_COMPRESSOPTS::LCB_COMPRESS_IN|lcb_COMPRESSOPTS::LCB_COMPRESS_OUT),
    LCB_COMPRESS_FORCE = 1 << 2,
}
pub enum lcb_DUMPFLAGS {
    LCB_DUMP_VBCONFIG =  0x01,
    LCB_DUMP_PKTINFO = 0x02,
    LCB_DUMP_BUFINFO = 0x04,
    LCB_DUMP_ALL = 0xff,
}
pub enum lcb_storage_t {
    LCB_ADD = 0x01,
    LCB_REPLACE = 0x02,
    LCB_SET = 0x03,
    LCB_APPEND = 0x04,
    LCB_PREPEND = 0x05,
}
pub enum lcb_timeunit_t {
    LCB_TIMEUNIT_NSEC = 0,
    LCB_TIMEUNIT_USEC = 1,
    LCB_TIMEUNIT_MSEC = 2,
    LCB_TIMEUNIT_SEC = 3,
}
pub enum lcb_type_t {
    LCB_TYPE_BUCKET = 0x00,
    LCB_TYPE_CLUSTER = 0x01,
}
pub enum lcb_VALUEFLAGS {
    LCB_VALUE_RAW = 0x00,
    LCB_VALUE_F_JSON = 0x01,
    LCB_VALUE_F_SNAPPYCOMP = 0x02,
}
pub enum lcb_observe_options_t {
    LCB_OBSERVE_MASTER_ONLY = 0x01,
}
pub enum lcb_observe_t {
    LCB_OBSERVE_FOUND = 0x00,
    LCB_OBSERVE_PERSISTED = 0x01,
    LCB_OBSERVE_NOT_FOUND = 0x80,
    LCB_OBSERVE_LOGICALLY_DELETED = 0x81,
    LCB_OBSERVE_MAX = 0x82,
}
pub enum lcb_replica_t {
    LCB_REPLICA_FIRST = 0x00,
    LCB_REPLICA_ALL = 0x01,
    LCB_REPLICA_SELECT = 0x02,
}
pub enum lcb_GETNODETYPE {
    LCB_NODE_HTCONFIG = 0x01,
    LCB_NODE_DATA = 0x02,
    LCB_NODE_VIEWS = 0x04,
    LCB_NODE_CONNECTED = 0x08,
    LCB_NODE_NEVERNULL = 0x10,
    LCB_NODE_HTCONFIG_CONNECTED = 0x09,
    LCB_NODE_HTCONFIG_ANY = 0x11,
}
pub enum lcb_http_method_t {
    LCB_HTTP_METHOD_GET = 0,
    LCB_HTTP_METHOD_POST = 1,
    LCB_HTTP_METHOD_PUT = 2,
    LCB_HTTP_METHOD_DELETE = 3,
    LCB_HTTP_METHOD_MAX = 4,
}
pub enum lcb_http_type_t {
    LCB_HTTP_TYPE_VIEW = 0,
    LCB_HTTP_TYPE_MANAGEMENT = 1,
    LCB_HTTP_TYPE_RAW = 2,
    LCB_HTTP_TYPE_N1QL = 3,
    LCB_HTTP_TYPE_MAX,
}
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
pub enum lcb_verbosity_level_t {
    LCB_VERBOSITY_DETAIL = 0x00,
    LCB_VERBOSITY_DEBUG = 0x01,
    LCB_VERBOSITY_INFO = 0x02,
    LCB_VERBOSITY_WARNING = 0x03,
}
pub enum lcb_WAITFLAGS {
    LCB_WAIT_DEFAULT = 0x00,
    LCB_WAIT_NOCHECK = 0x01,
}
pub enum lcb_errflags_t {
    LCB_ERRTYPE_INPUT = 1 << 0,
    LCB_ERRTYPE_NETWORK = 1 << 1,
    LCB_ERRTYPE_FATAL = 1 << 2,
    LCB_ERRTYPE_TRANSIENT = 1 << 3,
    LCB_ERRTYPE_DATAOP = 1 << 4,
    LCB_ERRTYPE_INTERNAL = 1 << 5,
    LCB_ERRTYPE_PLUGIN = 1 << 6,
    LCB_ERRTYPE_SRVLOAD = 1 << 7,
    LCB_ERRTYPE_SRVGEN = 1 << 8,
}
pub enum lcb_error_t {
    LCB_SUCCESS = 0x00,
    LCB_AUTH_CONTINUE = 0x01,
    LCB_AUTH_ERROR = 0x02,
    LCB_DELTA_BADVAL = 0x03,
    LCB_E2BIG = 0x04,
    LCB_EBUSY = 0x05,
    LCB_EINTERNAL = 0x06,
    LCB_EINVAL = 0x07,
    LCB_ENOMEM = 0x08,
    LCB_ERANGE = 0x09,
    LCB_ERROR = 0x0A,
    LCB_ETMPFAIL = 0x0B,
    LCB_KEY_EEXISTS = 0x0C,
    LCB_KEY_ENOENT = 0x0D,
    LCB_DLOPEN_FAILED = 0x0E,
    LCB_DLSYM_FAILED = 0x0F,
    LCB_NETWORK_ERROR = 0x10,
    LCB_NOT_MY_VBUCKET = 0x11,
    LCB_NOT_STORED = 0x12,
    LCB_NOT_SUPPORTED = 0x13,
    LCB_UNKNOWN_COMMAND = 0x14,
    LCB_UNKNOWN_HOST = 0x15,
    LCB_PROTOCOL_ERROR = 0x16,
    LCB_ETIMEDOUT = 0x17,
    LCB_CONNECT_ERROR = 0x18,
    LCB_BUCKET_ENOENT = 0x19,
    LCB_CLIENT_ENOMEM = 0x1A,
    LCB_CLIENT_ETMPFAIL = 0x1B,
    LCB_EBADHANDLE = 0x1C,
    LCB_SERVER_BUG = 0x1D,
    LCB_INVALID_HOST_FORMAT = 0x1F,
    LCB_INVALID_CHAR = 0x20,
    LCB_DURABILITY_ETOOMANY = 0x21,
    LCB_DUPLICATE_COMMANDS = 0x22,
    LCB_NO_MATCHING_SERVER = 0x23,
    LCB_BAD_ENVIRONMENT = 0x24,
    LCB_BUSY = 0x25,
    LCB_INVALID_USERNAME = 0x26,
    LCB_CONFIG_CACHE_INVALID = 0x27,
    LCB_SASLMECH_UNAVAILABLE = 0x28,
    LCB_TOO_MANY_REDIRECTS = 0x29,
    LCB_MAP_CHANGED = 0x2A,
    LCB_INCOMPLETE_PACKET = 0x2B,
    LCB_ECONNREFUSED = 0x2C,
    LCB_ESOCKSHUTDOWN = 0x2D,
    LCB_ECONNRESET = 0x2E,
    LCB_ECANTGETPORT = 0x2F,
    LCB_EFDLIMITREACHED = 0x30,
    LCB_ENETUNREACH = 0x31,
    LCB_ECTL_UNKNOWN = 0x32,
    LCB_ECTL_UNSUPPMODE = 0x33,
    LCB_ECTL_BADARG = 0x34,
    LCB_EMPTY_KEY = 0x35,
    LCB_SSL_ERROR = 0x36,
    LCB_SSL_CANTVERIFY = 0x37,
    LCB_SCHEDFAIL_INTERNAL = 0x38,
    LCB_CLIENT_FEATURE_UNAVAILABLE = 0x39,
    LCB_OPTIONS_CONFLICT = 0x3A,
    LCB_HTTP_ERROR = 0x3B,
    LCB_DURABILITY_NO_SYNCTOKEN = 0x3C,
    LCB_UNKNOWN_MEMCACHED_ERROR = 0x3D,
    LCB_MUTATION_LOST = 0x3E,
    LCB_MAX_ERROR = 0x1000,
}
pub enum lcb_KVBUFTYPE {
    LCB_KV_COPY = 0,
    LCB_KV_CONTIG,
    LCB_KV_IOV,
    LCB_KV_VBID,
}
pub enum lcb_HTCONFIG_URLTYPE {
    LCB_HTCONFIG_URLTYPE_25PLUS = 0x01,
    LCB_HTCONFIG_URLTYPE_COMPAT = 0x02,
    LCB_HTCONFIG_URLTYPE_TRYALL = 0x03,
}
pub enum lcb_log_severity_t {
    LCB_LOG_TRACE = 0,
    LCB_LOG_DEBUG,
    LCB_LOG_INFO,
    LCB_LOG_WARN,
    LCB_LOG_ERROR,
    LCB_LOG_FATAL,
    LCB_LOG_MAX,
}
pub enum lcb_RETRYCMDOPTS {
    LCB_RETRY_CMDS_NONE = 0,
    LCB_RETRY_CMDS_GET = 0x01,
    LCB_RETRY_CMDS_SAFE = 0x03,
    LCB_RETRY_CMDS_ALL = 0x07,
}
pub enum lcb_RETRYMODEOPTS {
    LCB_RETRY_ON_TOPOCHANGE = 0,
    LCB_RETRY_ON_SOCKERR,
    LCB_RETRY_ON_VBMAPERR,
    LCB_RETRY_ON_MISSINGNODE,
    LCB_RETRY_ON_MAX,
}
pub enum lcb_SSLOPTS {
    LCB_SSL_ENABLED = 1 << 0,
    LCB_SSL_NOVERIFY = 1 << 1,
}
pub enum lcb_vbucket_state_t {
    LCB_VBUCKET_STATE_ACTIVE = 1,
    LCB_VBUCKET_STATE_REPLICA = 2,
    LCB_VBUCKET_STATE_PENDING = 3,
    LCB_VBUCKET_STATE_DEAD = 4,
}
pub enum lcbvb_CHANGETYPE {
    LCBVB_NO_CHANGES = 0,
    LCBVB_SERVERS_MODIFIED = 1 << 0,
    LCBVB_MAP_MODIFIED = 1 << 1,
}
pub enum lcbvb_DISTMODE {
    LCBVB_DIST_VBUCKET = 0,
    LCBVB_DIST_KETAMA = 1,
}
pub enum lcbvb_SVCMODE {
    LCBVB_SVCMODE_PLAIN = 0,
    LCBVB_SVCMODE_SSL,
    LCBVB_SVCMODE__MAX,
}
pub enum lcbvb_SVCTYPE {
    LCBVB_SVCTYPE_DATA = 0,
    LCBVB_SVCTYPE_VIEWS,
    LCBVB_SVCTYPE_MGMT,
    LCBVB_SVCTYPE_IXQUERY,
    LCBVB_SVCTYPE_IXADMIN,
    LCBVB_SVCTYPE_N1QL,
    LCBVB_SVCTYPE__MAX,
}
pub enum lcb_config_transport_t {
    LCB_CONFIG_TRANSPORT_LIST_END = 0,
    LCB_CONFIG_TRANSPORT_HTTP = 1,
    LCB_CONFIG_TRANSPORT_CCCP,
    LCB_CONFIG_TRANSPORT_MAX,
}


// Type definitions
// pub type lcb_destroy_callback = Option<extern "C" fn(
//     cookie: *const c_void
// ) -> ()>;

// // lcb_durability_callback : couchbase.h
// pub type lcb_durability_callback = Option<extern "C" fn(
//     instance: lcb_t,
//     cookie: *const c_void,
//     err: lcb_error_t,
//     res: *const lcb_durability_resp_t
// ) -> ()>;

// // lcb_errmap_callback : couchbase.h
// pub type lcb_errmap_callback = Option<extern "C" fn(
//     instance: lcb_t,
//     bincode: lcb_U16
// ) -> lcb_error_t>;

// // lcb_flush_callback : couchbase.h
// pub type lcb_flush_callback = Option<extern "C" fn(
//     instance: lcb_t,
//     cookie: *const c_void,
//     error: lcb_error_t,
//     resp: *const lcb_flush_resp_t
// ) -> ()>;

// // lcb_get_callback : couchbase.h
// pub type lcb_get_callback = Option<extern "C" fn(
//     instance: lcb_t,
//     cookie: *const c_void,
//     error: lcb_error_t,
//     resp: *const lcb_get_resp_t
// ) -> ()>;

// // lcb_http_res_callback : couchbase.h
// pub type lcb_http_res_callback = Option<extern "C" fn(
//     request: lcb_http_request_t,
//     instance: lcb_t,
//     cookie: *const c_void,
//     error: lcb_error_t,
//     resp: *const lcb_http_resp_t
// ) -> ()>;

// // lcb_arithmetic_callback : couchbase.h
// pub type lcb_arithmetic_callback = Option<extern "C" fn(
//     instance: lcb_t,
//     cookie: *const c_void,
//     error: lcb_error_t,
//     resp: *const lcb_arithmetic_resp_t
// ) -> ()>;
// // lcb_bootstrap_callback : couchbase.h
// pub type lcb_bootstrap_callback = Option<extern "C" fn(
//     instance: lcb_t,
//     err: lcb_error_t
// ) -> ()>;
// // lcb_observe_callback : couchbase.h
// pub type lcb_observe_callback = Option<extern "C" fn(
//     instance: lcb_t,
//     cookie: *const c_void,
//     error: lcb_error_t,
//     resp: *const lcb_observe_resp_t
// ) -> ()>;
// // lcb_remove_callback : couchbase.h
// pub type lcb_remove_callback = Option<extern "C" fn(
//     instance: lcb_t,
//     cookie: *const c_void,
//     error: lcb_error_t,
//     resp: *const lcb_remove_resp_t
// ) -> ()>;
// // lcb_stat_callback : couchbase.h
// pub type lcb_stat_callback = Option<extern "C" fn(
//     instance: lcb_t,
//     cookie: *const c_void,
//     error: lcb_error_t,
//     resp: *const lcb_server_stat_resp_t
// ) -> ()>;
// // lcb_store_callback : couchbase.h
// pub type lcb_store_callback = Option<extern "C" fn(
//     instance: lcb_t,
//     cookie: *const c_void,
//     operation: lcb_storage_t,
//     error: lcb_error_t,
//     resp: *const lcb_store_resp_t
// ) -> ()>;
// // lcb_timings_callback : couchbase.h
// pub type lcb_timings_callback = Option<extern "C" fn(
//     instance: lcb_t,
//     cookie: *const c_void,
//     timeunit: lcb_timeunit_t,
//     min: lcb_U32, max: lcb_U32,
//     total: lcb_U32, maxtotal: lcb_U32
// ) -> ()>;
// // lcb_touch_callback : couchbase.h
// pub type lcb_touch_callback = Option<extern "C" fn(
//     instance: lcb_t,
//     cookie: *const c_void,
//     error: lcb_error_t,
//     resp: *const lcb_touch_resp_t
// ) -> ()>;
// // lcb_unlock_callback : couchbase.h
// pub type lcb_unlock_callback = Option<extern "C" fn(
//     instance: lcb_t,
//     cookie: *const c_void,
//     error: lcb_error_t,
//     resp: *const lcb_unlock_resp_t
// ) -> ()>;
// // lcb_verbosity_callback : couchbase.h
// pub type lcb_verbosity_callback = Option<extern "C" fn(
//     instance: lcb_t,
//     cookie: *const c_void,
//     error: lcb_error_t,
//     resp: *const lcb_verbosity_resp_t
// ) -> ()>;
// // lcb_version_callback : couchbase.h
// pub type lcb_version_callback = Option<extern "C" fn(
//     instance: lcb_t,
//     cookie: *const c_void,
//     error: lcb_error_t,
//     resp:
//     *const lcb_server_version_resp_t
// ) -> ()>;


// lcb_CMDREMOVE : api3.h
// lcb_CMDSTATS : api3.h
// lcb_CMDTOUCH : api3.h
// lcb_CMDUNLOCK : api3.h
// lcb_RESPCALLBACK : api3.h
// lcb_RESPTOUCH : api3.h
// lcb_RESPUNLOCK : api3.h


// lcb_logging_callback : cntl.h




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



// Variables
#[link(name = "couchbase")]
extern "C" {
    pub static lcb_version_g: lcb_U32;
}

// Functions
#[link(name = "couchbase")]
extern "C" {

// lcb_arithmetic() : couchbase.h
// lcb_breakout() : couchbase.h
// lcb_cancel_http_request() : couchbase.h
// lcb_cntl() : couchbase.h
// lcb_cntl_exists() : couchbase.h
// lcb_cntl_getu32() : couchbase.h
// lcb_cntl_setu32() : couchbase.h
// lcb_cntl_string() : couchbase.h
// lcb_connect() : couchbase.h

// lcb_create() : couchbase.h
    pub fn lcb_create(
        instance: *mut lcb_t,
        options: *const lcb_create_st
    ) -> lcb_error_t;

// lcb_create_io_ops() : couchbase.h
// lcb_destroy() : couchbase.h
// lcb_destroy_async() : couchbase.h
// lcb_destroy_io_ops() : couchbase.h
// lcb_disable_timings() : couchbase.h
// lcb_dump() : couchbase.h
// lcb_durability_poll() : couchbase.h
// lcb_enable_timings() : couchbase.h
// lcb_errmap_default() : couchbase.h
// lcb_flush() : couchbase.h
// lcb_get() : couchbase.h
// lcb_get_bootstrap_status() : couchbase.h
// lcb_get_cookie() : couchbase.h
// lcb_get_node() : couchbase.h
// lcb_get_num_nodes() : couchbase.h
// lcb_get_num_replicas() : couchbase.h
// lcb_get_replica() : couchbase.h
// lcb_get_server_list() : couchbase.h
// lcb_get_timings() : couchbase.h
// lcb_get_version() : couchbase.h
// lcb_is_waiting() : couchbase.h
// lcb_make_http_request() : couchbase.h
// lcb_mem_alloc() : couchbase.h
// lcb_mem_free() : couchbase.h
// lcb_observe() : couchbase.h
// lcb_refresh_config() : couchbase.h
// lcb_remove() : couchbase.h
// lcb_run_loop() : couchbase.h
// lcb_server_stats() : couchbase.h
// lcb_server_versions() : couchbase.h
// lcb_set_arithmetic_callback() : couchbase.h
// lcb_set_bootstrap_callback() : couchbase.h
// lcb_set_cookie() : couchbase.h
// lcb_set_destroy_callback() : couchbase.h
// lcb_set_errmap_callback() : couchbase.h
// lcb_set_get_callback() : couchbase.h
// lcb_set_http_complete_callback() : couchbase.h
// lcb_set_http_data_callback() : couchbase.h
// lcb_set_store_callback() : couchbase.h
// lcb_set_touch_callback() : couchbase.h
// lcb_set_unlock_callback() : couchbase.h
// lcb_set_verbosity() : couchbase.h
// lcb_store() : couchbase.h
// lcb_supports_feature() : couchbase.h
// lcb_tick_nowait() : couchbase.h
// lcb_touch() : couchbase.h
// lcb_unlock() : couchbase.h
// lcb_wait() : couchbase.h
// lcb_tick_nowait() : couchbase.h // for 2.5.0
// lcb_wait3() : couchbase.h


// lcb_timer_create() : deprecated.h
// lcb_timer_destroy() : deprecated.h


// lcb_cbflush3() : api3.h
// lcb_counter3() : api3.h
// lcb_endure3_ctxnew() : api3.h
// lcb_flush3() : api3.h
// lcb_get3() : api3.h
// lcb_get_callback3() : api3.h
// lcb_get_errtype() : error.h
// lcb_get_synctoken() : api3.h
// lcb_install_callback3() : api3.h
// lcb_iops_wire_bsd_impl2() : iops.h
// lcb_observe3_ctxnew() : api3.h
// lcb_remove3() : api3.h
// lcb_resp_get_synctoken() : api3.h
// lcb_rget3() : api3.h
// lcb_sched_enter() : api3.h
// lcb_sched_fail() : api3.h
// lcb_sched_flush() : api3.h
// lcb_sched_leave() : api3.h
// lcb_server_verbosity3() : api3.h
// lcb_server_versions3() : api3.h
// lcb_stats3() : api3.h
// lcb_store3() : api3.h
// lcb_touch3() : api3.h
// lcb_unlock3() : api3.h


// lcb_create_libev_io_opts() : libev_io_opts.h
// lcb_create_libevent_io_opts() : libevent_io_opts.h
// lcb_create_libuv_io_opts() : libuv_io_opts.h


// LCB_DEPR_API2() : deprecated.h
// lcb_verify_size() : deprecated.h


// lcb_strerror() : error.h


// lcbvb_compare() : vbucket.h
// lcbvb_create() : vbucket.h
// lcbvb_destroy() : vbucket.h
// lcbvb_free_diff() : vbucket.h
// lcbvb_genconfig() : vbucket.h
// lcbvb_genconfig_ex() : vbucket.h
// lcbvb_get_capibase() : vbucket.h
// lcbvb_get_changetype() : vbucket.h
// lcbvb_get_distmode() : vbucket.h
// lcbvb_get_error() : vbucket.h
// lcbvb_get_hostport() : vbucket.h
// lcbvb_get_nreplicas() : vbucket.h
// lcbvb_get_nservers() : vbucket.h
// lcbvb_get_port() : vbucket.h
// lcbvb_get_randhost() : vbucket.h
// lcbvb_get_resturl() : vbucket.h
// lcbvb_get_revision() : vbucket.h
// lcbvb_has_vbucket() : vbucket.h
// lcbvb_k2vb() : vbucket.h
// lcbvb_load_json() : vbucket.h
// lcbvb_make_ketama() : vbucket.h
// lcbvb_map_key() : vbucket.h
// lcbvb_nmv_remap() : vbucket.h
// lcbvb_parse_json() : vbucket.h
// lcbvb_replace_host() : vbucket.h
// lcbvb_save_json() : vbucket.h
// lcbvb_vbmaster() : vbucket.h
// lcbvb_vbreplica() : vbucket.h




    // pub fn lcb_connect(instance: lcb_t) -> lcb_error_t;
    // pub fn lcb_set_cookie(instance: lcb_t, cookie: *const c_void)
    //  -> ();
    // pub fn lcb_get_cookie(instance: lcb_t) -> *const c_void;
    // pub fn lcb_wait(instance: lcb_t) -> lcb_error_t;
    // // lcb_tick_nowait, //available in 2.5.0
    // pub fn lcb_wait3(instance: lcb_t, flags: lcb_WAITFLAGS) -> ();
    // pub fn lcb_breakout(instance: lcb_t) -> ();
    // pub fn lcb_set_bootstrap_callback(instance: lcb_t,
    //                                   callback: lcb_bootstrap_callback)
    //  -> lcb_bootstrap_callback;
    // pub fn lcb_get_bootstrap_status(instance: lcb_t) -> lcb_error_t;
    // pub fn lcb_refresh_config(instance: lcb_t) -> ();
    // pub fn lcb_destroy(instance: lcb_t) -> ();
    // pub fn lcb_set_destroy_callback(arg1: lcb_t, arg2: lcb_destroy_callback)
    //  -> lcb_destroy_callback;
    // pub fn lcb_destroy_async(instance: lcb_t, arg: *const c_void)
    //  -> ();
    // pub fn lcb_create_io_ops(op: *mut lcb_io_opt_t,
    //                          options: *const lcb_create_io_ops_st)
    //  -> lcb_error_t;
    // pub fn lcb_destroy_io_ops(op: lcb_io_opt_t) -> lcb_error_t;
    // pub fn lcb_set_get_callback(arg1: lcb_t, callback: lcb_get_callback)
    //  -> lcb_get_callback;
    // pub fn lcb_get(instance: lcb_t, command_cookie: *const c_void,
    //                num: lcb_SIZE, commands: *const *const lcb_get_cmd_t)
    //  -> lcb_error_t;
    // pub fn lcb_get_replica(instance: lcb_t,
    //                        command_cookie: *const c_void,
    //                        num: lcb_SIZE,
    //                        commands: *const *const lcb_get_replica_cmd_t)
    //  -> lcb_error_t;
    // pub fn lcb_set_unlock_callback(arg1: lcb_t, arg2: lcb_unlock_callback)
    //  -> lcb_unlock_callback;
    // pub fn lcb_unlock(instance: lcb_t, command_cookie: *const c_void,
    //                   num: lcb_SIZE, commands: *const *const lcb_unlock_cmd_t)
    //  -> lcb_error_t;
    // pub fn lcb_set_store_callback(arg1: lcb_t, callback: lcb_store_callback)
    //  -> lcb_store_callback;
    // pub fn lcb_store(instance: lcb_t, command_cookie: *const c_void,
    //                  num: lcb_SIZE, commands: *const *const lcb_store_cmd_t)
    //  -> lcb_error_t;
    // pub fn lcb_set_arithmetic_callback(arg1: lcb_t,
    //                                    arg2: lcb_arithmetic_callback)
    //  -> lcb_arithmetic_callback;
    // pub fn lcb_arithmetic(instance: lcb_t,
    //                       command_cookie: *const c_void,
    //                       num: lcb_SIZE,
    //                       commands: *const *const lcb_arithmetic_cmd_t)
    //  -> lcb_error_t;
    // pub fn lcb_set_observe_callback(arg1: lcb_t, arg2: lcb_observe_callback)
    //  -> lcb_observe_callback;
    // pub fn lcb_observe(instance: lcb_t, command_cookie: *const c_void,
    //                    num: lcb_SIZE,
    //                    commands: *const *const lcb_observe_cmd_t)
    //  -> lcb_error_t;
    // pub fn lcb_set_remove_callback(arg1: lcb_t, arg2: lcb_remove_callback)
    //  -> lcb_remove_callback;
    // pub fn lcb_remove(instance: lcb_t, command_cookie: *const c_void,
    //                   num: lcb_SIZE, commands: *const *const lcb_remove_cmd_t)
    //  -> lcb_error_t;
    // pub fn lcb_set_touch_callback(arg1: lcb_t, arg2: lcb_touch_callback)
    //  -> lcb_touch_callback;
    // pub fn lcb_touch(instance: lcb_t, cookie: *const c_void,
    //                  num: lcb_SIZE, commands: *const *const lcb_touch_cmd_t)
    //  -> lcb_error_t;
    // pub fn lcb_durability_poll(instance: lcb_t, cookie: *const c_void,
    //                            options: *const lcb_durability_opts_t,
    //                            ncmds: lcb_SIZE,
    //                            cmds: *const *const lcb_durability_cmd_t)
    //  -> lcb_error_t;
    // pub fn lcb_set_durability_callback(arg1: lcb_t,
    //                                    arg2: lcb_durability_callback)
    //  -> lcb_durability_callback;
    // pub fn lcb_set_stat_callback(arg1: lcb_t, arg2: lcb_stat_callback)
    //  -> lcb_stat_callback;
    // pub fn lcb_server_stats(instance: lcb_t,
    //                         command_cookie: *const c_void,
    //                         num: lcb_SIZE,
    //                         commands: *const *const lcb_server_stats_cmd_t)
    //  -> lcb_error_t;
    // pub fn lcb_server_versions(instance: lcb_t,
    //                            command_cookie: *const c_void,
    //                            num: lcb_SIZE,
    //                            commands:
    //                                *const *const lcb_server_version_cmd_t)
    //  -> lcb_error_t;
    // pub fn lcb_set_version_callback(arg1: lcb_t, arg2: lcb_version_callback)
    //  -> lcb_version_callback;
    // pub fn lcb_set_verbosity(instance: lcb_t,
    //                          command_cookie: *const c_void,
    //                          num: lcb_SIZE,
    //                          commands: *const *const lcb_verbosity_cmd_t)
    //  -> lcb_error_t;
    // pub fn lcb_set_verbosity_callback(arg1: lcb_t,
    //                                   arg2: lcb_verbosity_callback)
    //  -> lcb_verbosity_callback;
    // pub fn lcb_flush(instance: lcb_t, cookie: *const c_void,
    //                  num: lcb_SIZE, commands: *const *const lcb_flush_cmd_t)
    //  -> lcb_error_t;
    // pub fn lcb_set_flush_callback(arg1: lcb_t, arg2: lcb_flush_callback)
    //  -> lcb_flush_callback;
    // pub fn lcb_set_http_complete_callback(arg1: lcb_t,
    //                                       arg2: lcb_http_complete_callback)
    //  -> lcb_http_complete_callback;
    // pub fn lcb_set_http_data_callback(arg1: lcb_t,
    //                                   arg2: lcb_http_data_callback)
    //  -> lcb_http_data_callback;
    // pub fn lcb_make_http_request(instance: lcb_t,
    //                              command_cookie: *const c_void,
    //                              _type: lcb_http_type_t,
    //                              cmd: *const lcb_http_cmd_t,
    //                              request: *mut lcb_http_request_t)
    //  -> lcb_error_t;
    // pub fn lcb_cancel_http_request(instance: lcb_t,
    //                                request: lcb_http_request_t) -> ();
    // pub fn lcb_get_node(instance: lcb_t, _type: lcb_GETNODETYPE,
    //                     index: c_uint) -> *const c_char;
    // pub fn lcb_get_num_replicas(instance: lcb_t) -> lcb_S32;
    // pub fn lcb_get_num_nodes(instance: lcb_t) -> lcb_S32;
    // pub fn lcb_get_server_list(instance: lcb_t)
    //  -> *const *const c_char;
    // pub fn lcb_is_waiting(instance: lcb_t) -> c_int;
    // pub fn lcb_cntl(instance: lcb_t, mode: c_int, cmd: c_int,
    //                 arg: *mut c_void) -> lcb_error_t;
    // pub fn lcb_cntl_string(instance: lcb_t, key: *const c_char,
    //                        value: *const c_char) -> lcb_error_t;
    // pub fn lcb_cntl_setu32(instance: lcb_t, cmd: c_int, arg: lcb_U32)
    //  -> lcb_error_t;
    // pub fn lcb_cntl_getu32(instance: lcb_t, cmd: c_int) -> lcb_U32;
    // pub fn lcb_cntl_exists(ctl: c_int) -> c_int;
    // pub fn lcb_enable_timings(instance: lcb_t) -> lcb_error_t;
    // pub fn lcb_disable_timings(instance: lcb_t) -> lcb_error_t;
    // pub fn lcb_get_timings(instance: lcb_t, cookie: *const c_void,
    //                        callback: lcb_timings_callback) -> lcb_error_t;
    // pub fn lcb_get_version(version: *mut lcb_U32) -> *const c_char;
    // pub fn lcb_supports_feature(n: c_int) -> c_int;
    // pub fn lcb_errmap_default(instance: lcb_t, code: lcb_U16) -> lcb_error_t;
    // pub fn lcb_set_errmap_callback(arg1: lcb_t, arg2: lcb_errmap_callback)
    //  -> lcb_errmap_callback;
    // pub fn lcb_mem_alloc(size: lcb_SIZE) -> *mut c_void;
    // pub fn lcb_mem_free(ptr: *mut c_void) -> ();
    // pub fn lcb_run_loop(instance: lcb_t) -> ();
    // pub fn lcb_stop_loop(instance: lcb_t) -> ();
    // pub fn lcb_nstime() -> lcb_U64;
    // pub fn lcb_dump(instance: lcb_t, fp: *mut types::common::c95::FILE, flags: lcb_U32) -> ();
}