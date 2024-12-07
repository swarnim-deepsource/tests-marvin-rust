use std::io;

fn foo<W: io::Write>(w: &mut W) -> io::Result<()> {
    w.write(b"foo")?;
    let bytes = w.write(b"foo")?; // Should not catch this
    Ok(())
}

unsafe fn bar<R: io::Read>(r: &mut R) -> io::Result<()> {
    let mut src = String::default();
    r.read(src.as_bytes_mut())?;
    let bytes = r.read(src.as_bytes_mut())?; // Should not catch this
    Ok(())
}

struct A {
    b: String,
}

impl A {
    fn write(&mut self, s: &str) {
        self.b = String::from(s);
    }

    fn read(&mut self, s: &str) {
        self.b = String::from(s);
    }
}

fn baz(w: &mut A) -> std::io::Result<()> {
    // Should not catch these
    w.write("Hello there!");
    w.read("General Kenobi!");
    Ok(())
}
