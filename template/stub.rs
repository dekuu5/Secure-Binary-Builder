//! A stub that decrypts and runs the embedded binary in memory.
extern crate libc;

// Use absolute paths to the modules in the src directory
use crate::crypto;
use crate::fingerprint;
use crate::embed;
use std::process;

#[cfg(unix)]
use std::os::fd::{FromRawFd, AsRawFd};

fn main() {
    // 1. Read current executable
    let exe_data = std::fs::read(std::env::current_exe().unwrap()).unwrap();

    // 2. Extract encrypted payload
    let encrypted = embed::extract_from_stub(&exe_data).unwrap();

    // 3. Decrypt using machine fingerprint
    let fp = fingerprint::generate_fingerprint();
    let decrypted = crypto::decrypt_binary(&fp, &encrypted).unwrap();

    // 4. Execute in memory (no temp files)
    if let Err(e) = run_in_memory(&decrypted) {
        eprintln!("Failed to run binary: {}", e);
        process::exit(1);
    }
}

/// Execute a binary directly from memory (Unix)
#[cfg(unix)]
fn run_in_memory(binary: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    use std::ffi::CString;
    use std::io::Write;

    let name = CString::new("sbb_temp")?;
    let fd = unsafe { libc::memfd_create(name.as_ptr(), 0) };
    if fd == -1 { return Err("memfd_create failed".into()); }

    let mut file = unsafe { std::fs::File::from_raw_fd(fd) };
    file.write_all(binary)?;

    let path = format!("/proc/self/fd/{}", fd);
    let path_cstr = CString::new(path)?;
    let args_cstr = CString::new("")?;

    unsafe {
        libc::execl(
            path_cstr.as_ptr(),
            args_cstr.as_ptr(),
            std::ptr::null(),
        );
    }
    Err("Failed to execute binary".into())
}

/// Execute a binary directly from memory (Windows)
#[cfg(windows)]
fn run_in_memory(binary: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    use std::ptr;
    use winapi::um::memoryapi::{VirtualAlloc, VirtualProtect};
    use winapi::um::processthreadsapi::{CreateThread, WaitForSingleObject};
    use winapi::um::winnt::{MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READWRITE, PAGE_READWRITE};

    // Allocate memory
    let size = binary.len();
    let mem = unsafe { VirtualAlloc(
        ptr::null_mut(),
        size,
        MEM_COMMIT | MEM_RESERVE,
        PAGE_READWRITE,
    ) };
    if mem.is_null() { return Err("VirtualAlloc failed".into()); }

    // Copy binary into memory
    unsafe { ptr::copy_nonoverlapping(binary.as_ptr(), mem as *mut u8, size) };

    // Make memory executable
    let mut old_protect = 0;
    let prot_ok = unsafe { VirtualProtect(
        mem,
        size,
        PAGE_EXECUTE_READWRITE,
        &mut old_protect,
    ) };
    if prot_ok == 0 { return Err("VirtualProtect failed".into()); }

    // Execute
    let thread = unsafe { CreateThread(
        ptr::null_mut(),
        0,
        Some(std::mem::transmute(mem)),
        ptr::null_mut(),
        0,
        ptr::null_mut(),
    ) };
    if thread.is_null() { return Err("CreateThread failed".into()); }

    unsafe { WaitForSingleObject(thread, winapi::um::winbase::INFINITE) };
    Ok(())
}