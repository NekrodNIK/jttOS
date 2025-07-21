pub const TagHeader = extern struct {
    type: u32,
    size: u32,
};

pub const TagIterator = struct {
    ptr: [*]u8,

    pub fn next(self: *TagIterator) ?*TagHeader {
        const header: *TagHeader = @ptrCast(self.ptr);
        if (header.type == 0) return null;

        self.stepHeader();
        return @ptrCast(self.ptr);
    }

    fn stepHeader(self: *TagIterator) void {
        const header: *TagHeader = @ptrCast(self.ptr);
        self.ptr += (header.size + 7) & ~7;
    }
};
