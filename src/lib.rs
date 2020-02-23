#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![forbid(missing_docs)]

//! A rust wrapper around lai.

mod bindings {
    include!(concat!(env!("OUT_DIR"), "/raw_bindings.rs"));
}

