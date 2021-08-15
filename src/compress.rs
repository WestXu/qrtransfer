use std::io::Write;

pub fn compress(input: Vec<u8>) -> Vec<u8> {
    let mut output = Vec::new();
    {
        let mut writer = brotli::CompressorWriter::new(&mut output, 4096, 11, 22);
        writer.write_all(&input).expect("Failed compressing.");
    }
    output
}

pub fn decompress(input: Vec<u8>) -> Vec<u8> {
    let mut output = Vec::new();
    {
        let mut writer = brotli::DecompressorWriter::new(&mut output, 4096);
        writer.write_all(&input).expect("Failed decompressing.");
    }
    output
}

#[test]
fn test_compress() {
    let data: Vec<u8> = vec![
        119, 115, 108, 32, 47, 104, 111, 109, 101, 47, 108, 120, 117, 119, 115, 108, 47, 109, 105,
        110, 105, 99, 111, 110, 100, 97, 51, 47, 98, 105, 110, 47, 105, 112, 121, 116, 104, 111,
        110, 32, 45, 45, 112, 100, 98, 32, 45, 99, 32, 34, 115, 104, 111, 119, 95, 100, 102, 40,
        112, 100, 46, 114, 101, 97, 100, 95, 112, 105, 99, 107, 108, 101, 40, 80, 97, 116, 104, 40,
        114, 39, 37, 49, 39, 41, 41, 41, 34, 40, 80, 97, 116, 104, 40, 114, 39, 37, 49, 39, 41, 41,
        41, 34, 40, 80, 97, 116, 104, 40, 114, 39, 37, 49, 39, 41, 41, 41, 34, 40, 80, 97, 116,
        104, 40, 114, 39, 37, 49, 39, 41, 41, 41, 34,
    ];

    let compressed = compress(data.clone());
    let decompressed = decompress(compressed);
    assert_eq!(data, decompressed)
}
