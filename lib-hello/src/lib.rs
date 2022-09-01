use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};

#[repr(C)]
pub struct User {
    name: CString,
    age: c_int,
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn get_some_cstr(s: *mut *mut c_char) -> i32 {
    if s.is_null() || !(*s).is_null() {
        return libc::EINVAL as i32;
    }
    let val = CString::new("A string created in Rust").expect("Fail to allocate a CString");
    *s = val.into_raw();
    0
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn cstr_free(s: *mut *mut c_char) -> i32 {
    if s.is_null() || (*s).is_null() {
        return libc::EINVAL as i32;
    }
    drop(CString::from_raw(*s));
    *s = std::ptr::null_mut::<i8>();
    0
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn user_new_with_result(
    user: *mut *mut User,
    name: *const c_char,
    age: i32,
) -> i32 {
    if user.is_null() || !(*user).is_null() {
        return libc::EINVAL;
    }
    let u = Box::new(User {
        name: CStr::from_ptr(name).to_owned(),
        age,
    });
    *user = Box::into_raw(u);
    0
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn user_free_with_result(user: *mut *mut User) -> i32 {
    if user.is_null() || (*user).is_null() {
        return libc::EINVAL as i32;
    }
    drop(Box::from_raw(*user));
    *user = std::ptr::null_mut::<User>();
    0
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn user_new(name: *mut c_char, age: i32) -> Box<User> {
    Box::new(User {
        name: CString::from_raw(name),
        age,
    })
}

#[no_mangle]
pub extern "C" fn user_free(_: Option<Box<User>>) {
    println!("ddd");
}

impl Drop for User {
    fn drop(&mut self) {
        println!("dropping User object...");
    }
}

///# Safety
#[no_mangle]
pub unsafe extern "C" fn user_name_get(user: &User) -> *const c_char {
    user.name.as_ptr()
}

#[no_mangle]
pub extern "C" fn user_age_get(user: &User) -> c_int {
    user.age
}
