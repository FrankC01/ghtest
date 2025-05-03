use bitflags::bitflags;
use pyo3::prelude::*;

// The `bitflags!` macro generates `struct`s that manage a set of flags.
bitflags! {
    /// Represents a set of flags.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct Flags: u32 {
        /// The value `A`, at bit position `0`.
        const A = 0b00000001;
        /// The value `B`, at bit position `1`.
        const B = 0b00000010;
        /// The value `C`, at bit position `2`.
        const C = 0b00000100;

        /// The combination of `A`, `B`, and `C`.
        const ABC = Self::A.bits() | Self::B.bits() | Self::C.bits();
    }
}

/// Enchode a key from 'sig_scheme | prv_bytes' to bech32
#[pyfunction]
pub fn encode_bech32() -> u32 {
    let a:u32 = 1;
    let e1 = Flags::A | Flags::C;
    let e2 = Flags::B | Flags::C;
    assert_eq!((e1 | e2), Flags::ABC);   // union
    assert_eq!((e1 & e2), Flags::C);     // intersection
    assert_eq!((e1 - e2), Flags::A);     // set difference
    assert_eq!(!e2, Flags::A);           // set complement
    return a
}

/// The pysui_fastcrypto module implemented in Rust.  In order
/// to import the function name must match the Cargo.toml name
#[pymodule]
fn ghtest(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(encode_bech32, m)?)?;

    Ok(())
}
