use std::{u8, ops::{Mul, Neg}};

use feo_math::linear_algebra::vector3::Vector3;

#[derive(Clone, Debug)]
struct Normals {
    vec: Vec<[f32; 3]> // TODO: clean
}
impl Neg for Normals {
    type Output = Self;

    fn neg(self) -> Self::Output {
        todo!()
    }
}

#[derive(Clone, Debug)]
struct Curvature {
    crv: Vec<f32>
}

impl Mul<Curvature> for Normals {
    /// Vec of the translation to be done to each point
    type Output = Vec<f32>;

    fn mul(self, rhs: Curvature) -> Self::Output {
        todo!()
    }
}

impl Curvature {
    /// Returns the curvature of a face
    pub(crate) fn calc_crv(_pts: [f32; 9]) -> f32 {
        todo!();
    }

    /// Converts face curvature to vertex curvature
    pub fn face_to_vertex (&self) -> Self {
        todo!();
    }

    pub fn vertex_to_face (&self) -> Self {
        todo!();
    }
}


#[derive(Clone, Debug)]
struct PointCloud {
    xyz: Vec<f32>
}
impl PointCloud {
    // Note: Triangular prisms, and pyramids should handle all edge cases

    /// Initializes a point cloud for a triangular prism
    fn tri_prsm() -> Self {
        PointCloud {
            xyz: vec![
                f32::sqrt(3.0)/3.0, 0.0, 0.0,
                -f32::sqrt(3.0)/6.0, 0.5, 0.0,
                -f32::sqrt(3.0)/6.0, -0.5, 0.0,
                0.0, 0.0, f32::sqrt(2.0/3.0)
            ]
        }
    }

    /// Initializes a point cloud for a pyramid
    #[allow(dead_code)]
    fn rect_prsm() -> Self {
        PointCloud {
            xyz: vec![
                0.5, 0.5, 0.0,
                -0.5, 0.5, 0.0,
                -0.5, -0.5, 0.0,
                0.5, -0.5, 0.0,
                0.0, 0.0, f32::sqrt(2.0)/2.0
            ]
        }
    }
}
impl<'a> Into<Mesh<'a>> for PointCloud {
    fn into(self) -> Mesh<'a> {
        todo!("extract the faces");
    }
}

#[derive(Clone, Copy, Debug)]
struct Mesh<'a> {
    faces: &'a [f32] // A B C isolated for each face
}
impl<'a> Into<Curvature> for Mesh<'a> {
    fn into(self) -> Curvature {
        Curvature{
            crv: self.faces.chunks(9).into_iter().map(|a: &[f32]| 
                Curvature::calc_crv(a.try_into().unwrap())
            ).collect::<Vec<f32>>()
        }
    }
}
impl<'a> Into<Normals> for Mesh<'a> {
    fn into(self) -> Normals {
        // Counter Clockwise vertex ordering
        Normals{
            vec: {
                self.faces.chunks(9).into_iter().map(|a: &[f32]| {
                    // Smooth for now
                    let v1 = Vector3::from(TryInto::<[f32; 3]>::try_into(&a[6..9]).unwrap()) - Vector3::from(TryInto::<[f32; 3]>::try_into(&a[3..6]).unwrap()); //.normalize(None)
                    let v2 = Vector3::from(TryInto::<[f32; 3]>::try_into(&a[0..3]).unwrap()) - Vector3::from(TryInto::<[f32; 3]>::try_into(&a[3..6]).unwrap()); //.normalize(None)
                    
                    Vector3::cross_product(v1, v2).normalize(None).into()
                }).collect::<Vec<[f32; 3]>>()
            }
        }
    }
}

fn main() {
    let cloud = PointCloud::tri_prsm();

    let mesh = Into::<Mesh>::into(cloud);

    let curvature = Into::<Curvature>::into(mesh)
        .face_to_vertex();
    
    let normals = Into::<Normals>::into(mesh); // Skip conversion to mesh (Not necessary)

    let translation_vec = -normals * curvature;

    todo!("finish")
}
