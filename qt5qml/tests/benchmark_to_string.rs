#![feature(test)]

extern crate test;

const SMALL_ASCII: &str = "QString";
const MEDIUM_ASCII: &str = "A message that could be part of a UI";
const MEDIUM_UNICODE: &str = "В начале сотворил Бог небо и землю.";
const LARGE_UNICODE: &str = "
1	В начале сотворил Бог небо и землю.
2	Земля же была безвидна и пуста, и тьма над бездною, и Дух Божий носился над водою.

3	И сказал Бог: да будет свет. И стал свет.
4	И увидел Бог свет, что он хорош, и отделил Бог свет от тьмы.
5	И назвал Бог свет днем, а тьму ночью. И был вечер, и было утро: день один.
";

mod to_string_small {
    use test::Bencher;

    use qt5qml::core::QString;

    use crate::SMALL_ASCII;

    #[bench]
    fn qstring_to_utf8_lossy(b: &mut Bencher) {
        let input: QString = QString::from_utf8(SMALL_ASCII);
        b.iter(|| {
            let input = test::black_box(&input);
            let res: String = String::from_utf8_lossy(input.to_utf8().as_slice()).into();
            res
        });
    }

    #[bench]
    fn qstring_to_utf16_lossy(b: &mut Bencher) {
        let input: QString = QString::from_utf8(SMALL_ASCII);
        b.iter(|| {
            let input = test::black_box(&input);
            let res: String = String::from_utf16_lossy(input.utf16());
            res
        });
    }

    #[bench]
    fn qstring_to_utf8(b: &mut Bencher) {
        let input: QString = QString::from_utf8(SMALL_ASCII);
        b.iter(|| {
            let input = test::black_box(&input);
            let res: String = input.to_string();
            res
        });
    }

    #[bench]
    fn qstring_to_utf16(b: &mut Bencher) {
        let input: QString = QString::from_utf8(SMALL_ASCII);
        b.iter(|| {
            let input = test::black_box(&input);
            let res: String =
                String::from_utf16(input.utf16()).expect("QString with invalid unicode");
            res
        });
    }
}

mod to_string_empty {
    use test::Bencher;

    use qt5qml::core::QString;

    #[bench]
    fn qstring_to_utf8_lossy(b: &mut Bencher) {
        let input: QString = QString::from_utf8("");
        b.iter(|| {
            let input = test::black_box(&input);
            let res: String = String::from_utf8_lossy(input.to_utf8().as_slice()).into();
            res
        });
    }

    #[bench]
    fn qstring_to_utf16_lossy(b: &mut Bencher) {
        let input: QString = QString::from_utf8("");
        b.iter(|| {
            let input = test::black_box(&input);
            let res: String = String::from_utf16_lossy(input.utf16());
            res
        });
    }

    #[bench]
    fn qstring_to_utf8(b: &mut Bencher) {
        let input: QString = QString::from_utf8("");
        b.iter(|| {
            let input = test::black_box(&input);
            let res: String = input.to_string();
            res
        });
    }

    #[bench]
    fn qstring_to_utf16(b: &mut Bencher) {
        let input: QString = QString::from_utf8("");
        b.iter(|| {
            let input = test::black_box(&input);
            let res: String =
                String::from_utf16(input.utf16()).expect("QString with invalid unicode");
            res
        });
    }
}

mod to_string_medium {
    use test::Bencher;

    use qt5qml::core::QString;

    use crate::MEDIUM_ASCII;

    #[bench]
    fn qstring_to_utf8_lossy(b: &mut Bencher) {
        let input: QString = QString::from_utf8(MEDIUM_ASCII);
        b.iter(|| {
            let input = test::black_box(&input);
            let res: String = String::from_utf8_lossy(input.to_utf8().as_slice()).into();
            res
        });
    }

    #[bench]
    fn qstring_to_utf16_lossy(b: &mut Bencher) {
        let input: QString = QString::from_utf8(MEDIUM_ASCII);
        b.iter(|| {
            let input = test::black_box(&input);
            let res: String = String::from_utf16_lossy(input.utf16());
            res
        });
    }

    #[bench]
    fn qstring_to_utf8(b: &mut Bencher) {
        let input: QString = QString::from_utf8(MEDIUM_ASCII);
        b.iter(|| {
            let input = test::black_box(&input);
            let res: String = input.to_string();
            res
        });
    }

    #[bench]
    fn qstring_to_utf16(b: &mut Bencher) {
        let input: QString = QString::from_utf8(MEDIUM_ASCII);
        b.iter(|| {
            let input = test::black_box(&input);
            let res: String =
                String::from_utf16(input.utf16()).expect("QString with invalid unicode");
            res
        });
    }
}

mod to_string_medium_unicode {
    use test::Bencher;

    use qt5qml::core::QString;

    use crate::MEDIUM_UNICODE;

    #[bench]
    fn qstring_to_utf8_lossy(b: &mut Bencher) {
        let input: QString = QString::from_utf8(MEDIUM_UNICODE);
        b.iter(|| {
            let input = test::black_box(&input);
            let res: String = String::from_utf8_lossy(input.to_utf8().as_slice()).into();
            res
        });
    }

    #[bench]
    fn qstring_to_utf16_lossy(b: &mut Bencher) {
        let input: QString = QString::from_utf8(MEDIUM_UNICODE);
        b.iter(|| {
            let input = test::black_box(&input);
            let res: String = String::from_utf16_lossy(input.utf16());
            res
        });
    }

    #[bench]
    fn qstring_to_utf8(b: &mut Bencher) {
        let input: QString = QString::from_utf8(MEDIUM_UNICODE);
        b.iter(|| {
            let input = test::black_box(&input);
            let res: String = input.to_string();
            res
        });
    }

    #[bench]
    fn qstring_to_utf16(b: &mut Bencher) {
        let input: QString = QString::from_utf8(MEDIUM_UNICODE);
        b.iter(|| {
            let input = test::black_box(&input);
            let res: String =
                String::from_utf16(input.utf16()).expect("QString with invalid unicode");
            res
        });
    }
}

mod to_string_large_unicode {
    use test::Bencher;

    use qt5qml::core::QString;

    use crate::LARGE_UNICODE;

    #[bench]
    fn qstring_to_utf8_lossy(b: &mut Bencher) {
        let input: QString = QString::from_utf8(LARGE_UNICODE);
        b.iter(|| {
            let input = test::black_box(&input);
            let res: String = String::from_utf8_lossy(input.to_utf8().as_slice()).into();
            res
        });
    }

    #[bench]
    fn qstring_to_utf16_lossy(b: &mut Bencher) {
        let input: QString = QString::from_utf8(LARGE_UNICODE);
        b.iter(|| {
            let input = test::black_box(&input);
            let res: String = String::from_utf16_lossy(input.utf16());
            res
        });
    }

    #[bench]
    fn qstring_to_utf8(b: &mut Bencher) {
        let input: QString = QString::from_utf8(LARGE_UNICODE);
        b.iter(|| {
            let input = test::black_box(&input);
            let res: String = input.to_string();
            res
        });
    }

    #[bench]
    fn qstring_to_utf16(b: &mut Bencher) {
        let input: QString = QString::from_utf8(LARGE_UNICODE);
        b.iter(|| {
            let input = test::black_box(&input);
            let res: String =
                String::from_utf16(input.utf16()).expect("QString with invalid unicode");
            res
        });
    }
}
