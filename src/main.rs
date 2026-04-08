use std::io::Write;

#[repr(u16)]
enum WaveFormatCategory {
    /// Microsoft Pulse Code Modulation (PCM) format
    Pcm = 0x0001,
}

#[repr(C, packed)]
struct FormatChunkCommon {
    format_tag: u16,
    channels: u16,
    samples_per_sec: u32,
    avg_bytes_per_sec: u32,
    block_align: u16,
}

fn main() -> Result<(), std::io::Error> {
    let mut out = std::fs::File::create("audio.wav")?;
    out.write_all(b"RIFF")?;
    out.write_all(&0u32.to_le_bytes())?; // format chunk
    out.write_all(b"WAVE")?;
    out.write_all(b"fmt ")?;
    out.write_all(&(std::mem::size_of::<FormatChunkCommon>() as u32).to_le_bytes())?;
    out.write_all(unsafe {
        cast_to_bytes(&FormatChunkCommon {
            format_tag: WaveFormatCategory::Pcm as u16,
            channels: 1,
            samples_per_sec: 44100,
            avg_bytes_per_sec: 2 * 44100,
            block_align: 64,
        })
    })?;
    out.write_all(&0u32.to_le_bytes())?; // wave chunk
    Ok(())
}

unsafe fn cast_to_bytes<T>(t: &T) -> &[u8] {
    let ptr = t as *const T as *const u8;
    unsafe { std::slice::from_raw_parts(ptr, std::mem::size_of::<T>()) }
}
