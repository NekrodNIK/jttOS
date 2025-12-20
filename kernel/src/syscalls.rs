use core::slice;

use crate::{
    critical_section,
    device_manager::DEVICES,
    drivers::ps2,
    interrupts::InterruptContext,
    x86_utils::{sti, tsc_sleep},
};

const INVALID_ARGS: i32 = -1;
const UNKNOWN_SYSCALL: i32 = -2;

pub fn generic_handler(ctx: &mut InterruptContext) {
    ctx.eax = match ctx.eax {
        3 => read(ctx.ebx),
        10 => get_fb_addr(),
        11 => get_fb_width(),
        12 => get_fb_height(),
        _ => UNKNOWN_SYSCALL,
    } as _
}

fn read(fd: u32) -> i32 {
    if fd != 0 {
        return INVALID_ARGS;
    }
    DEVICES.ps2keyboard.read() as i32
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
