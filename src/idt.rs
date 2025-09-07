#![allow(dead_code)]
use core::arch::asm;
use paste::paste;

pub struct Idt(pub [InterruptDescriptor; 255]);

impl Idt {
    pub fn new() -> Self {
        Self([InterruptDescriptor::default(); 255])
    }

    pub unsafe fn load(&self) {
        unsafe {
            asm!("mov eax, {ptr}",
                 "lidt [eax]",
                 ptr = in(reg) self.0.as_ptr());
        }
    }
}

#[derive(Copy, Clone, Default)]
#[repr(packed)]
pub struct InterruptDescriptor {
    offset_low: u16,
    selector: u16,
    zero: u8,
    attributes: u8,
    offset_high: u16,
}

impl InterruptDescriptor {
    pub fn set_entry(&mut self, entry: fn()) {
        let address = entry as u32;
        self.offset_low = (address & 0xffff) as u16;
        self.offset_high = (address >> 16) as u16;
        self.attributes = 0x8e;
    }
}

pub fn interrupt_handler(id: u8) {
    match id {
        _ => panic!("IRQ{}", id),
    }
}

#[repr(C)]
struct InterruptStackFrame {
    instruction_pointer: u32,
    code_segment: u32,
    flags: u32,
}

macro_rules! gen_redirector {
    ($id:literal) => {
        paste! {
            extern "x86-interrupt" fn [<redirector $id>](_: InterruptStackFrame) {
                interrupt_handler($id);
            }
        }
    };
}

gen_redirector!(0);
gen_redirector!(1);
gen_redirector!(2);
gen_redirector!(3);
gen_redirector!(4);
gen_redirector!(5);
gen_redirector!(6);
gen_redirector!(7);
gen_redirector!(8);
gen_redirector!(9);
gen_redirector!(10);
gen_redirector!(11);
gen_redirector!(12);
gen_redirector!(13);
gen_redirector!(14);
gen_redirector!(15);
gen_redirector!(16);
gen_redirector!(17);
gen_redirector!(18);
gen_redirector!(19);
gen_redirector!(20);
gen_redirector!(21);
gen_redirector!(22);
gen_redirector!(23);
gen_redirector!(24);
gen_redirector!(25);
gen_redirector!(26);
gen_redirector!(27);
gen_redirector!(28);
gen_redirector!(29);
gen_redirector!(30);
gen_redirector!(31);
gen_redirector!(32);
gen_redirector!(33);
gen_redirector!(34);
gen_redirector!(35);
gen_redirector!(36);
gen_redirector!(37);
gen_redirector!(38);
gen_redirector!(39);
gen_redirector!(40);
gen_redirector!(41);
gen_redirector!(42);
gen_redirector!(43);
gen_redirector!(44);
gen_redirector!(45);
gen_redirector!(46);
gen_redirector!(47);
gen_redirector!(48);
gen_redirector!(49);
gen_redirector!(50);
gen_redirector!(51);
gen_redirector!(52);
gen_redirector!(53);
gen_redirector!(54);
gen_redirector!(55);
gen_redirector!(56);
gen_redirector!(57);
gen_redirector!(58);
gen_redirector!(59);
gen_redirector!(60);
gen_redirector!(61);
gen_redirector!(62);
gen_redirector!(63);
gen_redirector!(64);
gen_redirector!(65);
gen_redirector!(66);
gen_redirector!(67);
gen_redirector!(68);
gen_redirector!(69);
gen_redirector!(70);
gen_redirector!(71);
gen_redirector!(72);
gen_redirector!(73);
gen_redirector!(74);
gen_redirector!(75);
gen_redirector!(76);
gen_redirector!(77);
gen_redirector!(78);
gen_redirector!(79);
gen_redirector!(80);
gen_redirector!(81);
gen_redirector!(82);
gen_redirector!(83);
gen_redirector!(84);
gen_redirector!(85);
gen_redirector!(86);
gen_redirector!(87);
gen_redirector!(88);
gen_redirector!(89);
gen_redirector!(90);
gen_redirector!(91);
gen_redirector!(92);
gen_redirector!(93);
gen_redirector!(94);
gen_redirector!(95);
gen_redirector!(96);
gen_redirector!(97);
gen_redirector!(98);
gen_redirector!(99);
gen_redirector!(100);
gen_redirector!(101);
gen_redirector!(102);
gen_redirector!(103);
gen_redirector!(104);
gen_redirector!(105);
gen_redirector!(106);
gen_redirector!(107);
gen_redirector!(108);
gen_redirector!(109);
gen_redirector!(110);
gen_redirector!(111);
gen_redirector!(112);
gen_redirector!(113);
gen_redirector!(114);
gen_redirector!(115);
gen_redirector!(116);
gen_redirector!(117);
gen_redirector!(118);
gen_redirector!(119);
gen_redirector!(120);
gen_redirector!(121);
gen_redirector!(122);
gen_redirector!(123);
gen_redirector!(124);
gen_redirector!(125);
gen_redirector!(126);
gen_redirector!(127);
gen_redirector!(128);
gen_redirector!(129);
gen_redirector!(130);
gen_redirector!(131);
gen_redirector!(132);
gen_redirector!(133);
gen_redirector!(134);
gen_redirector!(135);
gen_redirector!(136);
gen_redirector!(137);
gen_redirector!(138);
gen_redirector!(139);
gen_redirector!(140);
gen_redirector!(141);
gen_redirector!(142);
gen_redirector!(143);
gen_redirector!(144);
gen_redirector!(145);
gen_redirector!(146);
gen_redirector!(147);
gen_redirector!(148);
gen_redirector!(149);
gen_redirector!(150);
gen_redirector!(151);
gen_redirector!(152);
gen_redirector!(153);
gen_redirector!(154);
gen_redirector!(155);
gen_redirector!(156);
gen_redirector!(157);
gen_redirector!(158);
gen_redirector!(159);
gen_redirector!(160);
gen_redirector!(161);
gen_redirector!(162);
gen_redirector!(163);
gen_redirector!(164);
gen_redirector!(165);
gen_redirector!(166);
gen_redirector!(167);
gen_redirector!(168);
gen_redirector!(169);
gen_redirector!(170);
gen_redirector!(171);
gen_redirector!(172);
gen_redirector!(173);
gen_redirector!(174);
gen_redirector!(175);
gen_redirector!(176);
gen_redirector!(177);
gen_redirector!(178);
gen_redirector!(179);
gen_redirector!(180);
gen_redirector!(181);
gen_redirector!(182);
gen_redirector!(183);
gen_redirector!(184);
gen_redirector!(185);
gen_redirector!(186);
gen_redirector!(187);
gen_redirector!(188);
gen_redirector!(189);
gen_redirector!(190);
gen_redirector!(191);
gen_redirector!(192);
gen_redirector!(193);
gen_redirector!(194);
gen_redirector!(195);
gen_redirector!(196);
gen_redirector!(197);
gen_redirector!(198);
gen_redirector!(199);
gen_redirector!(200);
gen_redirector!(201);
gen_redirector!(202);
gen_redirector!(203);
gen_redirector!(204);
gen_redirector!(205);
gen_redirector!(206);
gen_redirector!(207);
gen_redirector!(208);
gen_redirector!(209);
gen_redirector!(210);
gen_redirector!(211);
gen_redirector!(212);
gen_redirector!(213);
gen_redirector!(214);
gen_redirector!(215);
gen_redirector!(216);
gen_redirector!(217);
gen_redirector!(218);
gen_redirector!(219);
gen_redirector!(220);
gen_redirector!(221);
gen_redirector!(222);
gen_redirector!(223);
gen_redirector!(224);
gen_redirector!(225);
gen_redirector!(226);
gen_redirector!(227);
gen_redirector!(228);
gen_redirector!(229);
gen_redirector!(230);
gen_redirector!(231);
gen_redirector!(232);
gen_redirector!(233);
gen_redirector!(234);
gen_redirector!(235);
gen_redirector!(236);
gen_redirector!(237);
gen_redirector!(238);
gen_redirector!(239);
gen_redirector!(240);
gen_redirector!(241);
gen_redirector!(242);
gen_redirector!(243);
gen_redirector!(244);
gen_redirector!(245);
gen_redirector!(246);
gen_redirector!(247);
gen_redirector!(248);
gen_redirector!(249);
gen_redirector!(250);
gen_redirector!(251);
gen_redirector!(252);
gen_redirector!(253);
gen_redirector!(254);
gen_redirector!(255);
