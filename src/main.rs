use std::{ffi::OsString, os::windows::ffi::OsStrExt};

use windows::{core::PCWSTR, Win32::System::{Diagnostics::Debug::Extensions::{DebugCreate, IDebugClient5, DEBUG_CLASS_USER_WINDOWS}, Threading::INFINITE}};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 || args[1] != "-t" {
        println!("Usage: dbgsrvrs -t <transport>");
    }
    let transport = &args[2];
    let transport: OsString = transport.into();
    let transport: Vec<u16> = transport.encode_wide().chain(std::iter::once(0)).collect();


    let debug_client = unsafe { DebugCreate::<IDebugClient5>() }.unwrap();
    unsafe { debug_client.StartProcessServerWide(DEBUG_CLASS_USER_WINDOWS, PCWSTR(transport.as_ptr()), None) }.unwrap();
    println!("Process server started with transport: {}", args[2]);
    unsafe { debug_client.WaitForProcessServerEnd(INFINITE) }.unwrap();
}
