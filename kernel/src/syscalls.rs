use core::slice;

use utils::io::Write;

use crate::{
    TBW, critical_section,
    device_manager::DEVICES,
    drivers::ps2,
    interrupts::InterruptContext,
    print, println,
    x86_utils::{sti, tsc_sleep},
};

const INVALID_ARGS: i32 = -1;
const UNKNOWN_SYSCALL: i32 = -2;
const WRITE_ERROR: i32 = -3;

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

fn exit(code: u32) -> i32 {
    println!("EXIT WITH CODE: {}", code);
    0
}

fn write(buf: &[u8]) -> i32 {
    match TBW.borrow_mut().write(buf) {
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
