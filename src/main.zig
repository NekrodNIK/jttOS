export fn kmain() callconv(.C) void {
    var buffer = @as([*]volatile u8, @ptrFromInt(0xB8000));
    buffer[0] = 'H';
    buffer[1] = 15;
}
