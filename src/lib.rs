#![allow(dead_code)]

mod repository;

pub mod error;
pub mod protocol;

pub(crate) mod common {
    pub(crate) type Result<T, E = crate::error::internal::Error> = std::result::Result<T, E>;

    pub(crate) type Error = crate::error::internal::Error;
    pub(crate) type ErrorKind = crate::error::internal::ErrorKind;
}
