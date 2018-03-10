#![no_std]

extern crate mmu;
extern crate types;

use mmu::{NSEGS, TaskState};
use types::*;

#[repr(C)]
pub struct Context {
    pub edi: uint,
    pub esi: uint,
    pub ebx: uint,
    pub ebp: uint,
    pub eip: uint,
}

pub struct Proc;

#[repr(C)]
pub struct SegDesc {
    blob: [u8; 8],
}

#[repr(C)]
pub struct CPU {
    pub apicid: uchar,
    pub scheduler: *const Context,
    pub ts: TaskState,
    pub gdt: [SegDesc; NSEGS],
    pub started: uint,
    pub ncli: int,
    pub intena: int,
    pub prc: *const Proc,
}
