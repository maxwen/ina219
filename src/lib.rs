#![no_std]

extern crate alloc;

cfg_if::cfg_if! {
    if #[cfg(feature="ina219")] {
        pub mod error;
        pub mod ina219;
        pub mod physic;
        extern crate embedded_hal as emb_hal;
        extern crate byteorder;
        extern crate hashbrown;
        extern crate collection_literals;
        extern crate lazy_static;
    } else if #[cfg(feature ="physic")] {
        pub mod error;
        pub mod physic;
        extern crate collection_literals;
        extern crate lazy_static;
        extern crate hashbrown;
    }
}
