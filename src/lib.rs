pub fn merge_tracks<R1, R2, W>(read1: R1, read2: R2, write: &mut W) -> anyhow::Result<()>
    where R1: std::io::Read,
          R2: std::io::Read,
          W: std::io::Write + std::io::Seek {

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 22050,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut reader1 = hound::WavReader::new(read1)?;
    let mut reader2 = hound::WavReader::new(read2)?;
    let mut writer = hound::WavWriter::new(write, spec)?;

    println!("{:?}", reader1.spec());
    println!("{:?}", reader2.spec());

    let mut max_amp = 0_i16;

    let buffer : Vec<i16> = reader1.samples::<i16>()
        .zip(reader2.samples::<i16>())
        .map(|(sample1, sample2)| { (sample1.unwrap(), sample2.unwrap()) })
        .map(|(sample1, sample2)| {(sample1/2, sample2/2)})
        .map(|(sample1, sample2)| { sample1 + sample2 })
        .map(|sample| {
            if sample.abs() > max_amp { max_amp = sample.abs() }
            sample
        })
        .collect();

    let multiplier = i16::MAX / max_amp;
    buffer.iter()
        .map(|sample| sample * multiplier)
        .for_each(|sample| writer.write_sample(sample).unwrap());

    writer.finalize()?;
    Ok(())
}

#[cfg(test)]
mod test {
    use std::io::{BufReader, BufWriter};

    #[test]
    fn run() {
        let reader1 = BufReader::new(std::fs::File::open("CantinaBand60.wav").unwrap());
        let reader2 = BufReader::new(std::fs::File::open("ImperialMarch60.wav").unwrap());

        let mut writer = BufWriter::new(std::fs::File::create("merged.wav").unwrap());

        super::merge_tracks(reader1, reader2, &mut writer).unwrap();
    }
}
