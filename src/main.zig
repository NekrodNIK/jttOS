const std = @import("std");
const vga = @import("vga.zig");

const LOGO =
    \\   _ _   _    ___  ____
    \\  (_) |_| |_ / _ \/ ___| 
    \\  | | __| __| | | \___ \ 
    \\  | | |_| |_| |_| |___) |
    \\  / |\__|\__|\___/|____/ 
    \\|__/
;

export fn kmain() callconv(.C) void {
    vga.disable_cursor();
    vga.setFgColor(.white);
    vga.setBgColor(.blue);

    vga.clear();
    vga.print(LOGO);
}
