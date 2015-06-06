// Enumerations

// Extra enumerations
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
// #[derive(BitAnd, BitOr)]
pub enum lcb_COMPRESSOPTS {
    LCB_COMPRESS_NONE = 0x00,
    LCB_COMPRESS_IN = 1 << 0,
    LCB_COMPRESS_OUT = 1 << 1,
    LCB_COMPRESS_INOUT = 5,//(lcb_COMPRESSOPTS::LCB_COMPRESS_IN|lcb_COMPRESSOPTS::LCB_COMPRESS_OUT),
    LCB_COMPRESS_FORCE = 1 << 2,
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

#[derive(Copy)]
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
impl Clone for lcb_error_t {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_error_t {
    fn default() -> Self { unsafe { zeroed() } }
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


// lcb_CMDREMOVE : api3.h
// lcb_CMDSTATS : api3.h
// lcb_CMDTOUCH : api3.h
// lcb_CMDUNLOCK : api3.h
// lcb_RESPCALLBACK : api3.h
// lcb_RESPTOUCH : api3.h
// lcb_RESPUNLOCK : api3.h


// lcb_logging_callback : cntl.h


// // Variables
// #[link(name = "couchbase")]
// extern "C" {
//     pub static lcb_version_g: lcb_U32;
// }

// Functions
// #[link(name = "couchbase")]
extern "C" {

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

}