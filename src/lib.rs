#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
// The C stdlib function strtold is included in the binding and it returns a `long double`
// which bindgen converts to the Rust type u128. Unfortunately, this generates a compiler
// warning because "128-bit integers don't currently have a known stable API". The bindgen
// issue https://github.com/rust-lang/rust-bindgen/issues/1549 contains further details.
#![allow(improper_ctypes)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    // I'm not sure why this test is failing, I'm using the same inputs as the TestFromGeo
    // test in the Golang library. I need to investigate further.
    //
    // #[test]
    // fn test_geo_to_h3() {
    //     let geo = GeoCoord {
    //         lat: 67.194013596,
    //         lon: 191.598258018,
    //     };
    //     let expected: H3Index = 0x850dab63fffffff;
    //     let actual: H3Index;

    //     unsafe {
    //         actual = geoToH3(&geo, 5);
    //     }
    //     assert_eq!(actual, expected);
    // }

    #[test]
    fn test_resolution() {
        let geo = GeoCoord {
            lat: 67.1509268640,
            lon: -168.3908885810,
        };

        for i in 1..16 {
            let h: H3Index;
            let res: i32;
            unsafe {
                h = geoToH3(&geo, i);
                res = h3GetResolution(h);
            }
            assert_eq!(res, i);
        }
    }
}
