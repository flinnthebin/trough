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

/// # Safety
///
/// Only safe to call on packed types. If called on unpacked types, this
/// would allow you to read padding bytes, which may be uninitialized memory
// cast_to_bytes takes a reference to a T, and gives you back a slice of the bytes
// of that type.
unsafe fn cast_to_bytes<T>(t: &T) -> &[u8] {
    // cast t to a raw pointer to a T, then cast that to a raw pointer to a u8.
    // so now it is a pointer to the same place, of type byte instead of type T
    let ptr = t as *const T as *const u8;
    // create a slice of type ptr (u8), pass size of T (number of bytes of data)
    // returned unsafely as a slice
    // Safety:
    //
    // ptr is valid until ptr + sizeof<T> since it _was_ a &T which is valid for sizeof<T>
    // ptr to ptr+sizeof is valid since we had a &T
    // ptr to ptr+sizeof contains initialized bytes by the safety requirement of this function
    unsafe { std::slice::from_raw_parts(ptr, std::mem::size_of::<T>()) }
}
