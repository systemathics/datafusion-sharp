// Every allocation of the library goes through mimalloc, on every OS: the system allocators behave too differently. Windows' heap hands blocks of more than
// ~512 KB - the column buffers of large batches - to VirtualAlloc/VirtualFree each time, a kernel round trip, page faults and zeroed pages per batch, where
// glibc keeps and reuses them; mimalloc keeps and reuses them on both.
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[macro_use]
mod macros;

pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/datafusion_sharp_proto.rs"));
}

pub mod cancellation;
pub mod common;
pub mod context;
pub mod dataframe;
pub mod error;
pub mod logger;
mod mappers;
pub mod memory_store;
pub mod runtime;

pub use common::*;
pub use error::*;
pub use proto::*;

pub use context::*;
pub use dataframe::*;
pub use runtime::*;
