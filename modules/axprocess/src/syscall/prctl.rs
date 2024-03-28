pub const ARCH_SET_GS: i32 = 0x1001;
pub const ARCH_SET_FS: i32 = 0x1002;
pub const ARCH_GET_FS: i32 = 0x1003;
pub const ARCH_GET_GS: i32 = 0x1004;

pub fn sys_arch_prctl(code: i32, addr: u64) -> isize {
    #[cfg(target_arch = "x86_64")]
    {
        match code {
            ARCH_SET_FS => {
                unsafe { axhal::arch::write_thread_local_storage_register(addr as usize); }
            },
            _ => todo!(),
        }
    }

    0
}