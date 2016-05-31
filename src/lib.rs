#[allow(non_camel_case_types,dead_code,non_snake_case,private_in_public)]
mod ffi;
use std::path::Path;
use std::ffi::CString;
use std::fmt;

#[derive(Debug)]
#[derive(PartialEq)]
#[repr(C)]
pub enum Error {
    Nomem = -2,
    PathTooLong = -3,
    UnknownField = -4,
    UnknownUuid = -5,
    InvalidTrailId = -6,
    HandleIsNull = -7,
    HandleAlreadyOpened = -8,
    UnknownOption = -9,
    InvalidOptionValue = -10,
    InvalidUuid = -11,
    IoOpen = -65,
    IoClose = -66,
    IoWrite = -67,
    IoRead = -68,
    IoTruncate = -69,
    IoPackage = -70,
    InvalidInfoFile = -129,
    InvalidVersionFile = -130,
    IncompatibleVersion = -131,
    InvalidFieldsFile = -132,
    InvalidUuidsFile = -133,
    InvalidCodebookFile = -134,
    InvalidTrailsFile = -135,
    InvalidLexiconFile = -136,
    InvalidPackage = -137,
    TooManyFields = -257,
    DuplicateFields = -258,
    InvalidFieldname = -259,
    TooManyTrails = -260,
    ValueTooLong = -261,
    AppendFieldsMismatch = -262,
    LexiconTooLarge = -263,
    TimestampTooLarge = -264,
    TrailTooLong = -265,
    OnlyDiffFilter = -513,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            &Error::Nomem => "Nomem",
            &Error::PathTooLong => "PathTooLong",
            &Error::UnknownField => "UnknownField",
            &Error::UnknownUuid => "UnknownUuid",
            &Error::InvalidTrailId => "InvalidTrailId",
            &Error::HandleIsNull => "HandleIsNull",
            &Error::HandleAlreadyOpened => "HandleAlreadyOpened",
            &Error::UnknownOption => "UnknownOption",
            &Error::InvalidOptionValue => "InvalidOptionValue",
            &Error::InvalidUuid => "InvalidUuid",
            &Error::IoOpen => "IoOpen",
            &Error::IoClose => "IoClose",
            &Error::IoWrite => "IoWrite",
            &Error::IoRead => "IoRead",
            &Error::IoTruncate => "IoTruncate",
            &Error::IoPackage => "IoPackage",
            &Error::InvalidInfoFile => "InvalidInfoFile",
            &Error::InvalidVersionFile => "InvalidVersionFile",
            &Error::IncompatibleVersion => "IncompatibleVersion",
            &Error::InvalidFieldsFile => "InvalidFieldsFile",
            &Error::InvalidUuidsFile => "InvalidUuidsFile",
            &Error::InvalidCodebookFile => "InvalidCodebookFile",
            &Error::InvalidTrailsFile => "InvalidTrailsFile",
            &Error::InvalidLexiconFile => "InvalidLexiconFile",
            &Error::InvalidPackage => "InvalidPackage",
            &Error::TooManyFields => "TooManyFields",
            &Error::DuplicateFields => "DuplicateFields",
            &Error::InvalidFieldname => "InvalidFieldname",
            &Error::TooManyTrails => "TooManyTrails",
            &Error::ValueTooLong => "ValueTooLong",
            &Error::AppendFieldsMismatch => "AppendFieldsMismatch",
            &Error::LexiconTooLarge => "LexiconTooLarge",
            &Error::TimestampTooLarge => "TimestampTooLarge",
            &Error::TrailTooLong => "TrailTooLong",
            &Error::OnlyDiffFilter => "OnlyDiffFilter",
        };
        write!(f, "Error::{}", s)
    }
}

pub type UUID = [u8; 16];


pub struct Constructor {
    handle: *mut ffi::tdb_cons,
}

impl Constructor {
    pub fn new() -> Result<Constructor, ()> {
        let handle = unsafe { ffi::tdb_cons_init() };
        if handle.is_null() {
            Err(())
        } else {
            Ok(Constructor { handle: handle })
        }
    }

    pub fn open(&mut self, path: &Path, fields: &[&str]) -> Result<(), Error> {
        let c_path = path_cstr(path).as_ptr();
        let mut field_ptrs = Vec::new();
        for f in fields.iter() {
            field_ptrs.push(f.as_ptr());
        }
        let c_names = field_ptrs.as_slice().as_ptr() as *mut *const i8;
        let ret =
            unsafe { ffi::tdb_cons_open(self.handle, c_path, c_names, field_ptrs.len() as u64) };
        match ret {
            0 => Ok(()),
            _ => Err(unsafe { std::mem::transmute(ret) }),
        }
    }

    pub fn add(&mut self, uuid: &UUID, timestamp: u64, values: &[&str]) -> Result<(), Error> {
        let mut val_ptrs = Vec::new();
        let mut val_lens = Vec::new();
        for v in values.iter() {
            val_ptrs.push(v.as_ptr());
            val_lens.push(v.len() as u64);
        }
        let c_vals = val_ptrs.as_slice().as_ptr() as *mut *const i8;
        let c_vals_lens = val_lens.as_slice().as_ptr() as *const u64;
        let ret = unsafe {
            ffi::tdb_cons_add(self.handle,
                              uuid.as_ptr() as *mut u8,
                              timestamp,
                              c_vals,
                              c_vals_lens)
        };
        match ret {
            0 => Ok(()),
            _ => Err(unsafe { std::mem::transmute(ret) }),
        }
    }

    pub fn close(&mut self) {
        unsafe { ffi::tdb_cons_close(self.handle) };
    }

    pub fn finalize(&mut self) -> Result<(), Error> {
        let ret = unsafe { ffi::tdb_cons_finalize(self.handle) };
        match ret {
            0 => Ok(()),
            _ => Err(unsafe { std::mem::transmute(ret) }),
        }
    }
}

fn path_cstr(path: &Path) -> CString {
    CString::new(path.to_str().unwrap()).unwrap()
}

#[cfg(test)]
mod test_constructor {
    extern crate chrono;
    use super::{Constructor, UUID};
    use std::path::Path;

    #[test]
    fn main() {
        // match ret {
        //     Ok(()) => println!("Ok"),
        //     Err(e) => println!("Error {}", e),
        // }
        // create constructor object
        let mut constructor = Constructor::new().unwrap();
        assert!(!constructor.handle.is_null());

        // open a new db
        let field_names = ["field1", "field2"];
        let db_path = Path::new("test");
        assert!(constructor.open(db_path, &field_names).is_ok());

        //
        let uuid: UUID = [0; 16];
        let vals = ["cats", "dogs"];
        let local: chrono::DateTime<chrono::Local> = chrono::Local::now();
        let timestamp: u64 = local.timestamp() as u64;
        assert!(constructor.add(&uuid, timestamp, &vals).is_ok());

        // finalize db (saves it to disk)
        assert!(constructor.finalize().is_ok());
    }

}