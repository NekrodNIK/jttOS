const ports = @import("ports.zig");

const WIDTH = 80;
const HEIGHT = 25;
const SIZE = WIDTH * HEIGHT;

const Symbol = packed struct {
    code: u8,
    fg_color: Color,
    bg_color: Color,
};

const Color = enum(u4) {
    black = 0x0,
    blue = 0x1,
    green = 0x2,
    cyan = 0x3,
    red = 0x4,
    magenta = 0x5,
    brown = 0x6,
    light_gray = 0x7,
    dark_gray = 0x8,
    light_blue = 0x9,
    light_green = 0xa,
    light_cyan = 0xb,
    light_red = 0xc,
    light_magenta = 0xd,
    yellow = 0xe,
    white = 0xf,
};

const Position = struct { x: u16, y: u16 };

const buf_flat = @as(*[SIZE]Symbol, @ptrFromInt(0xb8000));
const buf = @as(*[HEIGHT][WIDTH]Symbol, @ptrCast(buf_flat));

var pos: Position = .{ .x = 0, .y = 0 };
var bg_color = Color.black;
var fg_color = Color.white;

pub fn setFgColor(color: Color) void {
    fg_color = color;
}

pub fn setBgColor(color: Color) void {
    bg_color = color;
}

pub fn print(str: []const u8) void {
    for (str) |s| {
        if (s == '\n') {
            pos.x = 0;
            pos.y = (pos.y + 1) % HEIGHT;
            continue;
        }

        buf[pos.y][pos.x] = .{
            .code = s,
            .fg_color = fg_color,
            .bg_color = bg_color,
        };

        pos.x += 1;
        pos.y += pos.x / WIDTH;
        pos.x %= WIDTH;
        pos.y %= HEIGHT;
    }
}

pub fn clear() void {
    @memset(buf_flat, Symbol{
        .code = ' ',
        .fg_color = fg_color,
        .bg_color = bg_color,
    });
}

pub fn disable_cursor() void {
    ports.outb(0x3D4, 0x0A);
    ports.outb(0x3D5, 0x20);
}
