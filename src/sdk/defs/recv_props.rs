use std::ffi::c_void;

pub type RecvVarProxyFn =
fn(data: *const CRecvProxyData, struct_ptr: *mut c_void, out_ptr: *mut c_void);
pub type ArrayLengthRecvProxyFn =
fn(struct_ptr: *mut c_void, object_id: i32, current_array_length: i32);
pub type DataTableRecvVarProxyFn =
fn(prop: *const RecvProp, out_ptr: *mut *mut c_void, data_ptr: *mut c_void, object_id: i32);

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
    pub float: f32,
    pub int: i32,
    pub string: *const libc::c_char,
    pub data: *mut c_void,
    pub vector: [f32; 3],
    pub int64: i64,
}

#[repr(C)]
pub struct DVariant {
    pub data: DVariantData,
    pub prop_type: SendPropType,
}

#[repr(C)]
pub struct RecvTable {
    pub props: *mut RecvProp,
    pub count: i32,
    pub decoder: *const c_void,
    pub table_name: *const libc::c_char,
    pub is_initialized: bool,
    pub is_in_main_list: bool,
}

#[repr(C)]
pub struct RecvProp {
    pub prop_name: *const libc::c_char,
    pub prop_type: SendPropType,
    pub prop_flags: i32,
    pub buffer_size: i32,
    pub is_inside_array: i32,
    pub extra_data_ptr: *const c_void,
    pub array_prop: *const RecvProp,
    pub array_length_proxy: ArrayLengthRecvProxyFn,
    pub proxy_fn: RecvVarProxyFn,
    pub data_table_proxy_fn: DataTableRecvVarProxyFn,
    pub data_table: *const RecvTable,
    pub offset: i32,
    pub element_stride: i32,
    pub elements_count: i32,
    pub parent_array_prop_name: *const libc::c_char,
}

#[repr(C)]
pub struct CRecvProxyData {
    pub recv_prop: *const RecvProp,
    pub value: DVariant,
    pub element_index: i32,
    pub object_id: i32,
}
