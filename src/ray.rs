//ray.rs: 'laser' that our camera shoots out
//to figure out what color each pixel should be
use crate::vec3::{Point3, Vec3};
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

//Create implemetation for the Ray (constructor, at, getters)
impl Ray {
    //Equivalent to constructor with paramters in C++
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self {
            orig: origin,
            dir: direction,
        }
    }
    //Origin
    pub fn origin(&self) -> Point3 {
        self.orig
    }
    //Direction
    pub fn direction(&self) -> Vec3 {
        self.dir
    }
    //'at' funciton
    pub fn at(&self, t: f64) -> Point3 {
        self.orig + (self.dir * t)
    }
}
