#[cfg(test)]
mod tests {
    // #[test]
    // fn from_hex_1() {
    //     let big_u288 = u288::from_hex("3fffffffffffffffffffffffffffffffb");
    //     assert_eq!(
    //         u288::from_hex()
    //     )
    // }

    use const_time_bignum::u288;

    #[test]
    fn left_shift_1() {
        assert_eq!(
            u288::from_slice(&[255, 0]) << u288::from_hex("1"),
            u288::from_slice(&[0, 255, 0]) // Looks funky because data is little endian
        );
    }

    #[test]
    fn right_shift_1() {
        assert_eq!(
            u288::from_slice(&[255, 0]) >> u288::from_hex("1"),
            u288::from_slice(&[0]) // Looks funky because data is little endian
        );
    }

    #[test]
    fn less_than_1() {
        assert_eq!(u288::from_hex("f0") < u288::from_hex("ff"), true);
    }

    #[test]
    fn less_than_2() {
        assert_eq!(
            u288::from_slice(&[0, 255, 0]) < u288::from_slice(&[255, 0, 255]),
            true
        );
    }

    #[test]
    fn less_than_3() {
        assert_eq!(
            u288::from_hex("ffffff") < u288::from_hex("ffffff"),
            false
        );
    }

    #[test]
    fn less_than_4() {
        assert_eq!(u288::from_hex("a0b5") < u288::from_hex("a0b5"), false);
    }

    #[test]
    fn greater_than_1() {
        assert_eq!(
            u288::from_hex("fffffff") > u288::from_hex("ffffff"),
            true
        );
    }

    #[test]
    fn greater_than_2() {
        assert_eq!(
            u288::from_hex("f000000") > u288::from_hex("826fe5"),
            true
        );
    }

    #[test]
    fn greater_than_3() {
        assert_eq!(
            u288::from_slice(&[255, 0, 255]) > u288::from_slice(&[0, 0, 255, 255]),
            false
        );
    }

    #[test]
    fn greater_than_4() {
        assert_eq!(u288::from_hex("8f27") > u288::from_hex("8f27"), false);
    }

    #[test]
    fn less_than_or_equal_1() {
        assert_eq!(
            u288::from_hex("38f6a") <= u288::from_hex("38f6a"),
            true
        );
    }

    #[test]
    fn less_than_or_equal_2() {
        assert_eq!(
            u288::from_hex("fff") <= u288::from_hex("f38f6a"),
            true
        );
    }

    #[test]
    fn less_than_or_equal_3() {
        assert_eq!(u288::from_hex("fff") <= u288::from_hex("ffe"), false);
    }

    #[test]
    fn greater_than_or_equal_1() {
        assert_eq!(u288::from_hex("fff") >= u288::from_hex("fff"), true);
    }

    #[test]
    fn greater_than_or_equal_2() {
        assert_eq!(u288::from_hex("fff") >= u288::from_hex("f5f"), true);
    }

    #[test]
    fn greater_than_or_equal_3() {
        assert_eq!(
            u288::from_hex("fff") >= u288::from_hex("f8fff"),
            false
        );
    }

    #[test]
    fn division_1() {
        assert_eq!(
            u288::from_hex("a") / u288::from_hex("2"),
            u288::from_hex("5")
        );
    }

    #[test]
    fn division_2() {
        assert_eq!(
            u288::from_hex("f") / u288::from_hex("4"),
            u288::from_hex("3")
        );
    }

    #[test]
    fn division_3() {
        assert_eq!(
            u288::from_hex("e") / u288::from_hex("10"),
            u288::from_hex("0")
        );
    }

    #[test]
    fn division_4() {
        assert_eq!(
            u288::from_slice(&[255, 255]) / u288::from_slice(&[255, 0]),
            u288::from_slice(&[1, 1])
        );
    }

    #[test]
    fn remainder_1() {
        assert_eq!(
            u288::from_hex("a") % u288::from_hex("3"),
            u288::from_hex("1")
        );
    }

    #[test]
    fn remainder_2() {
        assert_eq!(
            u288::from_hex("5") % u288::from_hex("3"),
            u288::from_hex("2")
        );
    }

    #[test]
    fn remainder_3() {
        assert_eq!(
            u288::from_hex("f6") % u288::from_hex("74e"),
            u288::from_hex("f6")
        );
    }

    #[test]
    fn remainder_4() {
        assert_eq!(
            u288::from_hex("fff123") % u288::from_hex("fff123"),
            u288::from_hex("0")
        );
    }

    #[test]
    fn to_hex_1() {
        assert_eq!(
            u288::from_hex("BABAFAFA").to_hex(),
            "0000000000000000000000000000000000000000000000000000000000000000babafafa"
        );
    }

    #[test]
    fn to_hex_2() {
        assert_eq!(
            u288::from_hex("3fffffffffffffffffffffffffffffffb").to_hex(),
            "0000000000000000000000000000000000000003fffffffffffffffffffffffffffffffb"
        );
    }

    #[test]
    fn from_slice_1() {
        assert_eq!(u288::from_slice(&[1, 1]), u288::from_hex("101"));
    }

    #[test]
    fn from_slice_2() {
        assert_eq!(u288::from_slice(&[255, 16]), u288::from_hex("10FF"));
    }

    #[test]
    fn from_hex_3() {
        assert_eq!(
            u288::from_hex("ff").get_bytes(),
            [
                255u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ]
        );
    }

    #[test]
    fn from_hex_4() {
        assert_eq!(
            u288::from_hex("1fff").get_bytes(),
            [
                255u8, 31u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ]
        );
    }

    // #[test]
    // fn add_msb_1() {
    //     let mut big_u288 = u288::from_hex("9");
    //     big_u288.add_msb();
    // }

    #[test]
    fn add_1() {
        assert_eq!(
            u288::from_hex("1") + u288::from_hex("ff"),
            u288::from_hex("100")
        );
    }

    #[test]
    fn add_2() {
        assert_eq!(
            u288::from_hex("C1583054D5A6350B37E23A")
                + u288::from_hex("2A677ACE04C0037CA98B6BC"),
            u288::from_hex("367cfdd3521a66cd5d098f6")
        );
    }

    #[test]
    fn add_3() {
        assert_eq!(
            u288::from_slice(&[0, 255, 255]) + u288::from_slice(&[255, 255, 0]),
            u288::from_slice(&[255, 254, 0, 1])
        );
    }

    #[test]
    fn sub_1() {
        assert_eq!(
            u288::from_hex("ff") - u288::from_hex("0f"),
            u288::from_hex("f0")
        );
    }

    #[test]
    fn sub_2() {
        assert_eq!(
            u288::from_slice(&[0, 255]) - u288::from_slice(&[255, 0]),
            u288::from_slice(&[1, 254])
        );
    }

    #[test]
    fn sub_3() {
        assert_eq!(
            u288::from_hex(
                "f40d1ebbd170aa4d28a333d8b12a27a70535f29f3e841e5655201f4ef7f31afc36ec06be"
            ) - u288::from_hex(
                "546030bdb669182f46cecd7a76c9ebb8249caa348f243cdce2a692ad90e9b15fe4f29116"
            ),
            u288::from_hex(
                "9facedfe1b07921de1d4665e3a603beee099486aaf5fe17972798ca16709699c51f975a8"
            )
        );
    }

    // #[test]
    // fn pad_array_hex_1() {
    //     assert_eq!(
    //         pad_array_hex(&[255, 255]),
    //         [
    //             255u8, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    //             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    //             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
    //         ]
    //     );
    // }

    #[test]
    fn multiply_1() {
        assert_eq!(
            u288::from_slice(&[255, 100]) * u288::from_slice(&[005, 000]),
            u288::from_slice(&[251, 248, 001])
        );
    }

    #[test]
    fn multiply_2() {
        assert_eq!(
            u288::from_slice(&[255, 255, 255, 000]) * u288::from_slice(&[255, 000, 000, 000]),
            u288::from_slice(&[001, 255, 255, 254])
        );
    }

    #[test]
    fn multiply_3() {
        assert_eq!(
            u288::from_slice(&[255, 255, 255]) * u288::from_slice(&[255, 255, 255]),
            u288::from_hex("fffffe000001")
        );
    }

    #[test]
    fn multiply_4() {
        assert_eq!(
            u288::from_slice(&[
                67, 114, 121, 112, 116, 111, 103, 114, 97, 112, 104, 105, 99, 32, 70
            ]) * u288::from_slice(&[
                133, 214, 190, 8, 84, 85, 109, 3, 124, 68, 82, 14, 64, 213, 6, 8
            ]),
            u288::from_hex("232e2481e77d27fa798895e14ee9e0f2779453994ac90ed284034da565ecf")
        );
    }

    #[test]
    fn multiply_5() {
        assert_eq!(
            u288::from_slice(&[255, 255]) * u288::from_slice(&[0, 1]),
            u288::from_slice(&[0, 255, 255])
        );
    }
}
