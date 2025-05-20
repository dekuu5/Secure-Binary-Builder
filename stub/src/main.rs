//! A stub that decrypts and runs the embedded binary in memory.
extern crate libc;

use common::crypto;
use common::fingerprint;
use common::embed;


// Use absolute 
use std::process;

#[cfg(unix)]
use std::os::fd::{FromRawFd, AsRawFd};

fn main() {
    println!("[*] Stub running...");

    // 1. Read self
    let exe_path = std::env::current_exe().unwrap();
    println!("[*] Current exe path: {:?}", exe_path);
    let exe_data = std::fs::read(&exe_path).expect("Failed to read current executable");
    println!("[+] ({} bytes)", exe_data.len());
    // 2. Extract encrypted payload
    let maybe_encrypted = embed::extract_from_stub(&exe_data);
    if maybe_encrypted.is_none() {
        eprintln!("❌ No embedded binary found in stub.");
        std::process::exit(1);
    }
    let encrypted = maybe_encrypted.unwrap();
    println!("[+] Extracted encrypted payload ({} bytes)", encrypted.len());

    // 3. Decrypt using fingerprint
    let fp = fingerprint::generate_fingerprint();
    println!("[*] Fingerprint used: {}", fp);

    let maybe_decrypted = crypto::decrypt_binary(&fp, &encrypted);
    if maybe_decrypted.is_none() {
        eprintln!("❌ Decryption failed. Possibly wrong fingerprint or corrupted data.");
        std::process::exit(1);
    }
    let decrypted = maybe_decrypted.unwrap();
    println!("[+] Decryption succeeded. Decrypted binary size: {} bytes", decrypted.len());

    // 4. Execute in memory
    println!("[*] Attempting to execute decrypted binary in memory...");
    if let Err(e) = run_in_memory(&decrypted) {
        eprintln!("❌ Failed to run binary: {}", e);
        std::process::exit(1);
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
            std::ptr::null::<std::ffi::c_void>(),
        );
    }
    Err("Failed to execute binary".into())
}
#[cfg(windows)]
fn run_in_memory(binary: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    use std::fs;
    use std::path::Path;
    use std::ffi::CString;
    use winapi::um::processthreadsapi::{CreateProcessA, PROCESS_INFORMATION, STARTUPINFOA};
    use winapi::shared::minwindef::FALSE;
    
    // Write to temporary file (more reliable for Windows)
    let temp_path = std::env::temp_dir().join("sbb_temp.exe");
    fs::write(&temp_path, binary)?;
    
    // Convert path to CString
    let path_str = temp_path.to_string_lossy().to_string();
    let path_cstr = CString::new(path_str)?;
    
    // Initialize process structures
    let mut startup_info: STARTUPINFOA = unsafe { std::mem::zeroed() };
    startup_info.cb = std::mem::size_of::<STARTUPINFOA>() as u32;
    
    let mut process_info: PROCESS_INFORMATION = unsafe { std::mem::zeroed() };
    
    // Create process
    let success = unsafe {
        CreateProcessA(
            path_cstr.as_ptr(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            FALSE,
            0,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            &mut startup_info,
            &mut process_info
        )
    };
    
    // Clean up temp file
    let _ = fs::remove_file(temp_path);
    
    if success == 0 {
        return Err("CreateProcess failed".into());
    }
    
    Ok(())
}