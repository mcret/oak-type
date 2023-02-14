#![no_std]
#![no_main]#![feature(type_alias_impl_trait)]

extern crate alloc;

use {defmt_rtt as _, panic_probe as _};

#[global_allocator]
static HEAP: Heap = Heap::empty();

#[embassy_executor::main]
async fn main()
{
    todo!()
}