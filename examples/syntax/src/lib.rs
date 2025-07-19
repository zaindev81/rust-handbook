pub mod dyn_module;
pub mod arc_module;
pub mod map_module;
pub mod impl_module;
pub mod result_module;
pub mod iter_module;
pub mod str_module;

pub use dyn_module::{dyn_main};
pub use arc_module::{arc_main};
pub use map_module::{map_main};
pub use impl_module::{impl_main};
pub use result_module::{result_main};
pub use iter_module::{iter_main};
pub use str_module::{str_main};