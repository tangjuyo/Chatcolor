pub mod chatcolor;
pub mod namecolor;

pub use chatcolor::{ChatColorExecutor, init_command_tree as init_chatcolor_command_tree};
pub use namecolor::{NameColorExecutor, init_command_tree as init_namecolor_command_tree}; 