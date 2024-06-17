mod connection;
mod machine; // Include the machine module

pub use connection::{Connection, ConnectionError};
pub use machine::{Machine, VmConfig}; // Export the Machine and VmConfig