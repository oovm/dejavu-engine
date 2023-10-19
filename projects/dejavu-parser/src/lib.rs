#![deny(missing_debug_implementations)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

pub use yggdrasil_rt::{YggdrasilNode, YggdrasilParser};

pub mod dejavu;
mod utils;

pub use yggdrasil_rt::YggdrasilError;
