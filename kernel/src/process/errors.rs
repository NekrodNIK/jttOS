use crate::{
    interrupts::{self, InterruptContext},
    paging, println,
};

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

//
pub fn npe_handler(ctx: &mut InterruptContext) {
    println!("NPE");
}

pub fn soe_handler(ctx: &mut InterruptContext) {
    println!("SOE");
}

pub fn ub_handler(ctx: &mut InterruptContext) {
    println!("UB");
}

pub fn stack_expand_handler(ctx: &mut InterruptContext) {
    paging::disable_paging();
    paging::enable_user_pages(ctx.cr2 as *mut u8);
    paging::enable_paging();
}
