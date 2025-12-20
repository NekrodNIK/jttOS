use crate::x86_utils::{EFlags, cli, sti};

pub fn wrap(f: impl FnOnce()) {
    let flag = EFlags::read().contains(EFlags::IF);
    if flag {
        unsafe { cli() }
    }
    f();
    if flag {
        unsafe { sti() }
    }
}
