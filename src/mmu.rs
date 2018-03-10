#![no_std]

extern crate types;

use types::*;

pub const FL_IF: uint = 0x00000200;

pub const NSEGS: usize = 6;

#[repr(C)]
pub struct TaskState {
    link: uint,
    esp0: uint,
    ss0: ushort,
    padding1: ushort,
    esp1: *mut uint,
    ss1: ushort,
    padding2: ushort,
    esp2: *mut uint,
    ss2: ushort,
    padding3: ushort,
    cr3: *mut c_void,
    eip: *mut uint,
    eflags: uint,
    eax: uint,
    ecx: uint,
    edx: uint,
    ebx: uint,
    esp: *mut uint,
    ebp: *mut uint,
    esi: uint,
    edi: uint,
    es: ushort,
    padding4: ushort,
    cs: ushort,
    padding5: ushort,
    ss: ushort,
    padding6: ushort,
    ds: ushort,
    padding7: ushort,
    fs: ushort,
    padding8: ushort,
    gs: ushort,
    padding9: ushort,
    ltd: ushort,
    padding10: ushort,
    t: ushort,
    iomb: ushort,
}

