use {kutil_cli::debug::*, std::io, thiserror::*};

//
// StoreError
//

/// Store error.
#[derive(Error, Debug)]
pub enum StoreError {
    /// ID.
    #[error("id: {0}")]
    ID(String),
}

impl Debuggable for StoreError {
    fn write_debug_for<WriteT>(&self, _writer: &mut WriteT, _context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        todo!()
    }
}
