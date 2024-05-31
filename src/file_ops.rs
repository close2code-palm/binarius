use std::ffi::CStr;
use std::fs::File;
use std::io::Read;
use std::os::fd::{AsRawFd, FromRawFd};
use std::thread::sleep;
use std::time::Duration;

use nix::libc::{c_char, readlink};
use nix::sys::fanotify::{EventFFlags, Fanotify, InitFlags, MarkFlags, MaskFlags};

#[cfg(target_os = "linux")]
pub fn get_fan() -> Fanotify {
    let fa_fd = Fanotify::init(
        InitFlags::FAN_CLASS_NOTIF | InitFlags::FAN_REPORT_TID,
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
            let mut file = unsafe { File::from_raw_fd(accessed.as_raw_fd()) };
            let mut content_buf = [0; 4096];
            _ = file.read(&mut content_buf);
            let content = String::from_utf8(content_buf.to_vec()).unwrap();
            println!("{content}");
            if content
                .contains("X5O!P%@AP[4\\PZX54(P^)7CC)7}$EICAR-STANDARD-ANTIVIRUS-TEST-FILE!$H+H*")
            {
                let buf: *mut c_char = std::ptr::null_mut();
                let fd_path =
                    format!("/proc/self/fd/{}", accessed.as_raw_fd()).as_ptr() as *const c_char;
                _ = unsafe {
                    readlink(fd_path, buf, 256);
                };
                let c_str = unsafe { CStr::from_ptr(buf) };
                let fp = c_str.to_str().unwrap();
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
    // let open_flag = OFlag::O_DIRECTORY;
    // let open_mode = Mode::S_IRWXG | Mode::S_IRWXU | Mode::S_IRWXO;
    // let dir = Dir::open(dir_path.as_str(), open_flag, open_mode)
    //     .unwrap_or_else(|_| Dir::open("/", open_flag, open_mode).unwrap());
    println!("dir opening");
    fan.mark::<str>(
        MarkFlags::FAN_MARK_ADD,
        MaskFlags::FAN_OPEN | MaskFlags::FAN_DELETE | MaskFlags::FAN_ONDIR,
        None,
        Some(dir_path.as_str()),
    )
    .unwrap_or_else(|e| eprintln!("{e}, dir"))
}

#[cfg(target_os = "linux")]
pub fn clear_fan(fan: &Fanotify) {
    fan.mark::<str>(MarkFlags::FAN_MARK_FLUSH, MaskFlags::empty(), None, None)
        .unwrap_or_else(|e| eprintln!("{e}"));
    std::process::exit(0);
}
