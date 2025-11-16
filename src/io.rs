use core::fmt;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone, Copy)]
pub enum Error {
    WriteZero,
    FmtError,
}

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()> {
        let mut pos = 0;

        while pos < buf.len() {
            pos += self.write(&buf[pos..])?;
        }

        Ok(())
    }

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

pub trait Read {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>;
    fn read_exact(&mut self, buf: &mut [u8]) -> Result<()> {
        let mut pos = 0;

        while pos < buf.len() {
            pos += self.read(&mut buf[pos..])?;
        }

        Ok(())
    }
}
