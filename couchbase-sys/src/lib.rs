extern crate libc;

pub use ffi::{
	lcb_create,
 //    lcb_connect,
 //    lcb_set_cookie,
 //    lcb_get_cookie,
 //    lcb_wait,
 //    // lcb_tick_nowait, //available in 2.5.0
 //    lcb_wait3,
 //    lcb_breakout,
 //    lcb_set_bootstrap_callback,
 //    lcb_get_bootstrap_status,
 //    lcb_refresh_config,
 //    lcb_destroy,
 //    lcb_set_destroy_callback,
 //    lcb_destroy_async,
 //    lcb_create_io_ops,
 //    lcb_destroy_io_ops,
 //    lcb_set_get_callback,
 //    lcb_get,
 //    lcb_get_replica,
 //    lcb_set_unlock_callback,
 //    lcb_unlock,
 //    lcb_set_store_callback,
 //    lcb_store,
 //    lcb_set_arithmetic_callback,
 //    lcb_arithmetic,
 //    lcb_set_observe_callback,
 //    lcb_observe,
 //    lcb_set_remove_callback,
 //    lcb_remove,
 //    lcb_set_touch_callback,
 //    lcb_touch,
 //    lcb_durability_poll,
 //    lcb_set_durability_callback,
 //    lcb_set_stat_callback,
 //    lcb_server_stats,
 //    lcb_server_versions,
 //    lcb_set_version_callback,
 //    lcb_set_verbosity,
 //    lcb_set_verbosity_callback,
 //    lcb_flush,
 //    lcb_set_flush_callback,
 //    lcb_set_http_complete_callback,
 //    lcb_set_http_data_callback,
 //    lcb_make_http_request,
 //    lcb_cancel_http_request,
 //    lcb_get_node,
 //    lcb_get_num_replicas,
 //    lcb_get_num_nodes,
 //    lcb_get_server_list,
 //    lcb_is_waiting,
 //    lcb_cntl,
 //    lcb_cntl_string,
 //    lcb_cntl_setu32,
 //    lcb_cntl_getu32,
 //    lcb_cntl_exists,
 //    lcb_enable_timings,
 //    lcb_disable_timings,
 //    lcb_get_timings,
 //    lcb_get_version,
 //    lcb_supports_feature,
 //    lcb_errmap_default,
 //    lcb_set_errmap_callback,
 //    lcb_mem_alloc,
 //    lcb_mem_free,
 //    lcb_run_loop,
 //    lcb_stop_loop,
 //    lcb_nstime,
 //    lcb_dump,
};

#[allow(non_camel_case_types)]
mod ffi {
    use libc::{
	    c_int,
	    c_uint,
	    c_char,
	    uint8_t,
	    uint16_t,
	    uint32_t,
	    uint64_t,
	    int16_t,
	    int32_t,
	    int64_t,
	    size_t,
	    ssize_t,
	    c_void,
	    time_t,
	    FILE,
	    c_short,
	};

	use std::option::Option;
	use std::clone::Clone;
	use std::default::Default;
	use std::mem::{
	    transmute,
	    zeroed,
	};

	// Basic type definitions
	pub type lcb_int32_t = int32_t;
	pub type lcb_int64_t = int64_t;
	pub type lcb_size_t = size_t;
	pub type lcb_ssize_t = ssize_t;
	pub type lcb_vbucket_t = uint16_t;
	pub type lcb_uint8_t = uint8_t;
	pub type lcb_uint16_t = uint16_t;
	pub type lcb_uint32_t = uint32_t;
	pub type lcb_uint64_t = uint64_t;
	pub type lcb_cas_t = uint64_t;
	pub type lcb_time_t = time_t;

	pub type lcb_S64 = lcb_int64_t;
	pub type lcb_U64 = lcb_uint64_t;
	pub type lcb_U32 = lcb_uint32_t;
	pub type lcb_S32 = lcb_int32_t;
	pub type lcb_U16 = lcb_uint16_t;
	pub type lcb_U8 = lcb_uint8_t;
	pub type lcb_SIZE = lcb_size_t;
	pub type lcb_SSIZE = lcb_ssize_t;
	pub type lcb_SECS = lcb_time_t;
	pub type lcb_CAS = lcb_cas_t;

    include!("ffi.rs");
    include!("iops_ffi.rs");
}
