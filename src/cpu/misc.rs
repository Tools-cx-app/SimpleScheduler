use crate::cpu::freqs::CpuFreqs;

pub fn get_freqs(freqs: &mut CpuFreqs, conut: isize) -> isize {
    freqs.freqs[conut as usize]
}
