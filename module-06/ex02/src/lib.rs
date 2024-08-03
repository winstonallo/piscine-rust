use std::cell::Cell;
use std::thread_local;

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq)]
enum Error {
    Success,
    FileNotFound,
    IsDirectory,
    WriteError,
    ReadError,
}

thread_local! {
    pub static LAST_ERROR: Cell<Error> = Cell::new(Error::Success)
}

#[allow(dead_code)]
impl Error {
    fn make_last(self) {
        LAST_ERROR.with(|last_error| {
            last_error.set(self);
        });
    }

    fn last() -> Self {
        LAST_ERROR.with(|last_error| last_error.get())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn last() {
        assert_eq!(Error::last(), Error::Success);
    }

    #[test]
    fn make_last() {
        let err = Error::FileNotFound;
        err.make_last();
        assert_eq!(Error::last(), Error::FileNotFound);
    }
}
