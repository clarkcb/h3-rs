extern crate libc;

#[link(name = "h3")]
extern "C" {
    fn geoToH3(g: *const GeoCoordRads, res: i32) -> H3Index;
}

const DEG_TO_RAD: f64 = std::f64::consts::PI / 180.0;

/// H3Index is a point in the H3 geospatial indexing system.
pub type H3Index = u64;

#[repr(C)]
pub struct GeoCoordRads {
    pub lat: f64,
    pub lon: f64,
}

impl GeoCoordRads {
    fn to_h3(&self, res: i32) -> H3Index {
        unsafe { geoToH3(self, res) }
    }
}

/// GeoCoord is a point on the earth. It is comprised of a latitude and longitude expressed in
/// degrees. The C API for H3 expects the latitude and longitude to be expressed in radians so
/// the coordinates are transparently converted to radians before being passed to the C library.
#[derive(Debug, Copy, Clone)]
pub struct GeoCoord {
    pub lat: f64,
    pub lon: f64,
}

impl GeoCoord {
    fn to_radians(&self) -> GeoCoordRads {
        GeoCoordRads {
            lat: self.lat * DEG_TO_RAD,
            lon: self.lon * DEG_TO_RAD,
        }
    }

    pub fn to_h3(&self, res: i32) -> H3Index {
        self.to_radians().to_h3(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_h3() {
        let geo = GeoCoord {
            lat: 67.194013596,
            lon: 191.598258018,
        };

        assert_eq!(geo.to_h3(5), 0x850dab63fffffff);
    }
}
