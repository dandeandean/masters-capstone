#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let secret: [libc::c_char; 8] = *::std::mem::transmute::<
        &[u8; 8],
        &[libc::c_char; 8],
    >(b"letmein\0");
    let mut buf: [libc::c_char; 20] = [0; 20];
    if argc < 2 as libc::c_int {
        printf(b"Usage: ./run <guess>\n\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    if argc > 1 as libc::c_int {
        strcpy(buf.as_mut_ptr(), *argv.offset(1 as libc::c_int as isize));
    }
    if 0 as libc::c_int
        == strncmp(
            buf.as_mut_ptr(),
            secret.as_ptr(),
            ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
        )
    {
        printf(b"You guessed correctly!\n\0" as *const u8 as *const libc::c_char);
        printf(
            b"The secret was: %s \n\0" as *const u8 as *const libc::c_char,
            secret.as_ptr(),
        );
        return 0 as libc::c_int;
    }
    printf(
        b"'%s' was not the secret.\nTry again later!\n\0" as *const u8
            as *const libc::c_char,
        buf.as_mut_ptr(),
    );
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

