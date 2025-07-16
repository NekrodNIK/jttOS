// https://wiki.osdev.org/Inline_Assembly/Examples#I.2FO_access

pub fn outb(port: u16, value: u8) void {
    asm volatile ("outb %[value],%[port]"
        :
        : [value] "{al}" (value),
          [port] "n{dx}" (port),
    );
}

pub fn outw(port: u16, value: u16) void {
    asm volatile ("outw %[value],%[port]"
        :
        : [value] "{ax}" (value),
          [port] "n{dx}" (port),
    );
}

pub fn inb(port: u16) u8 {
    return asm volatile ("inb %[port], %[ret]"
        : [ret] "={al}" (-> u8),
        : [port] "n{dx}" (port),
    );
}

pub fn inw(port: u16) u16 {
    return asm volatile ("inw %[port], %[ret]"
        : [ret] "={ax}" (-> u16),
        : [port] "n{dx}" (port),
    );
}

pub fn io_wait() void {
    outb(0x80, 0);
}
