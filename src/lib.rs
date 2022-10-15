cfg_if::cfg_if! {
    if #[cfg(feature="ina219")] {
        pub mod error;
        pub mod ina219;
        pub mod physic;
        extern crate embedded_hal as emb_hal;
        extern crate collection_literals;
        extern crate byteorder;
        extern crate core;

    } else if #[cfg(feature ="physic")] {
        pub mod error;
        pub mod physic;
        extern crate collection_literals;
        extern crate core;

    }
}
