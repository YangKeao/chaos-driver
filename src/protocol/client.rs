use anyhow::Result;
use nix::{
    fcntl::{open, OFlag},
    ioctl_none, ioctl_write_int, ioctl_write_ptr,
    sys::stat::Mode,
};

use std::{ffi::c_void, os::raw::c_int};

use super::injection::*;

const CHAOS_IOCTL_MAGIC: u8 = 0xC1;
pub struct Client {
    fd: c_int,
}

ioctl_none!(chaos_driver_get_version, CHAOS_IOCTL_MAGIC, 0);

#[repr(C, packed)]
pub struct ChaosInjection {
    matcher_type: u32,
    matcher_arg: *const c_void,
    matcher_arg_size: usize,

    injector_type: u32,
    injector_arg: *const c_void,
    injector_arg_size: usize,
}

const MATCHER_TYPE_FS_SYSCALL: u32 = 0;
const MATCHER_TYPE_BLK_IO: u32 = 1;
const INJECTOR_TYPE_DELAY: u32 = 0;

ioctl_write_ptr!(
    chaos_driver_chaos_inject,
    CHAOS_IOCTL_MAGIC,
    1,
    ChaosInjection
);

ioctl_write_int!(chaos_driver_chaos_recover, CHAOS_IOCTL_MAGIC, 2);

impl Client {
    pub fn build() -> Result<Self> {
        let fd = open("/dev/chaos", OFlag::empty(), Mode::empty())?;
        return Ok(Client { fd });
    }

    pub fn get_version(&self) -> Result<i32> {
        return Ok(unsafe { chaos_driver_get_version(self.fd)? });
    }

    pub fn inject(&self, injection: Injection) -> Result<i32> {
        match injection.matcher {
            Matcher::FsSyscall(fs_syscall) => {
                let matcher_type = MATCHER_TYPE_FS_SYSCALL;
                let matcher_arg = &fs_syscall.into() as *const RawFsSyscall as *const c_void;
                let matcher_arg_size = std::mem::size_of::<RawFsSyscall>();

                match injection.injector {
                    Injector::Delay(delay) => {
                        let injector_type = INJECTOR_TYPE_DELAY;
                        let injector_arg = &delay.into() as *const RawDelay as *const c_void;
                        let injector_arg_size = std::mem::size_of::<RawDelay>();

                        let raw_injection = ChaosInjection {
                            matcher_type,
                            matcher_arg,
                            matcher_arg_size,

                            injector_type,
                            injector_arg,
                            injector_arg_size,
                        };

                        Ok(unsafe { chaos_driver_chaos_inject(self.fd, &raw_injection)? })
                    }
                }
            }
            Matcher::BlkIo(blk_io) => {
                let matcher_type = MATCHER_TYPE_BLK_IO;
                let matcher_arg = &blk_io.into() as *const RawBlkIo as *const c_void;
                let matcher_arg_size = std::mem::size_of::<RawBlkIo>();

                match injection.injector {
                    Injector::Delay(delay) => {
                        let injector_type = INJECTOR_TYPE_DELAY;
                        let injector_arg = &delay.into() as *const RawDelay as *const c_void;
                        let injector_arg_size = std::mem::size_of::<RawDelay>();

                        let raw_injection = ChaosInjection {
                            matcher_type,
                            matcher_arg,
                            matcher_arg_size,

                            injector_type,
                            injector_arg,
                            injector_arg_size,
                        };

                        Ok(unsafe { chaos_driver_chaos_inject(self.fd, &raw_injection)? })
                    }
                }
            }
        }
    }

    pub fn recover(&self, id: libc::c_ulong) -> Result<i32> {
        Ok(unsafe { chaos_driver_chaos_recover(self.fd, id)? })
    }
}
