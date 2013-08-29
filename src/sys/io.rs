use sys;
use sys::err::{Error,ErrorResult, Err, Ok};

use sys::n;
use sys::{syscall1,syscall2};

use std::{cast,ptr};

pub static FILENAME_MAX : int = 260;

#[inline]
pub fn print(s: &str) -> int {
    unsafe {
        let n = raw::write(1, cast::transmute(s));
        if n < 0 {
            sys::fail(sys::err::msg(-1*n));
        }

        return n;
    }
}

#[inline]
pub fn println(s: &str) -> int {
    print(s) + print("\n")
}

#[unsafe_no_drop_flag]
pub struct File {
    priv path: ~str,
    priv flags: int,
    priv fd: int,
}

static DEFAULT_MODE : int = 438; // 0666

impl File {
    pub fn open(path: &str, mode: &str) -> ErrorResult<File> {
        if mode.len() == 0 {
            unsafe {
                return Err(cast::transmute(-1*sys::err::EINVAL))
            }
        }
        let flags = fmodeflags(mode) | raw::O_LARGEFILE;
        let fd = raw::open(path, flags, DEFAULT_MODE);
        if fd < 0{
            unsafe {
                Err(cast::transmute(fd))
            }
        } else {
            Ok(File {
                path: path.to_owned(),
                flags: flags,
                fd: fd
            })
        }
    }

    pub fn write_str(&self, s: &str) -> ErrorResult<int> {
        unsafe {
            self.write_bytes(cast::transmute(s))
        }
    }

    pub fn write_bytes(&self, buf: &[u8]) -> ErrorResult<int> {
        unsafe {
            let ret = raw::write(self.fd, buf);
            if ret < 0 {
                Err(cast::transmute(ret))
            } else {
                Ok(ret)
            }
        }
    }

    pub fn write_val<T>(&self, val: &T) -> ErrorResult<int> {
        use std::sys;
        unsafe {
            let buf : &[u8] = cast::transmute((val, sys::size_of::<T>()));
            self.write_bytes(buf)
        }
    }

    pub fn read_bytes(&self, out_buf: &mut [u8]) -> ErrorResult<int> {
        unsafe {
            let ret = raw::read(self.fd, out_buf);
            if ret < 0 {
                Err(cast::transmute(ret))
            } else {
                Ok(ret)
            }
        }
    }

    pub fn reset(&self) -> Error {
        unsafe {
            cast::transmute(raw::lseek(self.fd, 0, 0))
        }
    }

    pub fn close(self) { }
}

impl Drop for File {
    fn drop(&self) {
        if self.fd != 0 {
            raw::close(self.fd);
        }
    }
}

impl Clone for File {
    fn clone(&self) -> File {
        let fd = raw::open(self.path, self.flags, DEFAULT_MODE);
        if fd < 0 {
            sys::fail(sys::err::msg(fd*-1))
        } else {
            File {
                path: self.path.to_owned(),
                flags: self.flags,
                fd: fd
            }
        }
    }
}

fn fmodeflags(mode: &str) -> int {
    let mut flags = 0;

    if mode.len() > 0 {
        if mode.find('+').is_some() {
            flags = raw::O_RDWR;
        } else if mode[0] == 'r' as u8 {
            flags = raw::O_RDONLY;
        } else {
            flags = raw::O_WRONLY;
        }

        if mode.find('s').is_some() {
            flags |= raw::O_ASYNC;
        }

        if mode.find('x').is_some() {
            flags |= raw::O_EXCL;
        }
        if mode[0] != 'r' as u8 {
            flags |= raw::O_CREAT;
        }
        if mode[0] == 'w' as u8 {
            flags |= raw::O_TRUNC;
        }
        if mode[0] == 'a' as u8 {
            flags |= raw::O_APPEND;
        }
    }

    flags
}

#[inline]
pub fn mkdir(path: &str, mode: int) -> Error {
    unsafe {
        let mut name = [0u8,..FILENAME_MAX];
        let nmp : *mut u8 = cast::transmute(&mut name);
        let (ptr, len) : (*u8, uint) = cast::transmute(path);
        ptr::copy_memory(nmp, ptr, len);

        cast::transmute(syscall2(n::MKDIR, nmp as int, mode))
    }
}

#[inline]
pub fn rmdir(path: &str) -> Error {
    unsafe {
        let mut name = [0u8,..FILENAME_MAX];
        let nmp : *mut u8 = cast::transmute(&mut name);
        let (ptr, len) : (*u8, uint) = cast::transmute(path);
        ptr::copy_memory(nmp, ptr, len);

        cast::transmute(syscall1(n::RMDIR, nmp as int))
    }
}

pub mod raw {
    use sys::n;
    use sys::{syscall1,syscall3};
    use std::{cast,ptr};
    use super::FILENAME_MAX;

    pub static O_ACCMODE        : int = 0x0003;

    pub static O_RDONLY         : int = 0x0000;
    pub static O_WRONLY         : int = 0x0001;
    pub static O_RDWR           : int = 0x0002;

    pub static O_CREAT          : int = 0x0040;
    pub static O_EXCL           : int = 0x0080;
    pub static O_NOCTTY         : int = 0x0100;
    pub static O_TRUNC          : int = 0x0200;
    pub static O_APPEND         : int = 0x0400;
    pub static O_NONBLOCK       : int = 0x0800;

    pub static O_SYNC           : int = 0x101000;
    pub static O_ASYNC          : int = 0x002000;
    pub static O_LARGEFILE      : int = 0x008000;

    #[inline]
    pub fn write(fd: int, s: &[u8]) -> int {
        unsafe {
            let (ptr, len) : (int, int) = cast::transmute(s);
            syscall3(n::WRITE, fd, ptr, len)
        }
    }

    #[inline]
    pub fn lseek(fd: int, offset: int, whence: int) -> int {
        unsafe {
            syscall3(n::LSEEK, fd, offset, whence)
        }
    }

    #[inline]
    pub fn read(fd: int, buf: &mut [u8]) -> int {
        unsafe {
            let (ptr, len) : (int, int) = cast::transmute(buf);
            syscall3(n::READ, fd, ptr, len)
        }
    }

    #[inline]
    pub fn open(path: &str, flags: int, mode: int) -> int {
        unsafe {
            let mut name = [0u8,..FILENAME_MAX];
            let nmp : *mut u8 = cast::transmute(&mut name);
            let (ptr, len) : (*u8, uint) = cast::transmute(path);
            ptr::copy_memory(nmp, ptr, len);

            syscall3(n::OPEN, nmp as int, flags, mode)
        }
    }

    #[inline]
    pub fn close(fd: int) -> int {
        unsafe {
            syscall1(n::CLOSE, fd)
        }
    }

}
