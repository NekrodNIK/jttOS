const vga = @import("vga.zig");
const std = @import("std");

export fn kmain() callconv(.C) void {
    vga.print("Hello VGA!");
}
