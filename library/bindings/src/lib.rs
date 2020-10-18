use shsl_library::script::Script;
use libc::{size_t, c_char};

#[repr(C)]
pub struct Data {
    pub ptr: *const c_char,
    pub size: size_t,
}

impl Data {
    pub unsafe fn from_slice(slice: &[u8]) -> Self {
        Self {
            ptr: slice.as_ptr() as *const c_char,
            size: slice.len(),
        }
    }
}

#[repr(C)]
pub struct OwnedData {
    pub ptr: *mut c_char,
    pub size: size_t,
}

impl OwnedData {
    pub unsafe fn from_boxed_bytes(bytes: Box<[u8]>) -> Self {
        let size = bytes.len();
        Self {
            ptr: Box::into_raw(bytes) as *mut c_char,
            size,
        }
    }
}

#[no_mangle]
pub extern fn delete_owned_data(data: OwnedData) {
    unsafe {
        drop(Box::from_raw(data.ptr));
    }
}

#[no_mangle]
pub extern fn decode_script(data: *const c_char, length: size_t) -> *mut Script {
    assert!(!data.is_null());
    let data = unsafe {
        std::slice::from_raw_parts(data as *const u8, length)
    };

    match Script::decode(data) {
        Some(script) => Box::into_raw(Box::new(script)),
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern fn script_decompile(script: *const Script) -> OwnedData {
    let script = unsafe {
        script.as_ref().unwrap()
    };

    let decompiled = script.decompile();
    unsafe {
        OwnedData::from_boxed_bytes(decompiled.into_boxed_str().into_boxed_bytes())
    }
}

#[no_mangle]
pub extern fn delete_script(script: *mut Script) {
    assert!(!script.is_null());
    unsafe {
        drop(Box::from_raw(script));
    }
}
