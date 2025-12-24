use crate::main;
use core::arch::{asm, naked_asm};
use core::panic::PanicInfo;
use core::{fmt, slice};
use utils::io::Write;

pub const SYSCALL_EXIT: u32 = 0x1;
pub const SYSCALL_WRITE: u32 = 0x4;

#[macro_export]
macro_rules! syscall {
    ($x:expr) => {
        unsafe {
            core::arch::asm!(
                "int 0x80",
                in("eax") $x as u32,
                options(nostack)
            )
        }
    };
    ($x:expr, $y:expr) => {
        unsafe {
            core::arch::asm!(
                "int 0x80",
                in("eax") $x as u32,
                in("ebx") $y as u32,
                options(nostack)
            )
        }
    };
    ($x:expr, $y:expr, $z:expr) => {
        unsafe {
            core::arch::asm!(
                "int 0x80",
                in("eax") $x as u32,
                in("ebx") $y as u32,
                in("ecx") $z as u32,
                options(nostack)
            )
        }
    };
    ($x:expr, $y:expr, $z:expr, $w:expr) => {
        unsafe {
            core::arch::asm!(
                "int 0x80",
                in("eax") $x as u32,
                in("ebx") $y as u32,
                in("ecx") $z as u32,
                in("edx") $w as u32,
                options(nostack)
            )
        }
    };
    ($x:expr, $y:expr, $z:expr, $w:expr, $u:expr) => {
        unsafe {
            core::arch::asm!(
                "int 0x80",
                in("eax") $x as u32,
                in("ebx") $y as u32,
                in("ecx") $z as u32,
                in("edx") $w as u32,
                in("esi") $u as u32,
                options(nostack)
            )
        }
    };
    ($x:expr, $y:expr, $z:expr, $w:expr, $u:expr, $v:expr) => {
        unsafe {
            core::arch::asm!(
                "int 0x80",
                in("eax") $x as u32,
                in("ebx") $y as u32,
                in("ecx") $z as u32,
                in("edx") $w as u32,
                in("esi") $u as u32,
                in("edi") $v as u32,
                options(nostack)
            )
        }
    };

    ($x:expr, $y:expr, $z:expr, $w:expr, $u:expr, $v:expr, $s:expr) => {
        unsafe {
            core::arch::asm!(
                "int 0x80",
                in("eax") $x as u32,
                in("ebx") $y as u32,
                in("ecx") $z as u32,
                in("edx") $w as u32,
                in("esi") $u as u32,
                in("edi") $v as u32,
                in("ebp") $s as u32,
                options(nostack)
            )
        }
    };
}
pub(crate) use syscall;

#[inline(always)]
pub fn exit(code: u32) -> ! {
    syscall!(SYSCALL_EXIT, code);
    loop {}
}

#[inline(always)]
pub fn write(buffer: &[u8]) {
    syscall!(SYSCALL_WRITE, buffer.as_ptr(), buffer.len());
}

pub struct Writer;
impl Write for Writer {
    fn write(&mut self, buffer: &[u8]) -> utils::io::Result<usize> {
        write(buffer);
        Ok(buffer.len())
    }
    fn flush(&mut self) -> utils::io::Result<()> {
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        use $crate::stdlib::Writer;
        use utils::io::Write;
        write!(Writer, "{}", format_args!($($arg)*)).unwrap()
    }};
}

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {
        print!($($arg)*)
    };
}

#[unsafe(naked)]
#[unsafe(no_mangle)]
extern "C" fn _start() {
    naked_asm!("push eax", "push ecx", "call {}", sym start);
}

extern "C" fn start(argc: u32, argv: *const *const u8) -> ! {
    let args = unsafe { slice::from_raw_parts(argv as _, argc as _) };
    main(args);
    exit(0);
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    exit(2);
}
