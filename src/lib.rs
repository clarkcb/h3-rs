extern crate libc;

#[link(name = "h3")]
extern "C" {
    fn geoToH3(g: *const GeoCoordInternal, res: i32) -> u64;
    fn h3ToGeo(h3: u64, g: *mut GeoCoordInternal);
    fn h3ToGeoBoundary(h3: u64, gp: *mut GeoBoundaryInternal);
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
    /// Finds the centroid of the index.
    ///
    /// # Example
    ///
    /// ```
    /// TODO
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
    /// TODO
    /// ```
    pub fn to_geo_boundary(self) -> GeoBoundary {
        let mut gb = GeoBoundaryInternal::new();
        unsafe {
            h3ToGeoBoundary(self.0, &mut gb);
        }
        gb.convert()
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GeoCoordInternal {
    pub lat: f64,
    pub lon: f64,
}

impl GeoCoordInternal {
    pub fn new(lat: f64, lon: f64) -> GeoCoordInternal {
        GeoCoordInternal { lat, lon }
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
    pub fn new(lat: f64, lon: f64) -> GeoCoord {
        GeoCoord { lat, lon }
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
    /// // TODO: assert_eq!(coord.to_h3(5), H3Index(0x850dab63fffffff));
    /// ```
    pub fn to_h3(&self, res: i32) -> H3Index {
        self.to_radians().to_h3(res)
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct GeoBoundaryInternal {
    num_verts: i32,
    verts: [GeoCoordInternal; MAX_CELL_BNDRY_VERTS],
}

impl GeoBoundaryInternal {
    fn new() -> GeoBoundaryInternal {
        GeoBoundaryInternal {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_geo() {
        // TODO
    }

    #[test]
    fn test_to_geo_boundary() {
        // TODO
    }

    #[test]
    fn test_to_h3() {
        let geo = GeoCoord {
            lat: 67.194013596,
            lon: 191.598258018,
        };

        assert_eq!(geo.to_h3(5), H3Index(0x850dab63fffffff));
    }
}
