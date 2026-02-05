use serde_json::Value;
use std::ffi::CStr;

/// # Safety
#[unsafe(no_mangle)]
pub unsafe extern "C" fn prepare(json_str: *const i8) -> i32 {
    if json_str.is_null() {
        return 0;
    }

    let string = unsafe { CStr::from_ptr(json_str).to_string_lossy().into_owned() };
    let Ok(value) = serde_json::from_str::<Value>(&string) else {
        return 0;
    };

    let Ok(j) = serde_json::to_string_pretty(&value) else {
        return 0;
    };
    j.len() as i32
}

/// # Safety
#[unsafe(no_mangle)]
pub unsafe extern "C" fn prettify(json_str: *const i8, pretty_json: *mut u8) -> i32 {
    if json_str.is_null() {
        return 0;
    }

    let mut string = unsafe { CStr::from_ptr(json_str).to_string_lossy().into_owned() };
    if let Ok(value) = serde_json::from_str::<Value>(&string)
        && let Ok(j) = serde_json::to_string_pretty(&value)
    {
        string = j;
    }

    let len = string.len();
    unsafe {
        pretty_json.copy_from(string.as_ptr(), len);
    }
    len as i32
}

/// # Safety
#[unsafe(no_mangle)]
pub unsafe extern "C" fn deprettify(json_str: *const i8, depretty_json: *mut u8) -> i32 {
    if json_str.is_null() {
        return 0;
    }

    let mut string = unsafe { CStr::from_ptr(json_str).to_string_lossy().into_owned() };
    if let Ok(value) = serde_json::from_str::<Value>(&string)
        && let Ok(j) = serde_json::to_string(&value)
    {
        string = j;
    }

    let len = string.len();
    unsafe {
        depretty_json.copy_from(string.as_ptr(), len);
    }
    len as i32
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn prettify_works() {
        // 添加结束符\0避免越界
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
    fn deprettify_works() {
        #[rustfmt::skip]
        let origin_string = format!("{}\0",
r#"{
  "a": "aa",
  "b": {
    "bb": "bbb"
  },
  "c": [
    "cc"
  ]
}"#);
        let mut result = [0_u8; 38];
        unsafe {
            let l = deprettify(origin_string.as_ptr() as *const i8, result.as_mut_ptr());
            println!("{l}");
        }

        let target_string = r#"{"a":"aa","b":{"bb":"bbb"},"c":["cc"]}"#;
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
