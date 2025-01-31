//! Implementation for Hermit
use crate::Error;
use core::mem::MaybeUninit;

pub use crate::util::{inner_u32, inner_u64};

pub fn fill_inner(mut dest: &mut [MaybeUninit<u8>]) -> Result<(), Error> {
    let res = twizzler_rt_abi::random::twz_rt_get_random(
        dest,
        twizzler_rt_abi::random::GetRandomFlags::empty(),
    );
    if res == 0 {
        panic!("failed to fill entropy bytes");
    }
    Ok(())
}
