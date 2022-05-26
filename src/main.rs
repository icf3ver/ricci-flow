use std::{u8, ops::{Mul, Neg}};

#[derive(Clone, Debug)]
struct Normals {
    xyz: Vec<f32>
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


#[derive(Clone, Copy, Debug)]
struct PointCloud<'a> {
    xyz: &'a [f32]
}
impl<'a> PointCloud<'a> {
    /// Place holder function (Triangular prisms, and pyramids should handle all edge cases)
    fn tri_prsm() -> PointCloud<'a> {
        PointCloud {
            xyz: &[
                f32::sqrt(3.0)/3.0, 0.0, 0.0,
                todo!("easy enough next commit")
            ]
        }
    }
}
impl<'a> Into<Mesh<'a>> for PointCloud<'a> {
    fn into(self) -> Mesh<'a> {
        todo!("tmr extract the faces trivial");
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
        todo!("trivial")
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
