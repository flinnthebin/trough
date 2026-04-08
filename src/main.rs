use std::io::Write;

fn main() -> Result<(), std::io::Error> {
    let out = std::io::stdout();
    let mut out = out.lock();
    out.write_all(b"RIFF")?;
    out.write_all(&4u32.to_le_bytes())?;
    out.write_all(b"WAVE")?;
    Ok(())
}
