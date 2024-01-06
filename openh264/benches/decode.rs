#![feature(test)]

extern crate test;

use openh264::decoder::{Decoder, DecoderConfig};
use openh264::OpenH264API;
use test::{black_box, Bencher};

#[bench]
fn decode_yuv_single_512x512_cavlc(b: &mut Bencher) {
    let source = include_bytes!("../tests/data/single_512x512_cavlc.h264");

    let api = OpenH264API::from_source();
    let config = DecoderConfig::default();
    let mut decoder = Decoder::with_config(api, config).unwrap();

    b.iter(|| {
        let yuv = decoder.decode(&source[..]).unwrap().unwrap();
        let dim = yuv.dimension_rgb();

        black_box(dim);
    });
}

#[bench]
fn decode_yuv_single_512x512_cabac(b: &mut Bencher) {
    let source = include_bytes!("../tests/data/single_512x512_cabac.h264");

    let api = OpenH264API::from_source();
    let config = DecoderConfig::default();
    let mut decoder = Decoder::with_config(api, config).unwrap();

    b.iter(|| {
        let yuv = decoder.decode(&source[..]).unwrap().unwrap();
        let dim = yuv.dimension_rgb();

        black_box(dim);
    });
}

#[bench]
fn decode_yuv_single_1920x1080(b: &mut Bencher) {
    let source = include_bytes!("../tests/data/single_1920x1080_cabac.h264");

    let api = OpenH264API::from_source();
    let config = DecoderConfig::default();
    let mut decoder = Decoder::with_config(api, config).unwrap();

    b.iter(|| {
        let yuv = decoder.decode(&source[..]).unwrap().unwrap();
        let dim = yuv.dimension_rgb();

        black_box(dim);
    });
}

#[bench]
fn decode_yuv_multi_512x512(b: &mut Bencher) {
    let source = include_bytes!("../tests/data/multi_512x512.h264");

    let api = OpenH264API::from_source();
    let config = DecoderConfig::default();
    let mut decoder = Decoder::with_config(api, config).unwrap();

    b.iter(|| {
        let yuv = decoder.decode(&source[..]).unwrap().unwrap();
        let dim = yuv.dimension_rgb();

        black_box(dim);
    });
}

#[bench]
fn whole_decoder(b: &mut Bencher) {
    let source = include_bytes!("../tests/data/single_512x512_cavlc.h264");

    b.iter(|| {
        let api = OpenH264API::from_source();
        let config = DecoderConfig::default();
        let mut decoder = Decoder::with_config(api, config).unwrap();

        let yuv = decoder.decode(&source[..]).unwrap().unwrap();
        let dim = yuv.dimension_rgb();

        black_box(dim);
    });
}
