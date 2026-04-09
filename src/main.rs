use std::io::Write;
use zerocopy::{
    Immutable, IntoBytes,
    little_endian::{U16, U32},
};

#[derive(IntoBytes, Immutable)]
#[repr(u16)]
enum WaveFormatCategory {
    /// Microsoft Pulse Code Modulation (PCM) format
    Pcm = 0x0001u16.to_le(),
}

#[derive(IntoBytes, Immutable)]
#[repr(C, packed)]
struct FormatChunkCommon {
    format_tag: WaveFormatCategory,
    channels: U16,
    samples_per_sec: U32,
    avg_bytes_per_sec: U32,
    block_align: U16,
}

fn main() -> Result<(), std::io::Error> {
    let mut out = std::fs::File::create("audio.wav")?;
    out.write_all(b"RIFF")?;
    out.write_all(&0u32.to_le_bytes())?; // format chunk
    out.write_all(b"WAVE")?;
    out.write_all(b"fmt ")?;
    out.write_all(&(std::mem::size_of::<FormatChunkCommon>() as u32).to_le_bytes())?;
    FormatChunkCommon {
        format_tag: WaveFormatCategory::Pcm,
        channels: 1.into(),
        samples_per_sec: 44100.into(),
        avg_bytes_per_sec: (2 * 44100).into(),
        block_align: 64.into(),
    }
    .write_to_io(&mut out)?;
    Ok(())
}
