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
