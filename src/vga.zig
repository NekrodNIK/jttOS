const WIDTH = 80;
const HEIGHT = 25;
const SIZE = WIDTH * HEIGHT;

const Symbol = packed struct {
    code: u8,
    fg_color: Color,
    bg_color: Color,
};

pub const Color = packed struct {
    blue: bool = false,
    green: bool = false,
    red: bool = false,
    bright: bool = false,
};

var buffer = @as([*]volatile Symbol, @ptrFromInt(0xb8000))[0..SIZE];
var position: u16 = 0;

pub fn print(str: []const u8) void {
    for (str) |s| {
        buffer[position] = .{
            .code = s,
            .fg_color = .{},
            .bg_color = .{ .green = true },
        };

        position = (position + 1) % SIZE;
    }
}
