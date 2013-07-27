# Grease Bench

A benchmark collection tool written in Rust using cgroups.

## Current Status

Currently it only collects memory usage statistics, as fast as it can, outputs to a hardcoded
filename and drops privileges to a hardcoded uid.

It will also only work on Linux x86-64 due to it's use of the syscall kernel interface.

The code is quite hacked-together, not much time has been spent on refactoring or even writing many
comments.

Many of the routines can be sped up, very little time has been spent optimizing the code. The IO in
particular is implemented somewhat naively and could probably be implemented better, likely with
some buffering.

## Interesting facts

This tool doesn't use any of the Rust runtime. This means all the io is implemented directly as
kernel syscalls. This means that this code is a good showcase for "runtimeless" Rust code and using
some inline asm.
