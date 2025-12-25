use crate::{
    TBW,
    interrupts::{self, InterruptContext},
    paging, println,
    process::get_cur_process,
    x86_utils::{cli, sti, tsc_sleep},
};

use utils::io::Write;

pub fn pagefault_handler(ctx: &mut InterruptContext) {
    let us_bit = ctx.errcode & (1 << 2) != 0;
    debug_assert!(us_bit == (ctx.cs & 0b11 != 0));

    if !us_bit {
        interrupts::unhandled_panic(ctx);
    }

    enum UserErr {
        NPE,
        SOE,
        UB,
        GuardPage,
    }

    let user_err = match ctx.cr2 {
        0..0x200000 => UserErr::NPE,
        0x200000..0x400000 => UserErr::SOE,
        0x400000..0x800000 => UserErr::GuardPage,
        _ => UserErr::UB,
    };

    let handler = match user_err {
        UserErr::NPE => npe_handler,
        UserErr::SOE => soe_handler,
        UserErr::UB => ub_handler,
        UserErr::GuardPage => stack_expand_handler,
    };
    handler(ctx)
}

pub fn npe_handler(ctx: &mut InterruptContext) {
    let process = get_cur_process();
    writeln!(process.tbw, "NPE").unwrap();
    process.kill();
    sti();
    loop {}
}

pub fn soe_handler(ctx: &mut InterruptContext) {
    let process = get_cur_process();
    writeln!(process.tbw, "SOE").unwrap();
    process.kill();
    sti();
    loop {}
}

pub fn ub_handler(ctx: &mut InterruptContext) {
    let process = get_cur_process();
    writeln!(process.tbw, "UB").unwrap();
    process.kill();
    sti();
    loop {}
}

pub fn stack_expand_handler(ctx: &mut InterruptContext) {
    let process = get_cur_process();
    paging::disable_paging();
    paging::enable_stack_pages(process.pd, ctx.cr2);
    paging::enable_paging(process.pd);
}
