const std = @import("std");
const vga = @import("vga.zig");

export fn kmain() callconv(.C) void {
    vga.setFgColor(.white);
    vga.setBgColor(.blue);

    vga.clear();
    vga.print("Hello!\n");
}
