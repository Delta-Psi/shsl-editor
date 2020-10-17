use crate::script::Script;
use libc::{size_t, c_char};

#[repr(C)]
pub struct Data {
    pub ptr: *const c_char,
    pub size: size_t,
}

impl Data {
    unsafe fn from_slice(slice: &[u8]) -> Self {
        Self {
            ptr: slice.as_ptr() as *const c_char,
            size: slice.len(),
        }
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
pub extern fn script_string_count(script: *const Script) -> size_t {
    let script = unsafe {
        script.as_ref().unwrap()
    };

    script.strings.len()
}

#[no_mangle]
pub extern fn script_string_get(script: *const Script, index: size_t) -> Data {
    let script = unsafe {
        script.as_ref().unwrap()
    };
    let string = &script.strings[index];

    unsafe {
        Data::from_slice(string.as_bytes())
    }
}

#[no_mangle]
pub extern fn delete_script(script: *mut Script) {
    assert!(!script.is_null());
    unsafe {
        Box::from_raw(script);
    }
}
