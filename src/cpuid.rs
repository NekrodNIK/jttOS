use core::arch::asm;
use core::str;

#[derive(Default)]
pub struct Cpuid {
    vendor: Option<[u8; 12]>,
}

impl Cpuid {
    pub fn get_vendor(&mut self) -> &str {
        let arr: &[u8] = match self.vendor {
            Some(ref arr) => arr,
            None => self.read_vendor(),
        };

        unsafe { str::from_utf8_unchecked(arr) }
    }

    fn read_vendor(&mut self) -> &[u8] {
        let mut regs = [0u32; 3];

        unsafe {
            asm!(
                "cpuid",
                in("eax") 0,
                lateout("ebx") regs[0],
                lateout("edx") regs[1],
                lateout("ecx") regs[2],
            )
        };

        let arr: &mut [u8] = self.vendor.insert([0u8; 12]);

        regs.iter()
            .flat_map(|x| x.to_le_bytes())
            .enumerate()
            .for_each(|(i, v)| arr[i] = v);
        arr
    }
}
