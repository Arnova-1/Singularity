## 2026-04-29
- Freestanding binary/executable is a binary or an executable that can run without underlying OS.
- `no_std` is used so that the compiler not to use standard library such as libc. This is important step to produce a freestanding binary, since it tells the compiler to stop assuming that the OS environment already exist.
- `stdout` is provided by the OS, therefore `println` wont work.
- Every rust program must have exactly one `#[panic_handler]`, which usually provided by standard library, but on `no_std` env, it is required to define one manually. Hence the error `error: `#[panic_handler]` function required, but not found`, the compiler(`rustc`) is the enforcer.
- `!` is 'never' type, which is a type that can never have any value.
- `never` type is used heavily in freestanding environment because there wouldn't be any recovery mechanics that make fallback meaningful. Most of the error produced at this stage is non recoverable anyway.
- `panic = "abort"` disables unwind, a panic handler provided by OS-specific library.

## 2026-04-30
- In rust and C, the real entry point is a runtime function called `start`.
- Runtime is essentially a code that runs behind the scene to support the program, before, during, or after code execution.
- After looking at `rt.rs`, which is entrypoint of rust runtime, its task include initializing OS-specific stuff, setup stack guard to prevent stack overflow, stack overflow detection, setup main thread info, and store command line arguments.
- The runtime then called `main` which is usual entrypoint at userspace, which was wrapped with unwinding support: `let res = panic::catch_unwind(|| { main() });`.
- `lang_start` is essentially just a wrapper that pull all those standard std modules (`sys`, `thread`, `panic`, etc.) and setup all the runtime features before calling `main()`
- Since there's no std, rust runtime can't call those modules and the compiler has no idea how to setup an entry point, hence the error: `error: using `fn main` requires standard library`.
- In freestanding environment, we essentially need to build our own runtime from scratch instead of relying on `std`'s runtime, which is OS dependent.

### End of section 1

## 2026-04-30
- The ROM chip (commonly called BIOS/UEFI chip) is a small chip that's physically soldered or socketed onto the motherboard.
- When we turn our computer, the CPU began it's first instruction by pointing directly into the address of the ROM chip to execute firmware code. 
- That code performs a power-on self-test (POST), which detect if any hardware component are non functional and set initial state of CPU and hardware.
- Rough order:
  - -> Power Button
    - -> PSU stabilizes power
      - -> "Power Good" signal sent
        - -> Clock generator starts
          - -> CPU reads from ROM/BIOS chip
            - -> POST Begin
- After POST, the firmware start looking for bootable disk. If it finds one, the control is transferred to its bootloader (like GRUB, rEFInd, Windows Boot Manager)
- The bootloader then run set of routine consist of determining the location of the kernel image on the dist and load it into memory. It also needs to switch the CPU from the 16-bit real mode first to the 32-bit protected mode, and then to the 64-bit long mode.
- The bootloader also query certain information from the firmware and pass it to the kernel.

## 2026-05-01
- `core` is tiny subset of `std` that works anywhere, even with no operating system.
- Our code
  - -> `std` (standard library - files, io, networking, thread, etc.)
    - -> `core` (the absolute basics - Result, Option, iterators, math, etc.)
- Rust automatically link to `core` because even with `no_std` we still need basic features like Result, Option, math, etc.
- The compiler shipped `core` as already built binary that's compiled for common target like `x86_64-unknown-gnu`. Hence why it isn't compatible with our custom target, we would need to compile it ourselves.
