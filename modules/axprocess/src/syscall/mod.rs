mod fs;
mod prctl;

pub use syscalls::Sysno;

struct SyscallHandlerImpl;

#[crate_interface::impl_interface]
impl axhal::trap::SyscallHandler for SyscallHandlerImpl {
    fn handle_syscall(syscall_id: usize, args: [usize; 6]) -> isize {
        crate::syscall::syscall(syscall_id, args)
    }
}

#[no_mangle]
pub fn syscall(syscall_id: usize, args: [usize; 6]) -> isize {
    let ans: isize;

    let sysno = Sysno::new(syscall_id).unwrap();

    trace!(
        "[SYSCALL] {syscall_id} {} [{:#x}, {:#x}, {:#x}]",
        sysno.name(),
        args[0],
        args[1],
        args[2]
    );
    match sysno {
        Sysno::read => {
            ans = fs::sys_read(args[0], args[1].into(), args[2]);
        }
        Sysno::write => {
            ans = fs::sys_write(args[0], args[1].into(), args[2]);
        }
        Sysno::open => {
            ans = fs::sys_open(args[0].into(), args[1], args[2]);
        }
        Sysno::close => {
            ans = fs::sys_close(args[0]);
        }
        Sysno::writev => {
            ans = fs::sys_writev(args[0], args[1] as *const fs::IoVec, args[2]);
        }
        Sysno::exit => {
            axtask::exit(args[0] as i32);
        }
        Sysno::arch_prctl => {
            ans = prctl::sys_arch_prctl(args[0] as i32, args[1] as u64);
        }
        _ => {
            ans = 0;
        }
    }

    ans
}
