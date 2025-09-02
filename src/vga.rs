use core::ops;
use core::slice;

pub struct Vga<'a> {
    ptr: &'a mut [u8],
}

impl<'a> Vga<'a> {
    pub fn new() -> Self {
        return Self {
            ptr: unsafe { slice::from_raw_parts_mut(0xb8000 as *mut u8, 80 * 25) },
        };
    }

    pub fn clear(&mut self) {
        self.ptr.fill(0);
    }
}

impl<'a> ops::Index<usize> for Vga<'a> {
    type Output = u8;
    fn index(&self, index: usize) -> &Self::Output {
        &self.ptr[index]
    }
}

impl<'a> ops::IndexMut<usize> for Vga<'a> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.ptr[index]
    }
}

// impl ops::IndexMut<u8> for Vga<'_> {
//     fn index_mut(&mut self, index: u8) -> &mut Self::Output {

//     }
// }

// const ports = @import("ports.zig");

// const WIDTH = 80;
// const HEIGHT = 25;
// const SIZE = WIDTH * HEIGHT;

// const Symbol = packed struct {
//     code: u8,
//     fg_color: Color,
//     bg_color: Color,
// };

// const Color = enum(u4) {
//     black = 0x0,
//     blue = 0x1,
//     green = 0x2,
//     cyan = 0x3,
//     red = 0x4,
//     magenta = 0x5,
//     brown = 0x6,
//     light_gray = 0x7,
//     dark_gray = 0x8,
//     light_blue = 0x9,
//     light_green = 0xa,
//     light_cyan = 0xb,
//     light_red = 0xc,
//     light_magenta = 0xd,
//     yellow = 0xe,
//     white = 0xf,
// };

// const Position = struct { x: u16, y: u16 };

// const buf_flat = @as(*[SIZE]Symbol, @ptrFromInt(0xb8000));
// const buf = @as(*[HEIGHT][WIDTH]Symbol, @ptrCast(buf_flat));

// var pos: Position = .{ .x = 0, .y = 0 };
// var bg_color = Color.black;
// var fg_color = Color.white;

// pub fn setFgColor(color: Color) void {
//     fg_color = color;
// }

// pub fn setBgColor(color: Color) void {
//     bg_color = color;
// }

// pub fn print(str: []const u8) void {
//     for (str) |s| {
//         if (s == '\n') {
//             pos.x = 0;
//             pos.y = (pos.y + 1) % HEIGHT;
//             continue;
//         }

//         buf[pos.y][pos.x] = .{
//             .code = s,
//             .fg_color = fg_color,
//             .bg_color = bg_color,
//         };

//         pos.x += 1;
//         pos.y += pos.x / WIDTH;
//         pos.x %= WIDTH;
//         pos.y %= HEIGHT;
//     }
// }

// pub fn disable_cursor() void {
//     ports.outb(0x3D4, 0x0A);
//     ports.outb(0x3D5, 0x20);
// }
