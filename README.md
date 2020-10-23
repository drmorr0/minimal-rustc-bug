This is a small example of a possible compiler bug with rustc's AVR support.

Here we have an interrupt handler at `__vector_15` that calls another function 
(`Executor::push`); if the function called by the interrupt handler is not inlined, 
the generated assembly does not protect registers used by the function by pushing
them onto the stack.  Thus, we get a data corruption when that function modifies
those registers.

In this code, we have a simple loop that writes `0xBE -> R26` and `0xEF -> R27`;
when the interrupt code fires, these values get overwritten.  You can see the 
results of the compiled code in the `avr.objdump` file.  This was compiled using
`nightly-2020-10-23-x86_64-unknown-linux-gnu` toolchain.

This is a simplified example of a problem I discovered in [drmorr0/rustybot](https://github.com/drmorr0/rustybot).
It's entirely possible I'm doing something wrong (or at least not recommended) but I
found the behaviour surprising.

rust-lang bug report: https://github.com/rust-lang/rust/issues/78260
