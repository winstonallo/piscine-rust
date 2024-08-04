use std::ffi::{c_char, c_int, CStr, CString};
use std::ptr;

#[repr(C)]
#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum Error {
    ErrSuccess,
    ErrMemory,
    ErrNoMoreIds,
    ErrUnknownId,
}

#[allow(dead_code)]
pub type Id = c_int;

#[repr(C)]
#[derive(Debug)]
#[allow(dead_code)]
pub struct User {
    id: Id,
    name: *const c_char,
}

#[repr(C)]
#[derive(Debug)]
#[allow(dead_code)]
pub struct Database {
    next_user_id: Id,
    users: *mut User,
    count: usize,
    allocated: usize,
}

#[allow(dead_code)]
#[link(name = "awesome")]
extern "C" {
    pub fn create_database(database: *mut Database) -> Error;
    pub fn delete_database(database: *mut Database);
    pub fn create_user(database: *mut Database, name: *const c_char, result: *mut Id) -> Error;
    pub fn delete_user(database: *mut Database, id: Id) -> Error;
    pub fn get_user(database: *const Database, id: Id, result: *mut *const User) -> Error;
}

#[allow(dead_code)]
impl Database {
    fn new() -> Result<Self, Error> {
        let mut db = Database {
            next_user_id: 0,
            users: ptr::null_mut(),
            count: 0,
            allocated: 0,
        };
        // SAFETY:
        // Access raw memory via linked C library.
        let result = unsafe { create_database(&mut db) };
        if result == Error::ErrSuccess {
            Ok(db)
        } else {
            Err(result)
        }
    }

    fn create_user(&mut self, name: &CStr) -> Result<Id, Error> {
        let mut user_id: Id = 0;
        // SAFETY:
        // Access raw memory via linked C library.
        let result = unsafe { create_user(self, name.as_ptr(), &mut user_id) };
        if result == Error::ErrSuccess {
            Ok(user_id)
        } else {
            Err(result)
        }
    }

    fn delete_user(&mut self, id: Id) -> Result<(), Error> {
        // SAFETY:
        // Access raw memory via linked C library.
        let result = unsafe { delete_user(self, id) };
        if result == Error::ErrSuccess {
            Ok(())
        } else {
            Err(result)
        }
    }

    fn get_user(&self, id: Id) -> Result<&User, Error> {
        let mut user: *const User = ptr::null();
        // SAFETY:
        // Access raw memory via linked C library.
        let result = unsafe { get_user(self, id, &mut user) };
        if result == Error::ErrSuccess {
            unsafe { Ok(&*user) }
        } else {
            Err(result)
        }
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        // SAFETY:
        // Access raw memory via linked C library.
        unsafe { delete_database(self) };
    }
}

#[allow(dead_code)]
fn main() {
    match Database::new() {
        Ok(mut db) => {
            let name = CString::new("John Doe").expect("CString::new failed");
            match db.create_user(&name) {
                Ok(user_id) => println!("User created with ID: {}", user_id),
                Err(err) => eprintln!("Error creating user: {:?}", err),
            }

            match db.get_user(1) {
                Ok(user) => println!("Found user: {:?}", user),
                Err(err) => eprintln!("Error getting user: {:?}", err),
            }

            match db.delete_user(1) {
                Ok(_) => println!("User deleted"),
                Err(err) => eprintln!("Error deleting user: {:?}", err),
            }
        }
        Err(err) => eprintln!("Error creating database: {:?}", err),
    }
}
