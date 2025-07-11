const std = @import("std");

pub fn build(b: *std.Build) !void {
    const target = b.resolveTargetQuery(.{
        .cpu_arch = .x86,
        .os_tag = .freestanding,
        .abi = .none,
    });

    const exe = b.addExecutable(.{
        .name = "kernel.elf",
        .root_source_file = b.path("src/main.zig"),
        .target = target,
        .optimize = .ReleaseSmall,
    });

    addNasmFiles(exe, &.{ "src/entry.nasm", "src/multiboot.nasm" });
    exe.setLinkerScript(b.path("src/linker.ld"));
    b.installArtifact(exe);

    const make_iso_step = b.step("iso", "Create a bootable ISO image");
    make_iso_step.makeFn = makeISO;
    make_iso_step.dependOn(b.getInstallStep());

    const run_step = b.step("run", "Run with qemu emulator");
    run_step.makeFn = run;
    run_step.dependOn(make_iso_step);

    // // This allows the user to pass arguments to the application in the build
    // // command itself, like this: `zig build run -- arg1 arg2 etc`
    // if (b.args) |args| {
    //     run_cmd.addArgs(args);
    // }

    // // Creates a step for unit testing. This only builds the test executable
    // // but does not run it.
    // const lib_unit_tests = b.addTest(.{
    //     .root_module = lib_mod,
    // });

    // const run_lib_unit_tests = b.addRunArtifact(lib_unit_tests);

    // const exe_unit_tests = b.addTest(.{
    //     .root_module = exe_mod,
    // });

    // const run_exe_unit_tests = b.addRunArtifact(exe_unit_tests);

    // // Similar to creating the run step earlier, this exposes a `test` step to
    // // the `zig build --help` menu, providing a way for the user to request
    // // running the unit tests.
    // const test_step = b.step("test", "Run unit tests");
    // test_step.dependOn(&run_lib_unit_tests.step);
    // test_step.dependOn(&run_exe_unit_tests.step);
}

fn addNasmFiles(compile: *std.Build.Step.Compile, files: []const []const u8) void {
    const b = compile.step.owner;

    for (files) |file| {
        std.debug.assert(!std.fs.path.isAbsolute(file));
        const out = b.fmt("{s}.o", .{std.mem.sliceTo(file, '.')});

        const nasm = b.addSystemCommand(&.{ "nasm", "-felf32" });
        const obj = nasm.addPrefixedOutputFileArg("-o", out);
        nasm.addFileArg(b.path(file));
        compile.addObjectFile(obj);
    }
}

fn makeISO(step: *std.Build.Step, opts: std.Build.Step.MakeOptions) !void {
    _ = step;
    _ = opts;
    const cwd = std.fs.cwd();

    try cwd.makePath("iso/boot/grub");
    var iso_dir = try cwd.openDir("iso", .{});

    try cwd.copyFile("src/grub.cfg", iso_dir, "boot/grub/grub.cfg", .{});
    try cwd.copyFile("zig-out/bin/kernel.elf", iso_dir, "boot/kernel.elf", .{});

    var mkrescue = std.process.Child.init(&.{
        "grub2-mkrescue",
        "-o",
        "zig-out/os.iso",
        "iso",
    }, std.heap.page_allocator);

    try mkrescue.spawn();
    const term = try mkrescue.wait();
    try std.testing.expectEqual(term, std.process.Child.Term{ .Exited = 0 });

    iso_dir.close();
    try cwd.deleteTree("iso");
}

fn run(step: *std.Build.Step, opts: std.Build.Step.MakeOptions) !void {
    _ = step;
    _ = opts;

    var qemu = std.process.Child.init(&.{
        "qemu-system-i386",
        "-cdrom",
        "zig-out/os.iso",
    }, std.heap.page_allocator);

    try qemu.spawn();
    const term = try qemu.wait();
    try std.testing.expectEqual(term, std.process.Child.Term{ .Exited = 0 });
}
