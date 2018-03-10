#![no_std]
#![feature(lang_items)]
#![feature(core_intrinsics)]

#[no_mangle]
#[lang = "eh_personality"]
pub extern fn eh_personality() {}

#[no_mangle]
#[lang = "panic_fmt"]
pub fn panic_fmt() -> ! { loop {} }

extern "C" {
    fn mycpu<'a>() -> *mut CPU<'a>;
    fn panic(fmt: *const c_char);
}

extern crate prc;
extern crate memlayout;
extern crate mmu;
extern crate types;
extern crate x86;

use core::intrinsics;
use core::ptr;
use core::sync::atomic::{AtomicBool, Ordering};
use memlayout::*;
use mmu::*;
use prc::*;
use types::*;
use x86::*;

#[repr(C)]
pub struct SpinLock<'a> {
    locked: AtomicBool,

    name: *const c_char,
    cpu: *const CPU<'a>,
    pcs: &'a mut [uint; 10],
}

#[no_mangle]
pub extern fn initlock(lk: *mut SpinLock, nm: *const c_char) {
    let rlk = unsafe { &mut *lk };
    rlk.name = nm;
    rlk.locked = AtomicBool::new(false);
    rlk.cpu = ptr::null();
}

#[no_mangle]
pub unsafe extern fn acquire(lk: *mut SpinLock) {
    let rlk = &mut *lk;
    pushcli();
    if holding(rlk) != 0 {
        panic("acquire".as_ptr() as *const c_char);
    }

    while rlk.locked.swap(true, Ordering::SeqCst) != false { }

    core::intrinsics::atomic_fence();

    rlk.cpu = mycpu();
    getcallerpcs(lk, rlk.pcs);
}

#[no_mangle]
pub unsafe extern fn release(lk: *mut SpinLock) {
    let rlk = &mut *lk;
    if holding(rlk) == 0 {
        panic("release".as_ptr() as *const c_char);
    }

    rlk.pcs[0] = 0;
    rlk.cpu = ptr::null();

    core::intrinsics::atomic_fence();

    rlk.locked.store(false, Ordering::SeqCst);

    popcli();
}

#[no_mangle]
pub unsafe extern fn getcallerpcs(v: *const SpinLock, pcs: &mut [uint]) {
    let mut ebp: *const uint;
    let mut i: usize = 0;

    ebp = (v as *const uint).offset(-2);

    while i < 10 {
        if ebp == (0 as *const uint) || ebp < (KERNBASE as *const uint) || ebp == (0xFFFFFFFF as *const uint) {
            break;
        }
        pcs[i] = *(ebp.offset(1));
        ebp = (*ebp) as *const uint;
        i += 1;
    }

    while i < 10 {
        pcs[i] = 0;
    }
}

#[no_mangle]
pub fn holding(lk: &SpinLock) -> int {
    if lk.locked.load(Ordering::SeqCst) && lk.cpu == unsafe { mycpu() } { 1 } else { 0 }
}

#[no_mangle]
pub unsafe fn pushcli() {
    let eflags = readeflags();
    cli();
    let cpu = mycpu();
    if (*cpu).ncli == 0 {
        (*cpu).intena = (eflags & FL_IF) as i32;
    }
    (*cpu).ncli += 1;
}

#[no_mangle]
pub unsafe fn popcli() {
    if readeflags() & FL_IF != 0 {
        panic("popcli - interruptible".as_ptr() as *const c_char);
    }
    let cpu = mycpu();
    (*cpu).ncli -= 1;
    if (*cpu).ncli < 0 {
        panic("popcli".as_ptr() as *const c_char);
    }
    if (*cpu).ncli == 0 && (*cpu).intena != 0 {
        sti();
    }
}
