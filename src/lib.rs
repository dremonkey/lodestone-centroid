///
/// ## Overview
/// 
/// Centroid implementations for FeaturePolygon and FeatureLineString.
///
///  and [L.Polyline::getCenter](https://github.com/Leaflet/Leaflet/blob/cca6e6165fbb0e2c543336bdcc976fc8f82db20a/src/layer/vector/Polyline.js).

// Third party crates
extern crate lodestone_along;
extern crate lodestone_line_distance;
extern crate lodestone_linestring;
extern crate lodestone_point;
extern crate lodestone_polygon;

use lodestone_along::Along;
use lodestone_line_distance::LineDistance;
use lodestone_linestring::FeatureLineString;
use lodestone_point::FeaturePoint;
use lodestone_polygon::FeaturePolygon;

pub trait Centroid {
  fn centroid(&self) -> FeaturePoint;
}

impl Centroid for FeaturePolygon {
  /// Calculates the centroid of a FeaturePolygon. This only utilizes the 
  /// outer ring if there are multiple. Usable for non-intersecting polygons.
  /// 
  /// Inspired by [L.Polygon::getCenter](https://github.com/Leaflet/Leaflet/blob/cca6e6165fbb0e2c543336bdcc976fc8f82db20a/src/layer/vector/Polygon.js)
  fn centroid(&self) -> FeaturePoint {

    let mut area = 0.0;
    let mut x_sum = 0.0; 
    let mut y_sum = 0.0;

    let mut ring = self.coordinates().first().unwrap().to_vec();
    let mut prev = ring.remove(0);
    
    for coord in ring {
      let f = coord[1] * prev[0] - prev[1] * coord[0];
      
      x_sum += (coord[0] + prev[0]) * f;
      y_sum += (coord[1] + prev[1]) * f;
      area += f * 3.0;

      // set up for the next iteration
      prev = coord.clone();
    }

    FeaturePoint::new(vec![x_sum / area, y_sum / area])
  }
}

impl Centroid for FeatureLineString {
  /// Calculates the centroid of a FeatureLineString. This only utilizes the 
  /// outer ring if there are multiple. 
  fn centroid(&self) -> FeaturePoint {
    let half_distance = self.distance("m") / 2.0;
    self.along(half_distance, "m")
  }
}

#[cfg(test)]
mod tests {

  mod tests_poly {
    use lodestone_point::FeaturePoint;
    use lodestone_polygon::FeaturePolygon;
    use super::super::Centroid;

    #[test]
    fn test_small_square() {
      let ring = vec![vec![0.0, 0.0], vec![0.01, 0.0], vec![0.01, 0.01], vec![0.0, 0.01], vec![0.0, 0.0]];
      let expected = vec![0.005, 0.005];
      run_poly_test(ring, expected);
    }

    #[test]
    fn test_square() {
      let ring = vec![vec![0.0, 0.0], vec![0.0, 2.0], vec![2.0, 2.0], vec![2.0, 0.0], vec![0.0, 0.0]];
      let expected = vec![1.0, 1.0];
      run_poly_test(ring, expected);
    }

    #[test]
    fn test_triangle_90() {
      let ring = vec![vec![0.0, 0.0], vec![0.0, 2.0], vec![2.0, 0.0], vec![0.0, 0.0]];
      let expected = vec![2.0/3.0, 2.0/3.0];
      run_poly_test(ring, expected);
    }

    #[test]
    fn test_triangle_135() {
      let ring = vec![vec![1.0, 0.0], vec![0.0, 2.0], vec![2.0, 0.0], vec![1.0, 0.0]];
      let expected = vec![1.0, 2.0/3.0];
      run_poly_test(ring, expected);
    }

    // Helper method to test a polygon's centroid against an expected value
    fn run_poly_test(ring: Vec<Vec<f64>>, expected: Vec<f64>) -> () {
      let poly = FeaturePolygon::new(vec![ring]);
      let expected = FeaturePoint::new(expected);

      assert_eq!(poly.centroid(), expected);
    }
  }

  mod tests_line {
    use lodestone_point::FeaturePoint;
    use lodestone_linestring::FeatureLineString;
    use super::super::Centroid;

    #[test]
    fn test_simple() {
      let coords = vec![vec![0.0, 0.0], vec![0.0, 1.0], vec![1.0, 1.0], vec![1.0, 2.0]];
      
      // `expected` is not exactly [0.5, 1.0] as might be intuited
      // but within the expected range
      let expected = vec![0.49999999, 1.0000380];
      run_line_test(coords, expected);
    }

    fn run_line_test(coords: Vec<Vec<f64>>, expected: Vec<f64>) -> () {
      let line = FeatureLineString::new(coords);
      let expected = FeaturePoint::new(expected);

      assert_eq!(line.centroid(), expected);
    }
  }
}
