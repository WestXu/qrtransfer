use std::io::Write;

pub fn compress(input: Vec<u8>) -> Vec<u8> {
    let mut output = Vec::new();
    {
        let mut writer = brotli::CompressorWriter::new(&mut output, 4096, 11, 22);
        writer.write(&input);
    }
    output
}

pub fn decompress(input: Vec<u8>) -> Vec<u8> {
    let mut output = Vec::new();
    {
        let mut writer = brotli::DecompressorWriter::new(&mut output, 4096);
        writer.write(&input);
    }
    output
}
