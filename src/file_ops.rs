use nix::dir::Dir;
use nix::fcntl::OFlag;
use std::ffi::{CStr};
use std::fs::File;
use std::io::Read;
use std::mem;
use std::os::fd::{AsRawFd, FromRawFd};
use std::thread::sleep;
use std::time::Duration;

use nix::libc::{c_char, FAN_REPORT_FID, readlink};
use nix::sys::fanotify::{EventFFlags, Fanotify, InitFlags, MarkFlags, MaskFlags};
use nix::sys::stat::Mode;

#[cfg(target_os = "linux")]
pub fn get_fan() -> Fanotify {
    let fa_fd = Fanotify::init(
        InitFlags::FAN_CLASS_NOTIF | InitFlags::FAN_REPORT_TID | InitFlags::from_bits_retain(FAN_REPORT_FID).unwrap(),
        EventFFlags::O_RDWR,
    )
    .unwrap_or_else(|e| {
        eprintln!("{e}");
        panic!()
    });
    println!("fa inited");
    fa_fd
}

#[cfg(target_os = "linux")]
pub fn read_events(fa_fd: &Fanotify) {
    loop {
        let events = fa_fd.read_events().unwrap_or(vec![]);
        println!("{} events received", events.len());
        for e in events {
            let accessed = e.fd().unwrap();
            let mut file = unsafe { File::from_raw_fd(accessed.clone().as_raw_fd()) };
            let mut content_buf = [0; 4096];
            _ = file.read(&mut content_buf);
            mem::forget(file);
            let content = String::from_utf8(content_buf.to_vec()).unwrap();
            println!("{content}");
            if content.contains("VIRA") {
                let mut buf: Vec<c_char> = vec!['\t' as _; 256];
                println!("buffed");
                let fd_path =
                    format!("/proc/self/fd/{}", accessed.as_raw_fd()).as_ptr() as *const c_char;
                println!("fdpathed");
                _ = unsafe {
                    readlink(fd_path, buf.as_mut_ptr(), 256);
                };
                println!("readlink");
                let c_str = unsafe { CStr::from_ptr(buf.as_mut_ptr()) };
                println!("converted");
                let fp = c_str.to_str().unwrap_or_else(|e| {
                    eprintln!("{e}");
                    ""
                });
                println!("Virus detected in {}", fp);
            }
        }
        sleep(Duration::from_millis(1400));
    }
}

#[cfg(target_os = "linux")]
pub fn set_file_for_fan(fan: &Fanotify, file_name: &str) {
    fan.mark::<str>(
        MarkFlags::FAN_MARK_ADD,
        MaskFlags::FAN_OPEN | MaskFlags::FAN_MODIFY | MaskFlags::FAN_CLOSE,
        None,
        Some(file_name),
    )
    .unwrap_or_else(|e| eprintln!("{e}, file"))
}

#[cfg(target_os = "linux")]
pub fn set_dir_for_fan(fan: &Fanotify, dir_path: String) {
    let open_flag = OFlag::O_DIRECTORY;
    let open_mode = Mode::S_IRWXG | Mode::S_IRWXU | Mode::S_IRWXO;
    let dir = Dir::open(dir_path.as_str(), open_flag, open_mode).unwrap_or_else(|e| {
        eprintln!("rq dir f, {e}");
        Dir::open("/", open_flag, open_mode).unwrap()
    });
    println!("dir opening");
    fan.mark::<str>(
        MarkFlags::FAN_MARK_ADD | MarkFlags::FAN_MARK_ONLYDIR,
        MaskFlags::FAN_OPEN | MaskFlags::FAN_EVENT_ON_CHILD | MaskFlags::FAN_MOVED_FROM,
        Some(dir.as_raw_fd()),
        None,
    )
    .unwrap_or_else(|e| eprintln!("{e}, dir"))
}

#[cfg(target_os = "linux")]
pub fn clear_fan(fan: &Fanotify) {
    fan.mark::<str>(MarkFlags::FAN_MARK_FLUSH, MaskFlags::empty(), None, None)
        .unwrap_or_else(|e| eprintln!("{e}"));
    std::process::exit(0);
}
