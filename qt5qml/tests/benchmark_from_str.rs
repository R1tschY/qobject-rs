#![feature(test)]

extern crate test;

const SMALL_ASCII: &str = "QString";

mod to_string_small {
    use test::Bencher;

    use qt5qml::core::QString;

    use crate::SMALL_ASCII;

    #[bench]
    fn qstring_from_utf8(b: &mut Bencher) {
        b.iter(|| {
            let input: &str = test::black_box(&SMALL_ASCII);
            let res: QString = QString::from_utf8(input);
            res
        });
    }

    #[bench]
    fn qstring_to_utf16(b: &mut Bencher) {
        b.iter(|| {
            let input: &str = test::black_box(&SMALL_ASCII);
            let v: Vec<u16> = input.encode_utf16().collect();
            let res: QString = QString::from_utf16(&v);
            res
        });
    }

    #[bench]
    fn qstring_to_utf16_unchecked(b: &mut Bencher) {
        b.iter(|| {
            let input: &str = test::black_box(&SMALL_ASCII);
            let v: Vec<u16> = input.encode_utf16().collect();
            let res: QString = unsafe { QString::from_utf16_unchecked(&v) };
            res
        });
    }
}
