extern crate libc;

#[macro_use]
extern crate failure;

use std::ffi::CString;

use libc::{c_char, c_int, c_ulonglong};

#[link(name = "h3")]
extern "C" {
    fn geoToH3(g: *const GeoCoordInternal, res: c_int) -> c_ulonglong;
    fn h3ToGeo(h3: c_ulonglong, g: *mut GeoCoordInternal);
    fn h3ToGeoBoundary(h3: c_ulonglong, gp: *mut GeoBoundaryInternal);
    fn h3GetResolution(h: c_ulonglong) -> c_int;
    fn h3GetBaseCell(h: c_ulonglong) -> c_int;
    fn stringToH3(str: *const c_char) -> c_ulonglong;
    fn h3IsValid(h: c_ulonglong) -> c_int;
}

const DEG_TO_RAD: f64 = std::f64::consts::PI / 180.0;
const RAD_TO_DEG: f64 = 180.0 / std::f64::consts::PI;

// Maximum number of cell boundary vertices. The worst case is a pentagon: 5 original verts and
// 5 edge crossings.
const MAX_CELL_BNDRY_VERTS: usize = 10;

/// H3Index is a point in the H3 geospatial indexing system.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct H3Index(u64);

impl H3Index {
    /// Creates a new `H3Index` from the given point. If the point is not a valid index in H3
    /// then `None` is returned.
    ///
    /// # Example
    ///
    /// ```
    /// extern crate h3_rs as h3;
    /// use h3::H3Index;
    ///
    /// let h = H3Index::new(0x850dab63fffffff).unwrap();
    /// ```
    pub fn new(h: u64) -> Result<Self, Error> {
        let res;
        unsafe {
            res = h3IsValid(h);
        }
        if res == 0 {
            return Err(Error::InvalidIndex { value: h });
        }
        Ok(Self(h))
    }

    /// Converts a string to an H3 index.
    ///
    /// # Example
    ///
    /// ```
    /// extern crate h3_rs as h3;
    /// use h3::H3Index;
    ///
    /// assert_eq!(H3Index::from_str("0x850dab63fffffff").unwrap(), H3Index::new(0x850dab63fffffff).unwrap())
    /// ```
    pub fn from_str(s: &str) -> Result<Self, Error> {
        let c_str = match CString::new(s) {
            Ok(c_str) => c_str,
            Err(_) => {
                return Err(Error::InvalidString {
                    value: s.to_owned(),
                })
            }
        };

        let h;
        unsafe {
            h = stringToH3(c_str.as_ptr());
        }

        if h == 0 {
            return Err(Error::InvalidString {
                value: s.to_owned(),
            });
        }
        return Ok(H3Index(h));
    }

    /// Finds the centroid of the index.
    ///
    /// # Example
    ///
    /// ```
    /// extern crate h3_rs as h3;
    /// use h3::{GeoCoord, H3Index};
    ///
    /// let h = H3Index::new(0x850dab63fffffff).unwrap();
    /// assert_eq!(h.to_geo(), GeoCoord::new(67.15092686397712, -168.39088858096966));
    /// ```
    pub fn to_geo(self) -> GeoCoord {
        let mut geo = GeoCoordInternal::new(0.0, 0.0);
        unsafe {
            h3ToGeo(self.0, &mut geo);
        }
        geo.to_deg()
    }

    /// Finds the boundary of the index.
    ///
    /// # Example
    ///
    /// ```
    ///  // TODO
    /// ```
    pub fn to_geo_boundary(self) -> GeoBoundary {
        let mut gb = GeoBoundaryInternal::new();
        unsafe {
            h3ToGeoBoundary(self.0, &mut gb);
        }
        gb.convert()
    }

    /// Returns the resolution of the index.
    ///
    /// # Example
    ///
    /// ```
    /// extern crate h3_rs as h3;
    /// use h3::H3Index;
    ///
    /// let h = H3Index::new(0x850dab63fffffff).unwrap();
    /// assert_eq!(h.resolution(), 5);
    /// ```
    pub fn resolution(self) -> i32 {
        unsafe { h3GetResolution(self.0) }
    }

    /// Returns the base cell number of the index.
    ///
    /// # Example
    ///
    /// ```
    /// extern crate h3_rs as h3;
    /// use h3::H3Index;
    ///
    /// let h = H3Index::new(0x850dab63fffffff).unwrap();
    /// assert_eq!(h.base_cell(), 6);
    /// ```
    pub fn base_cell(self) -> i32 {
        unsafe { h3GetBaseCell(self.0) }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GeoCoordInternal {
    pub lat: f64,
    pub lon: f64,
}

impl GeoCoordInternal {
    pub fn new(lat: f64, lon: f64) -> Self {
        Self { lat, lon }
    }

    fn to_deg(&self) -> GeoCoord {
        GeoCoord::new(self.lat * RAD_TO_DEG, self.lon * RAD_TO_DEG)
    }

    fn to_h3(&self, res: i32) -> H3Index {
        unsafe { H3Index(geoToH3(self, res)) }
    }
}

/// GeoCoord is a point on the earth. It is comprised of a latitude and longitude expressed in
/// degrees. The C API for H3 expects the latitude and longitude to be expressed in radians so
/// the coordinates are transparently converted to radians before being passed to the C library.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct GeoCoord {
    pub lat: f64,
    pub lon: f64,
}

impl GeoCoord {
    /// Creates a new `GeoCoord` from the given latitude and longitude. The unit of the
    /// coordinates is degrees.
    ///
    /// # Example
    ///
    /// ```
    /// extern crate h3_rs as h3;
    /// use h3::GeoCoord;
    ///
    /// let mut coord: GeoCoord = GeoCoord::new(67.194013596, 191.598258018);
    /// ```
    pub fn new(lat: f64, lon: f64) -> Self {
        Self { lat, lon }
    }

    fn to_radians(&self) -> GeoCoordInternal {
        GeoCoordInternal::new(self.lat * DEG_TO_RAD, self.lon * DEG_TO_RAD)
    }

    /// Indexes the location at the specified resolution.
    ///
    /// # Example
    ///
    /// ```
    /// extern crate h3_rs as h3;
    /// use h3::{GeoCoord, H3Index};
    ///
    /// let mut coord: GeoCoord = GeoCoord::new(67.194013596, 191.598258018);
    /// assert_eq!(coord.to_h3(5).unwrap(), H3Index::new(0x850dab63fffffff).unwrap());
    /// ```
    pub fn to_h3(&self, res: i32) -> Result<H3Index, Error> {
        let index = self.to_radians().to_h3(res);
        if index.0 == 0 {
            return Err(Error::FailedConversion);
        }
        return Ok(index);
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct GeoBoundaryInternal {
    num_verts: i32,
    verts: [GeoCoordInternal; MAX_CELL_BNDRY_VERTS],
}

impl GeoBoundaryInternal {
    fn new() -> Self {
        Self {
            num_verts: 0,
            verts: [GeoCoordInternal::new(0.0, 0.0); MAX_CELL_BNDRY_VERTS],
        }
    }

    fn convert(&self) -> GeoBoundary {
        let mut verts = Vec::with_capacity(self.num_verts as usize);
        for i in 0..self.num_verts {
            verts.push(self.verts[i as usize].to_deg());
        }
        GeoBoundary { verts }
    }
}

/// GeoBoundary is a collection of points which defines the boundary of a cell.
#[derive(Debug, Clone)]
pub struct GeoBoundary {
    pub verts: Vec<GeoCoord>,
}

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "invalid value for H3 index: {}", value)]
    InvalidIndex { value: u64 },
    #[fail(display = "invalid string representation of H3 index: {}", value)]
    InvalidString { value: String },
    #[fail(display = "could not convert to H3 index")]
    FailedConversion,
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Setup {
        h3_index: H3Index,
        geo_coord: GeoCoord,
    }

    impl Setup {
        fn new() -> Self {
            Self {
                h3_index: H3Index::new(0x850dab63fffffff).unwrap(),
                geo_coord: GeoCoord::new(67.15092686397712, -168.39088858096966),
            }
        }
    }

    #[test]
    fn test_h3_from_str() {
        assert_eq!(
            H3Index::from_str("0x850dab63fffffff").unwrap(),
            H3Index::new(0x850dab63fffffff).unwrap()
        );
        assert!(H3Index::from_str("invalid string").is_err());
    }

    #[test]
    fn test_h3_to_geo() {
        let setup = Setup::new();

        assert_eq!(setup.h3_index.to_geo(), setup.geo_coord);
    }

    #[test]
    fn test_h3_to_geo_boundary() {
        // TODO
    }

    #[test]
    fn test_h3_resolution() {
        let setup = Setup::new();

        for res in 0..16 {
            let h = setup.geo_coord.to_h3(res).unwrap();
            assert_eq!(h.resolution(), res);
        }
    }

    #[test]
    fn test_h3_base_cell() {
        let setup = Setup::new();

        assert_eq!(setup.h3_index.base_cell(), 6);
    }

    #[test]
    fn test_geo_to_h3() {
        let setup = Setup::new();

        assert_eq!(setup.geo_coord.to_h3(5).unwrap(), setup.h3_index);
        assert!(setup.geo_coord.to_h3(-1).is_err());
        assert!(setup.geo_coord.to_h3(17).is_err());
    }
}
