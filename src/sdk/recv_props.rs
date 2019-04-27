pub type RecvVarProxyFn = fn(*const CRecvProxyData, *mut libc::c_void, *mut libc::c_void);
pub type ArrayLengthRecvProxyFn = fn(*mut libc::c_void, i32, i32);
pub type DataTableRecvVarProxyFn =
    fn(*const RecvProp, *mut *mut libc::c_void, *mut libc::c_void, i32);

#[repr(i32)]
pub enum SendPropType {
    Int = 0,
    Float,
    Vec,
    VecXY,
    String,
    Array,
    DataTable,
    Int64,
}

#[repr(C)]
pub union DVariantData {
    float: f32,
    int: i32,
    string: *const libc::c_char,
    data: *mut libc::c_void,
    vector: [f32; 3],
    int64: i64,
}

#[repr(C)]
pub struct DVariant {
    data: DVariantData,
    prop_type: SendPropType,
}

#[repr(C)]
pub struct RecvTable {
    props: *mut RecvProp,
    count: i32,
    decoder: *const libc::c_void,
    table_name: *const libc::c_char,
    is_initialized: bool,
    is_in_main_list: bool,
}

#[repr(C)]
pub struct RecvProp {
    prop_name: *const libc::c_char,
    prop_type: SendPropType,
    prop_flags: i32,
    buffer_size: i32,
    is_inside_array: i32,
    extra_data_ptr: *const libc::c_void,
    array_prop: *const RecvProp,
    array_length_proxy: ArrayLengthRecvProxyFn,
    proxy_fn: RecvVarProxyFn,
    data_table_proxy_fn: DataTableRecvVarProxyFn,
    data_table: *const RecvTable,
    offset: i32,
    element_stride: i32,
    elements_count: i32,
    parent_array_prop_name: *const libc::c_char,
}

#[repr(C)]
pub struct CRecvProxyData {
    recv_prop: *const RecvProp,
    value: DVariant,
    element_index: i32,
    object_id: i32,
}
