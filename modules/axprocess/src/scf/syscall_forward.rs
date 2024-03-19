use core::slice::{from_raw_parts, from_raw_parts_mut};
use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use alloc::{vec::Vec, vec};
use spin::Mutex;

use super::allocator::SyscallDataBuffer;
use super::queue::{ScfRequestToken, SyscallQueueBuffer};
use axmem::{UserInPtr, UserOutPtr};

use crate::syscall::Sysno;

const CHUNK_SIZE: usize = 256;

pub struct SyscallCondVar {
    ok: AtomicBool,
    ret_val: AtomicU64,
}

impl SyscallCondVar {
    pub fn new() -> Self {
        Self {
            ok: AtomicBool::new(false),
            ret_val: AtomicU64::new(0),
        }
    }

    pub fn signal(&self, ret_val: u64) {
        self.ret_val.store(ret_val, Ordering::Release);
        self.ok.store(true, Ordering::Release);
    }

    pub fn wait(&self) -> u64 {
        while !self.ok.load(Ordering::Acquire) {
            axtask::yield_now();
        }
        self.ret_val.load(Ordering::Acquire)
    }
}

/// Forwarded syscall args, does not contains syscall number.
#[repr(C)]
#[derive(Debug)]
struct SyscallArgs {
    args: [u64; 6],
}

fn send_request(opcode: Sysno, args_offset: u64, token: ScfRequestToken) {
    while !SyscallQueueBuffer::get().send(opcode, args_offset, token) {
        axtask::yield_now();
    }
    super::notify();
}

lazy_static::lazy_static! {
    static ref SCF_SPECIAL_MUST_MMAP_BUFFER: Mutex<Vec<(usize, usize, usize)>> = Mutex::new(vec![]);
}

pub fn scf_special_must_mmap_buffer_push(hpa: usize, va: usize, size: usize) {
    SCF_SPECIAL_MUST_MMAP_BUFFER.lock().push((hpa, va, size));
}

pub fn scf_special_must_mmap_buffer_check() {
    while let Some((hpa, va, size)) = SCF_SPECIAL_MUST_MMAP_BUFFER.lock().pop() {
        let ret = scf_special_must_mmap(hpa, va, size);
        if ret != 0 {
            warn!("scf_special_must_mmap failed, ret = {}, hpa = {}, va = {}, size = {}", ret, hpa, va, size);
        }
    }
}

// to notify the shadow process to mmap a range of memory
pub fn scf_special_must_mmap(hpa: usize, va: usize, size: usize) -> isize {
    // it's not standard and it's not a real syscall, so we use a fake syscall number,
    // we use `keyctl` because it's very unlikely to be used
    static SYSNO_SPECIAL_MUST_MMAP: Sysno = Sysno::keyctl;

    if size > u32::max_value() as usize {
        panic!("scf_special_must_mmap size too large");
    }

    debug!("scf_special_must_mmap hpa {:#x} va {:#x}", hpa, va);
    let pool = SyscallDataBuffer::get();
    let args = pool.alloc(SyscallArgs {
        args: [hpa as u64, va as u64, size as u64, 0, 0, 0],
    });
    let cond = SyscallCondVar::new();
    send_request(
        SYSNO_SPECIAL_MUST_MMAP,
        pool.offset_of(args),
        ScfRequestToken::from(&cond),
    );
    let ret = cond.wait();
    unsafe {
        pool.dealloc(args);
    }
    ret as _
}

pub fn scf_write(fd: usize, buf: UserInPtr<u8>, len: usize) -> isize {
    scf_special_must_mmap_buffer_check();
    debug!("scf write_direct fd {} len {:#x}", fd, len);
    debug!("ptr: {:#x}", buf.as_ptr() as usize);

    let pool = SyscallDataBuffer::get();
    let chunk_ptr = unsafe { pool.alloc_array_uninit::<u8>(len) };
    buf.read_buf(unsafe { from_raw_parts_mut(chunk_ptr as _, len) });

    unsafe {
        for ele in from_raw_parts_mut(chunk_ptr as _, len) {
            debug!("{:#x}", *ele);
        }
        pool.dealloc(chunk_ptr);
    }

    let pool = SyscallDataBuffer::get();
    let args = pool.alloc(SyscallArgs {
        args: [fd as u64, buf.as_ptr() as u64, len as u64, 0, 0, 0],
    });
    let cond = SyscallCondVar::new();
    send_request(
        Sysno::writev, // use writev as a temporary placeholder
        pool.offset_of(args),
        ScfRequestToken::from(&cond),
    );
    let ret = cond.wait();
    unsafe {
        pool.dealloc(args);
    }
    ret as _
}

pub fn scf_write_indirect(fd: usize, buf: UserInPtr<u8>, len: usize) -> isize {
    scf_special_must_mmap_buffer_check();
    debug!("scf write fd {} len {:#x}", fd, len);
    assert!(len < CHUNK_SIZE);
    let pool = SyscallDataBuffer::get();
    let chunk_ptr = unsafe { pool.alloc_array_uninit::<u8>(len) };
    buf.read_buf(unsafe { from_raw_parts_mut(chunk_ptr as _, len) });
    let args = pool.alloc(SyscallArgs {
        args: [fd as u64, pool.offset_of(chunk_ptr), len as u64, 0, 0, 0],
    });
    let cond = SyscallCondVar::new();
    send_request(
        Sysno::write,
        pool.offset_of(args),
        ScfRequestToken::from(&cond),
    );
    let ret = cond.wait();
    unsafe {
        pool.dealloc(chunk_ptr);
        pool.dealloc(args);
    }
    ret as _
}

pub fn scf_read(fd: usize, mut buf: UserOutPtr<u8>, len: usize) -> isize {
    scf_special_must_mmap_buffer_check();
    assert!(len < CHUNK_SIZE);
    let pool = SyscallDataBuffer::get();
    let chunk_ptr = unsafe { pool.alloc_array_uninit::<u8>(len) };
    let args = pool.alloc(SyscallArgs {
        args: [fd as u64, pool.offset_of(chunk_ptr), len as u64, 0, 0, 0],
    });
    let cond = SyscallCondVar::new();
    send_request(
        Sysno::read,
        pool.offset_of(args),
        ScfRequestToken::from(&cond),
    );
    let ret = cond.wait();
    unsafe {
        buf.write_buf(from_raw_parts(chunk_ptr as _, len));
        pool.dealloc(chunk_ptr);
        pool.dealloc(args);
    }
    ret as _
}
