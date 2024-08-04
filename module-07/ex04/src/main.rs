use std::ffi::CString;
#[allow(dead_code)]
use std::{ffi::CStr, os::raw::c_void, str::from_utf8_unchecked};

#[derive(Debug)]
struct Errno(libc::c_int);

#[allow(dead_code)]
impl Errno {
    fn last() -> Self {
        // SAFETY:
        // Here, we read a raw pointer to the local thread's errno.
        let errno = unsafe { *libc::__errno_location() };
        Self(errno)
    }

    fn make_last(self) {
        // SAFETY:
        // Here, we write to a raw pointer to the local thread's errno.
        unsafe {
            let errno = libc::__errno_location();
            *errno = self.0
        };
    }

    fn description(self) -> &'static str {
        // SAFETY:
        // Here, we get a raw `mut c_char` pointer to the local thread's errno
        // description, which is guaranteed to be valid.
        // We then convert it into a [u8] slice to convert it into a string.
        unsafe {
            let err_str = libc::strerror(self.0);
            let mut len = 0;

            while *err_str.add(len) != 0 {
                len += 1;
            }

            let bytes = std::slice::from_raw_parts(err_str as *mut u8, len);
            from_utf8_unchecked(bytes)
        }
    }
}

#[derive(Clone, Copy)]
struct Fd(libc::c_int);

#[allow(dead_code, unused_variables)]
impl Fd {
    const STDIN: Self = Self(0);
    const STDOUT: Self = Self(1);
    const STDERR: Self = Self(2);

    fn open(file: &CStr) -> Result<Self, Errno> {
        let filename_as_ptr = file.as_ptr();
        // SAFETY:
        // Here, we use open the file at path `file` in readonly mode.
        let fd = unsafe { libc::open(filename_as_ptr, libc::O_RDONLY) };
        match fd {
            -1 => Err(Errno::last()),
            _ => Ok(Self(fd)),
        }
    }

    fn create(file: &CStr) -> Result<Self, Errno> {
        let filename_as_ptr = file.as_ptr();
        // SAFETY:
        // Here, we use open the file at path `file` in readonly mode.
        let fd = unsafe {
            libc::open(
                filename_as_ptr,
                libc::O_CREAT | libc::O_TRUNC | libc::O_WRONLY,
            )
        };
        match fd {
            -1 => Err(Errno::last()),
            _ => Ok(Self(fd)),
        }
    }

    fn write(self, data: &[u8]) -> Result<usize, Errno> {
        // SAFETY:
        // Here, we write into a file descriptor directly and handle errors.
        let res = unsafe { libc::write(self.0, data.as_ptr() as *const c_void, data.len()) };
        match res {
            -1 => Err(Errno::last()),
            _ => Ok(res as usize),
        }
    }

    fn read(self, buf: &mut [u8]) -> Result<usize, Errno> {
        // SAFETY:
        // Here, we read from a file descriptor directly into our bufand handle errors.
        let res = unsafe { libc::read(self.0, buf.as_mut_ptr() as *mut c_void, buf.len()) };
        match res {
            -1 => Err(Errno::last()),
            _ => Ok(res as usize),
        }
    }

    fn close(self) -> Result<(), Errno> {
        // SAFETY:
        // Here, we close a file descriptor directly and handle errors.
        match unsafe { libc::close(self.0) } {
            -1 => Err(Errno::last()),
            _ => Ok(()),
        }
    }
}

#[derive(Clone)]
struct File(Fd);

#[allow(dead_code)]
impl File {
    fn open(file: &CStr) -> Result<Self, Errno> {
        match Fd::open(file) {
            Ok(fd) => Ok(Self(fd)),
            Err(err) => Err(err),
        }
    }

    fn create(file: &CStr) -> Result<Self, Errno> {
        match Fd::create(file) {
            Ok(fd) => Ok(Self(fd)),
            Err(err) => Err(err),
        }
    }

    fn write(&self, data: &[u8]) -> Result<usize, Errno> {
        match self.0.write(data) {
            Ok(res) => Ok(res),
            Err(err) => Err(err),
        }
    }

    fn read(&self, buf: &mut [u8]) -> Result<usize, Errno> {
        match self.0.read(buf) {
            Ok(res) => Ok(res),
            Err(err) => Err(err),
        }
    }

    fn leak(self) -> Fd {
        self.0
    }
}

impl Drop for File {
    fn drop(&mut self) {
        match self.0.close() {
            Ok(()) => (),
            Err(err) => eprintln!(
                "error closing file descriptor {}: {}",
                self.0 .0,
                err.description()
            ),
        }
    }
}

fn main() {
    let path = CString::new("hello").expect("msg");
    let file = File::create(&path).expect("error creating file");
    let data = "Hello, World!".as_bytes();
    let mut buf = [0u8; 20];
    let _ = file.write(&data).expect("error writing to file");
    let _ = file.read(&mut buf).expect("error reading from file");
}
#[cfg(test)]
mod tests {
    use super::*;
    use libc;
    use std::ffi::CString;

    #[test]
    fn test_last() {
        unsafe {
            *libc::__errno_location() = libc::ENOENT;
        }
        let errno = Errno::last();
        assert_eq!(errno.0, libc::ENOENT);
    }

    #[test]
    fn test_make_last() {
        let errno = Errno(libc::EAGAIN);
        errno.make_last();
        let new_errno = unsafe { *libc::__errno_location() };
        assert_eq!(new_errno, libc::EAGAIN);
    }

    #[test]
    fn test_description() {
        let errno = Errno(libc::ENOENT);
        let description = errno.description();
        assert_eq!(description, "No such file or directory");

        let errno = Errno(libc::EAGAIN);
        let description = errno.description();
        assert_eq!(description, "Resource temporarily unavailable");
    }

    #[test]
    fn test_fd_open() {
        let file_name = CString::new("/tmp/non_existent_file").expect("CString::new failed");
        match Fd::open(&file_name) {
            Ok(_) => panic!("Expected an error, but got Ok"),
            Err(e) => assert_eq!(e.description(), "No such file or directory"),
        }
    }

    #[test]
    fn test_fd_create() {
        let file_name = CString::new("/tmp/test_file").expect("CString::new failed");
        match Fd::create(&file_name) {
            Ok(fd) => {
                assert!(fd.0 >= 0);
                fd.close().expect("Failed to close file");
                unsafe {
                    libc::unlink(file_name.as_ptr());
                }
            }
            Err(_) => panic!("Expected Ok, but got Err"),
        }
    }

    #[test]
    fn test_file_create() {
        let file_name = CString::new("/tmp/test_file_create").expect("CString::new failed");
        let file = File::create(&file_name).expect("error creating file");
        let data = "Hello, World!".as_bytes();
        let mut buf = [0u8; 20];
        let _ = file.write(&data).expect("error writing to file");
        let _ = file.read(&mut buf);
        drop(file)
    }
}
