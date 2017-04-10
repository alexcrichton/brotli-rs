extern crate brotli2;
extern crate timebomb;

use std::io::prelude::*;
use brotli2::write::BrotliDecoder;

// This is a BR file generated by head -c10 /dev/urandom | bro --output file.br
const DATA: &'static [u8] = &[139, 4, 128, 227, 139, 226, 91, 233, 134, 14, 218, 140, 196, 3];

/// In this test, we drop a write::BrotliDecoder after supplying it a truncated input stream.
///
/// The decoder should detect that it is impossible to decode more data and not
/// go into an infinite loop waiting for more data.
#[test]
fn drop_writer_incomplete_input_no_loop() {
    let run = || {
        let mut decoder = BrotliDecoder::new(Vec::new());
        const PREFIX_LEN: usize = 10;
        decoder.write_all(&DATA[..PREFIX_LEN]).unwrap();
        std::mem::drop(decoder);
    };

    timebomb::timeout_ms(run, 5000); // 5 seconds should be plenty of time
}

/// Same as above, but verifying that we get an error if we manually call `finish`;
#[test]
fn finish_writer_incomplete_input_error() {
    let run = || {
        let mut decoder = BrotliDecoder::new(Vec::new());
        const PREFIX_LEN: usize = 10;
        decoder.write_all(&DATA[..PREFIX_LEN]).unwrap();
        decoder.finish().err().expect("finish should error because of incomplete input");
    };

    timebomb::timeout_ms(run, 5000); // 5 seconds should be plenty of time
}
