//! Kleine Soundeffekte, im Code erzeugt (keine Asset-Dateien, keine Lizenzfragen).
//! Erzeugt 16-bit-Mono-WAV-Bytes, die `macroquad::audio::load_sound_from_bytes` lädt.
//! Keine Abhängigkeit zu macroquad, daher nativ mit `cargo test` testbar.

pub const SAMPLE_RATE: u32 = 44_100;

/// Rechteckwelle mit linearem Ausklingen, als komplette WAV-Datei.
pub fn beep_wav(freq_hz: f32, duration_s: f32, volume: f32) -> Vec<u8> {
    let n = (SAMPLE_RATE as f32 * duration_s) as u32;
    let amp = (volume.clamp(0.0, 1.0) * i16::MAX as f32) as f32;
    let mut samples = Vec::with_capacity(n as usize);
    for i in 0..n {
        let t = i as f32 / SAMPLE_RATE as f32;
        let phase = (t * freq_hz).fract();
        let square = if phase < 0.5 { 1.0 } else { -1.0 };
        let env = 1.0 - i as f32 / n.max(1) as f32;
        samples.push((square * amp * env) as i16);
    }
    wav_mono16(&samples)
}

/// Verpackt 16-bit-Mono-Samples als RIFF/WAVE.
pub fn wav_mono16(samples: &[i16]) -> Vec<u8> {
    let data_len = (samples.len() * 2) as u32;
    let mut out = Vec::with_capacity(44 + data_len as usize);
    out.extend_from_slice(b"RIFF");
    out.extend_from_slice(&(36 + data_len).to_le_bytes());
    out.extend_from_slice(b"WAVEfmt ");
    out.extend_from_slice(&16u32.to_le_bytes()); // fmt-Chunk-Länge
    out.extend_from_slice(&1u16.to_le_bytes()); // PCM
    out.extend_from_slice(&1u16.to_le_bytes()); // Mono
    out.extend_from_slice(&SAMPLE_RATE.to_le_bytes());
    out.extend_from_slice(&(SAMPLE_RATE * 2).to_le_bytes()); // Byte-Rate
    out.extend_from_slice(&2u16.to_le_bytes()); // Block-Align
    out.extend_from_slice(&16u16.to_le_bytes()); // Bits pro Sample
    out.extend_from_slice(b"data");
    out.extend_from_slice(&data_len.to_le_bytes());
    for s in samples {
        out.extend_from_slice(&s.to_le_bytes());
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wav_header_is_valid() {
        let wav = beep_wav(880.0, 0.1, 0.3);
        assert_eq!(&wav[0..4], b"RIFF");
        assert_eq!(&wav[8..16], b"WAVEfmt ");
        assert_eq!(&wav[36..40], b"data");
        let riff_len = u32::from_le_bytes(wav[4..8].try_into().unwrap()) as usize;
        assert_eq!(riff_len + 8, wav.len());
    }

    #[test]
    fn beep_has_expected_length_and_fades_out() {
        let wav = beep_wav(440.0, 0.05, 0.5);
        let n = (SAMPLE_RATE as f32 * 0.05) as usize;
        assert_eq!(wav.len(), 44 + n * 2);
        let first = i16::from_le_bytes([wav[44], wav[45]]).unsigned_abs();
        let last = i16::from_le_bytes([wav[wav.len() - 2], wav[wav.len() - 1]]).unsigned_abs();
        assert!(first > last);
    }
}
