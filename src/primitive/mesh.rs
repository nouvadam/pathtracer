use crate::hit::*;
use crate::hitables::{BvhNode, HitableList, AABB};
use crate::material::Material;
use crate::primitive::triangle::*;
use crate::ray::*;
use crate::V3;
/// Mesh of triangles, or polygon model
#[derive(Clone)]
pub struct Mesh {
    triangles: Box<BvhNode>,
    material: Box<dyn Material + Sync + Send>,
    bounding_box: AABB,
}

impl Hitable for Mesh {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        self.triangles.hit(r, t_min, t_max)
    }

    fn bounding_box(&self) -> AABB {
        self.bounding_box.clone()
    }
}

impl Mesh {
    /// Returns new polygon model, if it could be made.
    ///
    /// `file_path` - Path to the .obj file.
    ///
    /// `material` - Material of the model.
    pub fn new(
        file_path: &str,
        material: Box<dyn Material + Sync + Send>,
    ) -> Result<Mesh, std::io::Error> {
        use std::fs;
        let file_to_parse = fs::read_to_string(file_path)?;

        let mut triangles_list = HitableList::new();
        let obj_file =
            wavefront_obj::obj::parse(file_to_parse).expect("MESH CREATION FAILED. WE ARE DOOMED");

        for object in obj_file.objects {
            for primitive in object.geometry {
                for shape in primitive.shapes.iter() {
                    //.take(700) {
                    match shape.primitive {
                        wavefront_obj::obj::Primitive::Triangle(x0, x1, x2) => {
                            triangles_list.add(Box::new(Triangle::new(
                                V3::new(
                                    object.vertices[x0.0].into(),
                                    object.vertices[x1.0].into(),
                                    object.vertices[x2.0].into(),
                                ),
                                match x0.2 {
                                    Some(_normal) => Some(V3::new(
                                        object.normals[x0.2.unwrap()].into(),
                                        object.normals[x1.2.unwrap()].into(),
                                        object.normals[x2.2.unwrap()].into(),
                                    )),
                                    None => None,
                                },
                                material.clone(),
                            )));
                        }
                        _ => {}
                    }
                }
            }
        }

        let bounding_box = triangles_list.bounding_box();

        Ok(Mesh {
            triangles: Box::new(BvhNode::new(triangles_list)),
            material,
            bounding_box,
        })
    }
}

impl From<wavefront_obj::obj::Vertex> for V3<f32> {
    fn from(vertex: wavefront_obj::obj::Vertex) -> V3<f32> {
        V3::new(vertex.x as f32, vertex.y as f32, vertex.z as f32)
    }
}
