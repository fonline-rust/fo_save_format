use serde_json::ser::Formatter;
use std::io;

#[derive(Clone, Debug)]
pub struct PrettyFormatter<'a> {
    current_indent: usize,
    has_value: bool,
    indent: &'a [u8],
    inside_array: bool,
}

impl<'a> PrettyFormatter<'a> {
    /// Construct a pretty printer formatter that defaults to using two spaces for indentation.
    pub fn new() -> Self {
        PrettyFormatter::with_indent(b"  ")
    }

    /// Construct a pretty printer formatter that uses the `indent` string for indentation.
    pub fn with_indent(indent: &'a [u8]) -> Self {
        PrettyFormatter {
            current_indent: 0,
            has_value: false,
            indent,
            inside_array: false,
        }
    }
}

impl Default for PrettyFormatter<'_> {
    fn default() -> Self {
        PrettyFormatter::new()
    }
}

impl Formatter for PrettyFormatter<'_> {
    #[inline]
    fn begin_array<W: ?Sized + io::Write>(&mut self, writer: &mut W) -> io::Result<()> {
        //self.current_indent += 1;
        self.has_value = false;
        self.inside_array = true;
        writer.write_all(b"[")
    }

    #[inline]
    fn end_array<W: ?Sized + io::Write>(&mut self, writer: &mut W) -> io::Result<()> {
        //self.current_indent -= 1;

        //if self.has_value {
        // try!(writer.write_all(b"\n"));
        //try!(indent(writer, self.current_indent, self.indent));
        //}
        self.inside_array = false;

        writer.write_all(b"]")
    }

    #[inline]
    fn begin_array_value<W: ?Sized + io::Write>(
        &mut self,
        writer: &mut W,
        first: bool,
    ) -> io::Result<()> {
        if first {
            //try!(writer.write_all(b"\n"));
        } else {
            //try!(writer.write_all(b",\n"));
            writer.write_all(b", ")?;
        }
        //try!(indent(writer, self.current_indent, self.indent));
        Ok(())
    }

    #[inline]
    fn end_array_value<W: ?Sized + io::Write>(&mut self, _writer: &mut W) -> io::Result<()> {
        self.has_value = true;
        Ok(())
    }

    #[inline]
    fn begin_object<W: ?Sized + io::Write>(&mut self, writer: &mut W) -> io::Result<()> {
        if !self.inside_array {
            self.current_indent += 1;
        }
        self.has_value = false;
        writer.write_all(b"{")
    }

    #[inline]
    fn end_object<W: ?Sized + io::Write>(&mut self, writer: &mut W) -> io::Result<()> {
        if !self.inside_array {
            self.current_indent -= 1;

            if self.has_value {
                writer.write_all(b"\n")?;
                indent(writer, self.current_indent, self.indent)?;
            }
        }

        writer.write_all(b"}")
    }

    #[inline]
    fn begin_object_key<W: ?Sized + io::Write>(
        &mut self,
        writer: &mut W,
        first: bool,
    ) -> io::Result<()> {
        if !self.inside_array {
            if first {
                writer.write_all(b"\n")?;
            } else {
                writer.write_all(b",\n")?;
            }
            indent(writer, self.current_indent, self.indent)
        } else if !first {
            writer.write_all(b", ")
        } else {
            Ok(())
        }
    }

    #[inline]
    fn begin_object_value<W: ?Sized + io::Write>(&mut self, writer: &mut W) -> io::Result<()> {
        writer.write_all(b": ")
    }

    #[inline]
    fn end_object_value<W: ?Sized + io::Write>(&mut self, _writer: &mut W) -> io::Result<()> {
        self.has_value = true;
        Ok(())
    }
}

fn indent<W: ?Sized + io::Write>(wr: &mut W, n: usize, s: &[u8]) -> io::Result<()> {
    for _ in 0..n {
        wr.write_all(s)?;
    }

    Ok(())
}
