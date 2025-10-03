use core::{arch::x86::__cpuid, mem::MaybeUninit, slice};

const GET_VENDOR_ID: u32 = 0;

pub struct Vendor([u8; 12]);

impl Vendor {
    pub fn as_str(&self) -> &str {
        unsafe { str::from_utf8_unchecked(&self.0) }
    }
}

pub fn get_vendor_id() -> Vendor {
    let result = unsafe { __cpuid(GET_VENDOR_ID) };

    let mut vendor = [0u8; 12];

    vendor[0..=3].copy_from_slice(&result.ebx.to_le_bytes());
    vendor[4..=7].copy_from_slice(&result.edx.to_le_bytes());
    vendor[8..=11].copy_from_slice(&result.ecx.to_le_bytes());

    Vendor(vendor)
}
