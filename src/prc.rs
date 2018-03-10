#![no_std]

extern crate mmu;
extern crate types;

use mmu::NSEGS;
use types::*;

pub struct Context;
pub struct TaskState;
pub struct Proc;
pub struct SegDesc;

#[repr(C)]
pub struct CPU<'a> {
    pub apicid: uchar,
    pub context: *const Context,
    pub ts: TaskState,
    pub gdt: &'a [SegDesc; NSEGS],
    pub started: uint,
    pub ncli: int,
    pub intena: int,
    pub prc: *const Proc,
}
