const std = @import("std");

pub fn build(b: *std.Build) !void {
    var disabled_features = std.Target.Cpu.Feature.Set.empty;
    var enabled_features = std.Target.Cpu.Feature.Set.empty;

    disabled_features.addFeature(@intFromEnum(std.Target.x86.Feature.mmx));
    disabled_features.addFeature(@intFromEnum(std.Target.x86.Feature.sse));
    disabled_features.addFeature(@intFromEnum(std.Target.x86.Feature.sse2));
    disabled_features.addFeature(@intFromEnum(std.Target.x86.Feature.avx));
    disabled_features.addFeature(@intFromEnum(std.Target.x86.Feature.avx2));
    enabled_features.addFeature(@intFromEnum(std.Target.x86.Feature.soft_float));

    const target = b.resolveTargetQuery(.{
        .cpu_arch = .x86,
        .os_tag = .freestanding,
        .cpu_features_sub = disabled_features,
        .cpu_features_add = enabled_features,
    });

    const exe = b.addExecutable(.{
        .name = "kernel.elf",
        .root_source_file = b.path("src/main.zig"),
        .target = target,
        .optimize = b.standardOptimizeOption(.{}),
    });

    addNasmFiles(exe, &.{ "src/entry.nasm", "src/multiboot/header.nasm" });
    exe.setLinkerScript(b.path("src/linker.ld"));
    b.installArtifact(exe);

    const make_iso_step = b.step("iso", "Create a bootable ISO image");
    make_iso_step.makeFn = makeISO;
    make_iso_step.dependOn(b.getInstallStep());

    const run_step = b.step("run", "Run with qemu emulator");
    run_step.makeFn = run;
    run_step.dependOn(make_iso_step);
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

    try cwd.copyFile("grub.cfg", iso_dir, "boot/grub/grub.cfg", .{});
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
        "-boot",
        "order=d",
        "-cdrom",
        "zig-out/os.iso",
    }, std.heap.page_allocator);

    try qemu.spawn();
    const term = try qemu.wait();
    try std.testing.expectEqual(term, std.process.Child.Term{ .Exited = 0 });
}
