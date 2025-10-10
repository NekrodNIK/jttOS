use core::fmt;

pub type Result<T> = core::result::Result<T, Error>;

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub enum Error {
    Other,
    NotFound,
    PermissionDenied,
    ConnectionRefused,
    ConnectionReset,
    ConnectionAborted,
    NotConnected,
    AddrInUse,
    AddrNotAvailable,
    BrokenPipe,
    AlreadyExists,
    InvalidInput,
    InvalidData,
    TimedOut,
    Interrupted,
    Unsupported,
    OutOfMemory,
    WriteZero,
    FmtError,
}

// TODO: expand to buffered writers
pub trait Write {
    fn write_all(&mut self, buf: &[u8]) -> Result<()>;

    fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> Result<()> {
        struct Adapter<'a, T: Write + ?Sized> {
            inner: &'a mut T,
            error: Result<()>,
        }

        impl<T: Write + ?Sized> fmt::Write for Adapter<'_, T> {
            fn write_str(&mut self, s: &str) -> fmt::Result {
                match self.inner.write_all(s.as_bytes()) {
                    Ok(()) => Ok(()),
                    Err(e) => {
                        self.error = Err(e);
                        Err(fmt::Error)
                    }
                }
            }
        }

        let mut output = Adapter {
            inner: self,
            error: Ok(()),
        };

        match fmt::write(&mut output, args) {
            Ok(()) => Ok(()),
            Err(..) => output.error,
        }
    }
}
