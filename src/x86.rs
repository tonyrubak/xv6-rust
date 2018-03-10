#![no_std]
#![feature(asm)]

extern crate types;

use types::*;

#[inline]
pub fn readeflags() -> uint {
    let eflags: uint;

    unsafe { asm!("pushfl; popl $0" : "=r" (eflags) ::: "volatile") };

    eflags
}

#[inline]
pub fn cli() {
    unsafe { asm!("cli" :::: "volatile") };
}

#[inline]
pub fn sti() {
    unsafe { asm!("sti" :::: "volatile") };
}
