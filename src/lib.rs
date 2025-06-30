pub mod error;
pub mod terminal;
pub mod buffered_executor;
pub mod service;

pub use error::{Result, TerminalError};
pub use terminal::{Terminal, TerminalManager, TerminalState, SessionState};
pub use buffered_executor::{BufferedCommandExecutor, ExecuteResult, ReadResult};
pub use service::TerminalService;