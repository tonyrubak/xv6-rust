#![no_std]
#![feature(lang_items)]
#![feature(core_intrinsics)]
#![feature(panic_implementation)]

#[no_mangle]
#[lang = "eh_personality"]
pub extern fn eh_personality() {}

extern "C" {
    fn mycpu() -> *mut CPU;
}

extern crate prc;
extern crate memlayout;
extern crate mmu;
extern crate types;
extern crate x86;

use core::intrinsics;
use core::panic::PanicInfo;
use core::ptr;
use memlayout::*;
use mmu::*;
use prc::*;
use types::*;
use x86::*;

#[no_mangle]
#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    unsafe { intrinsics::abort() }
}

#[repr(C)]
pub struct SpinLock {
    locked: uint,

    name: *const c_char,
    cpu: *const CPU,
    pcs: [uint; 10],
}

#[no_mangle]
pub extern fn initlock(lk: *mut SpinLock, nm: *const c_char) {
    let rlk = unsafe { &mut *lk };
    rlk.name = nm;
    rlk.locked = 0;
    rlk.cpu = ptr::null();
    rlk.pcs = [0; 10];
}

#[no_mangle]
pub unsafe extern "C" fn acquire(lk: *mut SpinLock) {
    let rlk = &mut *lk;
    pushcli();
    if holding(rlk) != 0 {
        panic!("acquire");
    }

    while intrinsics::atomic_xchg(&mut rlk.locked as *mut uint, 1) != 0 { }

    core::intrinsics::atomic_fence();

    rlk.cpu = mycpu();
    getcallerpcs(&lk as *const _ as *const c_void, rlk.pcs.as_mut_ptr());
}

#[no_mangle]
pub unsafe extern fn release(lk: *mut SpinLock) {
    let rlk = &mut *lk;
    if holding(rlk) == 0 {
        panic!("release");
    }

    rlk.pcs[0] = 0;
    rlk.cpu = ptr::null();

    core::intrinsics::atomic_fence();

    intrinsics::atomic_store(&mut rlk.locked as *mut uint, 0);

    popcli();
}

#[no_mangle]
pub unsafe extern "C" fn getcallerpcs(v: *const c_void, pcs_ptr: *mut uint) {
    let mut i: usize = 0;
    let pcs = core::slice::from_raw_parts_mut(pcs_ptr, 10);

    let mut ebp = (v as *const uint).offset(-2);

    while i < 10 {
        if ebp == (0 as *const uint) || ebp < (KERNBASE as *const uint) || ebp == (0xFFFFFFFF as *const uint) {
            break;
        }
        pcs[i] = *(ebp.offset(1)) as uint;
        ebp = (*ebp) as *const uint;
        i += 1;
    }

    while i < 10 {
        pcs[i] = 0;
        i += 1;
    }
}

#[no_mangle]
pub extern fn holding(lk: &SpinLock) -> int {
    if lk.locked != 0 && lk.cpu == unsafe { mycpu() } { 1 } else { 0 }
}

#[no_mangle]
pub unsafe extern "C" fn pushcli() {
    let eflags = readeflags();
    cli();
    let cpu = &mut *mycpu();

    let ncli = cpu.started as usize + core::mem::size_of::<CPU>();

    if ncli == 0 {
        cpu.intena = (eflags & FL_IF) as i32;
    }
    cpu.ncli = cpu.ncli + 1;
}

#[no_mangle]
pub unsafe extern "C" fn popcli() {
    if readeflags() & FL_IF != 0 {
        panic!("popcli - interruptible");
    }
    let cpu = &mut *mycpu();
    cpu.ncli -= 1;

    if cpu.ncli < 0 {
        panic!("popcli");
    }
    if cpu.ncli == 0 && cpu.intena != 0 {
        sti();
    }
}
