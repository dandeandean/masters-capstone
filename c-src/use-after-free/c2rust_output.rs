#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct String_0 {
    pub s: *mut libc::c_char,
    pub len: libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn consume(mut s: *mut String_0) {
    printf(
        b"string struct: {'%s', len=%d}\n\0" as *const u8 as *const libc::c_char,
        (*s).s,
        (*s).len,
    );
    free(s as *mut libc::c_void);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut s: *mut String_0 = malloc(::std::mem::size_of::<String_0>() as libc::c_ulong)
        as *mut String_0;
    if argc < 2 as libc::c_int {
        printf(b"Usage: ./run <string>\n\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    let ref mut fresh0 = (*s).s;
    *fresh0 = *argv.offset(1 as libc::c_int as isize);
    (*s).len = strlen((*s).s) as libc::c_int;
    consume(s);
    consume(s);
    return 0 as libc::c_int;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}

