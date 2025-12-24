pub fn page_fault_handler(ctx: &mut InterruptContext) {
    let us_bit = ctx.errcode & (1 << 2) != 0;

    if !us_bit {
        unhandled_panic(ctx);
    }

    enum UserErr {
        NPE,
        SOE,
        UB,
        GuardPage,
    }

    let user_err = match ctx.cr2 {
        0..0x7000 => UserErr::NPE,
        0x80000..0x402000 => UserErr::SOE,
        0x402000..0x800000 => UserErr::GuardPage,
        _ => UserErr::UB,
    };

    let handler = match user_err {
        UserErr::NPE => &USERSPACE_NPE_HANDLER,
        UserErr::SOE => &USERSPACE_SOE_HANDLER,
        UserErr::UB => &USERSPACE_UB_HANDLER,
        UserErr::GuardPage => &USERSPACE_STACK_EXPAND_HANDLER,
    }
    .load(Ordering::Relaxed);

    unsafe {
        (mem::transmute::<_, fn(&mut InterruptContext)>(handler))(ctx);
    }
}

    fn reload_process() {
        paging::disable_paging();
        paging::init_user_paging();
        paging::enable_paging();
        jump_to_userspace(user_main, 0x800_000 as _);
    }

    interrupts::register_handler(0xe, interrupts::page_fault_handler);
    interrupts::register_userspace_npe_handler(|_| {
        unsafe {
            writeln!(*TBW.load(Ordering::Relaxed), "NPE").unwrap();
        }
        reload_process()
    });
    interrupts::register_userspace_soe_handler(|_| {
        unsafe {
            writeln!(*TBW.load(Ordering::Relaxed), "SOE").unwrap();
        }
        reload_process()
    });
    interrupts::register_userspace_ub_handler(|_| {
        unsafe {
            writeln!(*TBW.load(Ordering::Relaxed), "UB").unwrap();
        }
        reload_process()
    });
    interrupts::register_userspace_stack_endpand_handler(|ctx: &mut InterruptContext| {
        disable_paging();
        enable_user_pages(ctx.cr2 as *mut u8);
        unsafe {
            writeln!(*TBW.load(Ordering::Relaxed), "{}", ctx.cr2).unwrap();
        }
        enable_paging();
    });

    jump_to_userspace(user_main, 0x800_000 as _);
