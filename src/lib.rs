pub fn merge_tracks() {

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 22050,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
        let mut reader1 = hound::WavReader::open("CantinaBand60.wav").unwrap();
        let mut reader2 = hound::WavReader::open("ImperialMarch60.wav").unwrap();
        let mut writer = hound::WavWriter::create("merged.wav", spec).unwrap();

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

    writer.finalize().unwrap();
}

#[cfg(test)]
mod test {
    #[test]
    fn run() {
        super::merge_tracks();
    }
}
