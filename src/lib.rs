use serde_json::Value;
use std::ffi::CStr;

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn prepare(json_str: *const i8) -> i32 {
    let string = unsafe { CStr::from_ptr(json_str).to_string_lossy().into_owned() };
    let value = serde_json::from_str::<Value>(&string);
    if let Ok(j) = value.and_then(|v| serde_json::to_string_pretty(&v)) {
        j.len() as i32
    } else {
        0
    }
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn prettify(json_str: *const i8, pretty_json: *mut u8) -> i32 {
    if json_str.is_null() {
        return 0;
    }
    let string = unsafe { CStr::from_ptr(json_str).to_string_lossy().into_owned() };
    let value = serde_json::from_str::<Value>(&string);
    if let Ok(j) = value.and_then(|v| serde_json::to_string_pretty(&v)) {
        let len = j.len();
        unsafe {
            pretty_json.copy_from(j.as_ptr(), len);
        }
        len as i32
    } else {
        let len = string.len();
        unsafe {
            pretty_json.copy_from(string.as_ptr(), len);
        }
        len as i32
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        let origin_string = format!("{}\0", r#"{"a":"aa","b":{"bb":"bbb"},"c":["cc"]}"#);
        let mut result = [0_u8; 68];
        unsafe {
            prettify(origin_string.as_ptr() as *const i8, result.as_mut_ptr());
        }

        #[rustfmt::skip]
        let target_string =
r#"{
  "a": "aa",
  "b": {
    "bb": "bbb"
  },
  "c": [
    "cc"
  ]
}"#;
        assert_eq!(String::from_utf8(result.to_vec()).unwrap(), target_string);
    }

    #[test]
    fn no_panic() {
        let origin_string = format!("{}\0", r#"1234567890asdfghjkl"#);
        let mut result = [0_u8; 20];
        unsafe {
            prettify(origin_string.as_ptr() as *const i8, result.as_mut_ptr());
        }
        assert_eq!(String::from_utf8(result.to_vec()).unwrap(), origin_string);
    }
}
