use core::slice;

use alloc::borrow::ToOwned;
use utils::io::Write;

use crate::{
    TBW, critical_section,
    device_manager::DEVICES,
    drivers::ps2,
    interrupts::InterruptContext,
    print, println,
    process::{self, PROCESSES, get_process},
    x86_utils::{sti, tsc_sleep},
};

const INVALID_ARGS: i32 = -1;
const UNKNOWN_SYSCALL: i32 = -2;
const WRITE_ERROR: i32 = -3;
const EXIT_ERROR: i32 = -4;

pub fn generic_handler(ctx: &mut InterruptContext) {
    ctx.eax = match ctx.eax {
        1 => exit(ctx.ebx),
        3 => read(),
        4 => {
            if ctx.ebx == 0 {
                INVALID_ARGS
            } else {
                let buf = unsafe { slice::from_raw_parts(ctx.ebx as _, ctx.ecx as _) };
                write(buf)
            }
        }
        10 => get_fb_addr(),
        11 => get_fb_width(),
        12 => get_fb_height(),
        _ => UNKNOWN_SYSCALL,
    } as _
}

fn read() -> i32 {
    DEVICES.ps2keyboard.read() as i32
}

fn exit(code: u32) -> ! {
    let process = get_process(0).unwrap();
    write!(process.tbw, "EXIT WITH CODE {}\n", code).unwrap();
    process.kill();
}

fn write(buf: &[u8]) -> i32 {
    match get_process(0).unwrap().tbw.write(buf) {
        Ok(count) => count as _,
        Err(_) => WRITE_ERROR,
    }
}

fn get_fb_addr() -> i32 {
    unsafe { crate::framebuffer_addr as i32 }
}

fn get_fb_width() -> i32 {
    unsafe { crate::framebuffer_width as i32 }
}

fn get_fb_height() -> i32 {
    unsafe { crate::framebuffer_height as i32 }
}
