use super::{NormalizedF32, Path2};
use spade::handles::VoronoiVertex::Inner;
use spade::{DelaunayTriangulation, Triangulation};

pub type SpadeTriangulation = spade::DelaunayTriangulation<spade::Point2<f32>>;
pub trait SpadePoint2Extension {
    fn to_nannou_pt2(&self) -> nannou::prelude::Point2;
}

impl SpadePoint2Extension for spade::Point2<f32> {
    fn to_nannou_pt2(&self) -> nannou::prelude::Point2 {
        nannou::prelude::pt2(self.x, self.y)
    }
}

pub trait SpadeNannouPoint2Extension {
    fn to_spade_pt2(&self) -> spade::Point2<f32>;
}

impl SpadeNannouPoint2Extension for nannou::prelude::Point2 {
    fn to_spade_pt2(&self) -> spade::Point2<f32> {
        spade::Point2::new(self.x, self.y)
    }
}

pub trait TriangulationExtension {
    fn polygons(&self) -> Vec<VoronoiPolygon>;
}

impl TriangulationExtension for SpadeTriangulation {
    fn polygons(&self) -> Vec<VoronoiPolygon> {
        self.voronoi_faces()
            .map(|face| {
                let centroid = face.as_delaunay_vertex().position().to_nannou_pt2();
                let points = face
                    .adjacent_edges()
                    .filter_map(|edge| match edge.from() {
                        // from is an inner face of a Delaunay triangulation.
                        // We get the actual point by calling circumcenter().
                        Inner(from) => {
                            let point = from.circumcenter().to_nannou_pt2();
                            Some(point)
                        }
                        _ => None,
                    })
                    .collect();

                VoronoiPolygon { centroid, points }
            })
            .collect()
    }
}

pub struct VoronoiPolygon {
    pub centroid: nannou::prelude::Point2,
    pub points: Path2,
}

impl VoronoiPolygon {
    // The closer amount is to 0.0, the closer the resulting points
    // are to the centroid of the polygon.
    pub fn extruded_points<GetExtrusionAmount>(
        &self,
        get_extrusion_amount: GetExtrusionAmount,
    ) -> Path2
    where
        GetExtrusionAmount: Fn(&nannou::prelude::Point2) -> NormalizedF32,
    {
        self.points
            .iter()
            .map(|point| {
                let amount = get_extrusion_amount(point);
                self.centroid.lerp(*point, amount)
            })
            .collect()
    }
}
