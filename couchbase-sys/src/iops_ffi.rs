/***************
* Enumerations *
****************/
// lcb_iomodel_t : iops.h
pub enum lcb_iomodel_t {
    LCB_IOMODEL_EVENT,
    LCB_IOMODEL_COMPLETION,
}

/*******************
* Type definitions *
********************/
// lcb_io_connect_cb : iops.h
pub type lcb_io_connect_cb = Option<extern "C" fn(
    socket: *mut lcb_sockdata_t,
    status: c_int
) -> ()>;

// lcb_io_create_fn : iops.h
pub type lcb_io_create_fn = Option<extern "C" fn(
    version: c_int,
    io: *mut lcb_io_opt_t,
    cookie: *mut c_void
) -> lcb_error_t>;

// lcb_io_procs_fn : iops.h
pub type lcb_io_procs_fn = Option<extern "C" fn(
    loop_procs: *mut lcb_loop_procs,
    timer_procs: *mut lcb_timer_procs,
    bsd_procs: *mut lcb_bsd_procs,
    ev_procs: * mut lcb_ev_procs,
    completion_procs: *mut lcb_completion_procs,
    iomodel: *mut lcb_iomodel_t
) -> ()>;


// lcb_io_start_fn : iops.h
pub type lcb_io_start_fn = Option<extern "C" fn(
    iops: lcb_io_opt_t
) -> ()>;

// lcb_io_stop_fn : iops.h
pub type lcb_io_stop_fn = Option<extern "C" fn(
    iops: lcb_io_opt_t
) -> ()>;

// lcb_io_tick_fn : iops.h
pub type lcb_io_tick_fn = Option<extern "C" fn(
    iops: lcb_io_opt_t
) -> ()>;

// lcb_io_timer_cancel_fn : iops.h
pub type lcb_io_timer_cancel_fn = Option<extern "C" fn(
    iops: lcb_io_opt_t,
    timer: *mut c_void
) -> ()>;

// lcb_io_timer_create_fn : iops.h
pub type lcb_io_timer_create_fn = Option<extern "C" fn(
    iops: lcb_io_opt_t
) -> *mut c_void>;

// lcb_io_timer_destroy_fn : iops.h
pub type lcb_io_timer_destroy_fn = Option<extern "C" fn(
    iops: lcb_io_opt_t,
    timer: *mut c_void
) -> ()>;

// lcb_io_timer_schedule_fn : iops.h
pub type lcb_io_timer_schedule_fn = Option<extern "C" fn(
    iops: lcb_io_opt_t,
    timer: *mut c_void,
    usecs: lcb_U32,
    uarg: *mut c_void,
    callback: lcb_ioE_callback
) -> c_int>;

// lcb_ioC_chkclosed_fn : iops.h
pub type lcb_ioC_chkclosed_fn = Option<extern "C" fn(
    iops: lcb_io_opt_t,
    sd: *mut lcb_sockdata_t,
    flags: c_int
) -> c_int>;

// lcb_ioC_close_fn : iops.h
pub type lcb_ioC_close_fn = Option<extern "C" fn(
    iops: lcb_io_opt_t,
    sd: *mut lcb_sockdata_t
) -> c_uint>;

// lcb_ioC_cntl_fn : iops.h
pub type lcb_ioC_cntl_fn = Option<extern "C" fn(
    iops: lcb_io_opt_t,
    sd: *mut lcb_sockdata_t,
    mode: c_int,
    option: c_int,
    arg: *mut c_void
) -> c_int>;

// lcb_ioC_connect_fn : iops.h
pub type lcb_ioC_connect_fn = Option<extern "C" fn(
    iops: lcb_io_opt_t,
    sd: *mut lcb_sockdata_t,
    dst: *const sockaddr,
    naddr: c_uint,
    callback: lcb_io_connect_cb
) -> c_int>;

// lcb_ioC_nameinfo_fn : iops.h
pub type lcb_ioC_nameinfo_fn = Option<extern "C" fn(
    iops: lcb_io_opt_t,
    sock: *mut lcb_sockdata_t,
    ni: *mut lcb_nameinfo_st
) -> c_int>;

// lcb_ioC_read2_callback : iops.h
pub type lcb_ioC_read2_callback = Option<extern "C" fn(
    sd: *mut lcb_sockdata_t,
    nread: lcb_SSIZE,
    arg: *mut c_void
) -> ()>;

// lcb_ioC_read2_fn : iops.h
pub type lcb_ioC_read2_fn = Option<extern "C" fn(
    iops: lcb_io_opt_t,
    sd: *mut lcb_sockdata_t,
    iov: *mut lcb_IOV,
    niov: lcb_SIZE,
    uarg: *mut c_void,
    callback: lcb_ioC_read2_callback
) -> c_int>;

// lcb_ioC_read_callback : iops.h
/*deprecated*/
pub type lcb_ioC_read_callback = Option<extern "C" fn(
    sd: *mut lcb_sockdata_t,
    nread: lcb_SSIZE
) -> ()>;

// lcb_ioC_read_fn : iops.h
/*deprecated See lcb_ioC_read2_fn(). Wrapped if not implemented*/
// typedef int (*lcb_ioC_read_fn)(lcb_io_opt_t,lcb_sockdata_t*,lcb_ioC_read_callback);
pub type lcb_ioC_read_fn = Option<extern "C" fn(
    iops: lcb_io_opt_t,
    sd: *mut lcb_sockdata_t,
    callback: lcb_ioC_read_callback
) -> c_int>;

// lcb_ioC_serve_callback : iops.h
pub type lcb_ioC_serve_callback = Option<extern "C" fn(
    sd_server: *mut lcb_sockdata_t,
    sd_client: *mut lcb_sockdata_t,
    status: c_int
) -> ()>;

// lcb_ioC_serve_fn : iops.h
pub type lcb_ioC_serve_fn = Option<extern "C" fn(
    iops: lcb_io_opt_t,
    server_socket: *mut lcb_sockdata_t,
    listen_addr: *const sockaddr,
    callback: lcb_ioC_serve_callback
) -> c_int>;

// lcb_ioC_socket_fn : iops.h
pub type lcb_ioC_socket_fn = Option<extern "C" fn(
    iops: lcb_io_opt_t,
    domain: c_int,
    _type: c_int,
    protocol: c_int
) -> *mut lcb_sockdata_t>;

// lcb_ioC_wballoc_fn : iops.h
/*deprecated See lcb_ioC_write2_fn(). Wrapped if not implemented*/
// typedef lcb_io_writebuf_t* (*lcb_ioC_wballoc_fn)(lcb_io_opt_t,lcb_sockdata_t *);
pub type lcb_ioC_wballoc_fn = Option<extern "C" fn(
    iops: lcb_io_opt_t,
    sd: *mut lcb_sockdata_t
) -> *mut lcb_io_writebuf_t>;

// lcb_ioC_wbfree_fn : iops.h
/*deprecated See lcb_ioC_write2_fn(). Wrapped if not implemented */
// typedef void (*lcb_ioC_wbfree_fn)(lcb_io_opt_t,lcb_sockdata_t*,lcb_io_writebuf_t*);
pub type lcb_ioC_wbfree_fn = Option<extern "C" fn(
    iops: lcb_io_opt_t,
    sd: *mut lcb_sockdata_t,
    wb: *mut lcb_io_writebuf_t
) -> ()>;

// lcb_ioC_write2_callback : iops.h
pub type lcb_ioC_write2_callback = Option<extern "C" fn(
    sd: *mut lcb_sockdata_t,
    status: c_int,
    arg: *mut c_void
) -> ()>;

// lcb_ioC_write2_fn : iops.h
pub type lcb_ioC_write2_fn = Option<extern "C" fn(
    iops: lcb_io_opt_t,
    sd: *mut lcb_sockdata_t,
    iov: *mut lcb_IOV,
    niov: lcb_SIZE,
    uarg: *mut c_void,
    callback: lcb_ioC_write2_callback
) -> c_int>;

// lcb_ioC_write_callback : iops.h
/*deprecated See lcb_ioC_write2_fn(). This will be wrapped if not implemented */
// typedef void (*lcb_ioC_write_callback)(lcb_sockdata_t*,lcb_io_writebuf_t*,int);
pub type lcb_ioC_write_callback = Option<extern "C" fn(
    sd: *mut lcb_sockdata_t,
    wb: *mut lcb_io_writebuf_t,
    nwb: c_int
) -> ()>;

// lcb_ioC_write_fn : iops.h
/*deprecated*/
// typedef int (*lcb_ioC_write_fn)(lcb_io_opt_t,lcb_sockdata_t*,lcb_io_writebuf_t*,lcb_ioC_write_callback);
pub type lcb_ioC_write_fn = Option<extern "C" fn(
    iops: lcb_io_opt_t,
    sd: *mut lcb_sockdata_t,
    wb: *mut lcb_io_writebuf_t,
    callback: lcb_ioC_write_callback
) -> c_int>;


// lcb_ioE_callback : iops.h
pub type lcb_ioE_callback = Option<extern "C" fn(
    sock: lcb_socket_t,
    events: c_short,
    uarg: *mut c_void
) -> ()>;

// lcb_ioE_chkclosed_fn : iops.h
pub type lcb_ioE_chkclosed_fn = Option<extern "C" fn(
    iops: lcb_io_opt_t,
    sock: lcb_socket_t,
    flags: c_int
) -> c_int>;

// lcb_ioE_close_fn : iops.h
pub type lcb_ioE_close_fn = Option<extern "C" fn(
    iops: lcb_io_opt_t,
    sock: lcb_socket_t
) -> ()>;

// lcb_ioE_cntl_fn : iops.h
pub type lcb_ioE_cntl_fn = Option<extern "C" fn(
    iops: lcb_io_opt_t,
    sock: lcb_socket_t,
    mode: c_int,
    option: c_int,
    arg: *mut c_void
) -> c_int>;

// lcb_ioE_connect_fn : iops.h
pub type lcb_ioE_connect_fn = Option<extern "C" fn(
    iops: lcb_io_opt_t,
    sock: lcb_socket_t,
    dst: *const sockaddr,
    addrlen: c_uint
) -> c_int>;

// lcb_ioE_event_cancel_fn : iops.h
pub type lcb_ioE_event_cancel_fn = Option<extern "C" fn(
    iops: lcb_io_opt_t,
    sock: lcb_socket_t,
    event: *mut c_void
) -> ()>;

// lcb_ioE_event_create_fn : iops.h
pub type lcb_ioE_event_create_fn = Option<extern "C" fn(
    iops: lcb_io_opt_t
) -> *mut c_void>;

// lcb_ioE_event_destroy_fn : iops.h
pub type lcb_ioE_event_destroy_fn = Option<extern "C" fn(
    iops: lcb_io_opt_t,
    event: *mut c_void
) -> ()>;

// lcb_ioE_event_watch_fn : iops.h
pub type lcb_ioE_event_watch_fn = Option<extern "C" fn(
    iops: lcb_io_opt_t,
    socket: lcb_socket_t,
    event: *mut c_void,
    evflags: c_short,
    uarg: *mut c_void,
    callback: lcb_ioE_callback
) -> c_int>;

// lcb_ioE_recv_fn : iops.h
pub type lcb_ioE_recv_fn = Option<extern "C" fn(
    iops: lcb_io_opt_t,
    sock: lcb_socket_t,
    target_buf: *mut c_void,
    buflen: lcb_SIZE,
    _unused_flags: c_int
) -> lcb_SSIZE>;

// lcb_ioE_recvv_fn : iops.h
pub type lcb_ioE_recvv_fn = Option<extern "C" fn(
    iops: lcb_io_opt_t,
    sock: lcb_socket_t,
    iov: *mut lcb_IOV,
    niov: lcb_SIZE
) -> lcb_SSIZE>;

// lcb_ioE_send_fn : iops.h
pub type lcb_ioE_send_fn = Option<extern "C" fn(
    iops: lcb_io_opt_t,
    sock: lcb_socket_t,
    srcbuf: *const c_void,
    buflen: lcb_SIZE,
    _ignored: c_int
) -> lcb_SSIZE>;

// lcb_ioE_sendv_fn : iops.h
pub type lcb_ioE_sendv_fn = Option<extern "C" fn(
    iops: lcb_io_opt_t,
    sock: lcb_socket_t,
    iov: *mut lcb_IOV,
    niov: lcb_SIZE
) -> lcb_SSIZE>;

// lcb_ioE_socket_fn : iops.h
pub type lcb_ioE_socket_fn = Option<extern "C" fn(
    iops: lcb_io_opt_t,
    domain: c_int,
    _type: c_int,
    protocol: c_int
) -> lcb_socket_t>;

// PRIVATE lcb_ioE_bind_fn : iops.h
pub type lcb_ioE_bind_fn = Option<extern "C" fn(
    iops: lcb_io_opt_t,
    sock: lcb_socket_t,
    srcaddr: *const sockaddr,
    addrlen: c_uint
) -> c_int>;

// PRIVATE lcb_ioE_listen_fn : iops.h
pub type lcb_ioE_listen_fn = Option<extern "C" fn(
    iops: lcb_io_opt_t,
    bound_sock: lcb_socket_t,
    queuelen: c_uint
) -> c_int>;

// PRIVATE lcb_ioE_accept_fn : iops.h
pub type lcb_ioE_accept_fn = Option<extern "C" fn(
    iops: lcb_io_opt_t,
    lsnsock: lcb_socket_t
) -> lcb_socket_t>;

// lcb_socket_t : iops.h
pub type lcb_socket_t = c_int;

pub type lcb__iops3fndummy = Option<extern "C" fn() -> ()>;

#[repr(C)]
#[derive(Copy)]
pub struct sockaddr;
impl Clone for sockaddr {
    fn clone(&self) -> Self { *self }
}
impl Default for sockaddr {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_sockdata_st {
    pub socket: lcb_socket_t,
    pub parent: lcb_io_opt_t,
    pub lcbconn: *mut lcbio_SOCKET,
    pub closed: c_int,
    pub is_reading: c_int,
    pub read_buffer: lcb_buf_info,
}
impl Clone for lcb_sockdata_st {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_sockdata_st {
    fn default() -> Self { unsafe { zeroed() } }
}
pub type lcb_sockdata_t = lcb_sockdata_st;

#[repr(C)]
#[derive(Copy)]
pub struct lcbio_SOCKET;
impl Clone for lcbio_SOCKET {
    fn clone(&self) -> Self { *self }
}
impl Default for lcbio_SOCKET {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
// deprecated
pub struct lcb_io_writebuf_st {
    pub parent: *mut lcb_io_opt_st,
    pub buffer: lcb_buf_info,
}
impl Clone for lcb_io_writebuf_st {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_io_writebuf_st {
    fn default() -> Self { unsafe { zeroed() } }
}
pub type lcb_io_writebuf_t = lcb_io_writebuf_st;

// deprecated
#[repr(C)]
#[derive(Copy)]
pub struct lcb_buf_info {
    pub root: *mut c_char,
    pub size: lcb_SIZE,
    pub ringbuffer: ringbuffer_st,
    pub lcb_buf_info: [ringbuffer_st; 2usize],
}
impl Clone for lcb_buf_info {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_buf_info {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_nameinfo_st_s {
    pub name: *mut sockaddr,
    pub len: *mut c_int,
}
impl Clone for lcb_nameinfo_st_s {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_nameinfo_st_s {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_nameinfo_st {
    pub local: lcb_nameinfo_st_s,
    pub remote: lcb_nameinfo_st_s,
}
impl Clone for lcb_nameinfo_st {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_nameinfo_st {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct ringbuffer_st;
impl Clone for ringbuffer_st {
    fn clone(&self) -> Self { *self }
}
impl Default for ringbuffer_st {
    fn default() -> Self { unsafe { zeroed() } }
}


#[repr(C)]
#[derive(Copy)]
pub struct lcb_iovec_st {
    pub iov_base: *mut c_void,
    pub iov_len: size_t,
}
impl Clone for lcb_iovec_st {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_iovec_st {
    fn default() -> Self { unsafe { zeroed() } }
}
pub type lcb_IOV = lcb_iovec_st;

#[repr(C)]
#[derive(Copy)]
pub struct lcb_io_opt_st_u_base {
    pub cookie: *mut c_void,
    pub error: c_int,
    pub need_cleanup: c_int,
}
impl Clone for lcb_io_opt_st_u_base {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_io_opt_st_u_base {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcbio_TABLE;
impl Clone for lcbio_TABLE {
    fn clone(&self) -> Self { *self }
}
impl Default for lcbio_TABLE {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_timerprocs_st {
    pub create: lcb_io_timer_create_fn,
    pub destroy: lcb_io_timer_destroy_fn,
    pub cancel: lcb_io_timer_cancel_fn,
    pub schedule: lcb_io_timer_schedule_fn,
}
impl Clone for lcb_timerprocs_st {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_timerprocs_st {
    fn default() -> Self { unsafe { zeroed() } }
}
pub type lcb_timer_procs = lcb_timerprocs_st;

#[repr(C)]
#[derive(Copy)]
pub struct lcb_loopprocs_st {
    pub start: lcb_io_start_fn,
    pub stop: lcb_io_stop_fn,
    pub tick: lcb_io_tick_fn,
}
impl Clone for lcb_loopprocs_st {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_loopprocs_st {
    fn default() -> Self { unsafe { zeroed() } }
}
pub type lcb_loop_procs = lcb_loopprocs_st;

#[repr(C)]
#[derive(Copy)]
pub struct lcb_bsdprocs_st {
    pub socket0: lcb_ioE_socket_fn,
    pub connect0: lcb_ioE_connect_fn,
    pub recv: lcb_ioE_recv_fn,
    pub recvv: lcb_ioE_recvv_fn,
    pub send: lcb_ioE_send_fn,
    pub sendv: lcb_ioE_sendv_fn,
    pub close: lcb_ioE_close_fn,
    pub bind: lcb_ioE_bind_fn,
    pub listen: lcb_ioE_listen_fn,
    pub accept: lcb_ioE_accept_fn,
    pub is_closed: lcb_ioE_chkclosed_fn,
    pub cntl: lcb_ioE_cntl_fn,
}
impl Clone for lcb_bsdprocs_st {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_bsdprocs_st {
    fn default() -> Self { unsafe { zeroed() } }
}
pub type lcb_bsd_procs = lcb_bsdprocs_st;

#[repr(C)]
#[derive(Copy)]
pub struct lcb_evprocs_st {
    pub create: lcb_ioE_event_create_fn,
    pub destroy: lcb_ioE_event_destroy_fn,
    pub cancel: lcb_ioE_event_cancel_fn,
    pub watch: lcb_ioE_event_watch_fn,
}
impl Clone for lcb_evprocs_st {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_evprocs_st {
    fn default() -> Self { unsafe { zeroed() } }
}
pub type lcb_ev_procs = lcb_evprocs_st;

#[repr(C)]
#[derive(Copy)]
pub struct lcb_completion_procs {
    pub socket: lcb_ioC_socket_fn,
    pub close: lcb_ioC_close_fn,
    pub read: lcb_ioC_read_fn,
    pub connect: lcb_ioC_connect_fn,
    pub wballoc: lcb_ioC_wballoc_fn,
    pub wbfree: lcb_ioC_wbfree_fn,
    pub write: lcb_ioC_write_fn,
    pub write2: lcb_ioC_write2_fn,
    pub read2: lcb_ioC_read2_fn,
    pub serve: lcb_ioC_serve_fn,
    pub nameinfo: lcb_ioC_nameinfo_fn,
    pub is_closed: lcb_ioC_chkclosed_fn,
    pub cntl: lcb_ioC_cntl_fn,
}
impl Clone for lcb_completion_procs {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_completion_procs {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_iops2_st {
    pub cookie: *mut c_void,
    pub error: c_int,
    pub need_cleanup: c_int,
    pub get_procs: lcb_io_procs_fn,
    pub iot: *mut lcbio_TABLE,
}
impl Clone for lcb_iops2_st {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_iops2_st {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_iops3_st {
    pub cookie: *mut c_void,
    pub error: c_int,
    pub need_cleanup: c_int,
    pub pads: [lcb__iops3fndummy; 17usize],
    pub get_procs: lcb_io_procs_fn,
    pub iot: *mut lcbio_TABLE,
}
impl Clone for lcb_iops3_st {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_iops3_st {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_io_opt_st_u {
    pub _data_: [u64; 22usize],
}
impl lcb_io_opt_st_u {
    pub unsafe fn base(&mut self) -> *mut lcb_io_opt_st_u_base {
        let raw: *mut u8 = transmute(&self._data_);
        transmute(raw.offset(0))
    }
    // First two are depricated anyway
    // pub unsafe fn v0(&mut self) -> *mut lcb_iops_evented_st {
    //     let raw: *mut u8 = transmute(&self._data_);
    //     transmute(raw.offset(0))
    // }
    // pub unsafe fn v1(&mut self) -> *mut lcb_iops_completion_st {
    //     let raw: *mut u8 = transmute(&self._data_);
    //     transmute(raw.offset(0))
    // }
    pub unsafe fn v2(&mut self) -> *mut lcb_iops2_st {
        let raw: *mut u8 = transmute(&self._data_);
        transmute(raw.offset(0))
    }
    pub unsafe fn v3(&mut self) -> *mut lcb_iops3_st {
        let raw: *mut u8 = transmute(&self._data_);
        transmute(raw.offset(0))
    }
}
impl Clone for lcb_io_opt_st_u {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_io_opt_st_u {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct lcb_io_opt_st {
    pub version: c_int,
    pub dlhandle: *mut c_void,
    pub destructor: Option<extern "C" fn(iops: *mut lcb_io_opt_st,) -> ()>,
    pub v: lcb_io_opt_st_u,
}
impl Clone for lcb_io_opt_st {
    fn clone(&self) -> Self { *self }
}
impl Default for lcb_io_opt_st {
    fn default() -> Self { unsafe { zeroed() } }
}
pub type lcb_io_opt_t = *mut lcb_io_opt_st;