use std::ops::*;
use std::f64;

pub struct Vec3 {
    pub e : [f64; 3],
}

impl Vec3 {

    // Static functions
    pub fn new() -> Vec3{
        Vec3{e : [0.0, 0.0, 0.0] }
    }

    pub fn with_values(x : f64, y: f64, z: f64) -> Vec3{
        Vec3{ e : [x, y, z] }
    }

    pub fn dot(u: &Vec3, v: &Vec3) -> f64{
        u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
    }

    pub fn cross(u: &Vec3, v: &Vec3) -> Vec3{
        Vec3::with_values(
            u.e[1] * v.e[2] - u.e[2] * v.e[1],
            u.e[2] * v.e[0] - u.e[0] * v.e[2],
            u.e[0] * v.e[1] - u.e[1] * v.e[0]
        )
    }

    pub fn unit_vector(v: &Vec3) -> Vec3 {
        let length = v.len();
        Vec3::with_values(
            v[0] / length,
            v[1] / length,
            v[2] / length,
        )
    }

    // Instance functions
    fn length_squared(&self) -> f64 {
        self.e[0]*self.e[0] + self.e[1]*self.e[1] + self.e[2]*self.e[2]
    }

    pub fn len(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn to_string(&self) -> String {
        format!("{} {} {}", self.e[0], self.e[1], self.e[2])
    }

}

/*
    Traits to implements overloading
*/
impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3{
        Vec3 { e: [-self.e[0], -self.e[1], -self.e[2]] }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [self.e[0] + other.e[0],
                self.e[1] + other.e[1],
                self.e[2] + other.e[2]]
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [self.e[0] - other.e[0],
                self.e[1] - other.e[1],
                self.e[2] - other.e[2]]
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3){
        self.e[0] += other.e[0];
        self.e[1] += other.e[1];
        self.e[2] += other.e[2];
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3){
        self.e[0] -= other.e[0];
        self.e[1] -= other.e[1];
        self.e[2] -= other.e[2];
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, t : f64){
        self.e[0] *= t;
        self.e[1] *= t;
        self.e[2] *= t;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t : f64){
        self.e[0] /= t;
        self.e[1] /= t;
        self.e[2] /= t;
    }
}

// Operator to use index on the struct
impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, i: usize) -> &f64{
        &self.e[i]
    }
}

impl IndexMut<usize> for Vec3 {
    
    fn index_mut(&mut self, i: usize) -> &mut f64 {
        &mut self.e[i]
    }
}


#[cfg(test)]
mod test_vec3 {
    use super::*;

    #[test]
    fn test_new_empty(){
        let vector1 = Vec3::new();

        assert_eq!(vector1.e, [0.0,0.0,0.0]);
    }

    #[test]
    fn test_with_values(){
        let vector = Vec3::with_values(10.0, 15.0, 20.0);

        assert_eq!(vector.e, [10.0, 15.0, 20.0]);
    }

    #[test]
    fn test_negative_vector(){
        let vector = Vec3::with_values(10.0, 15.0, 20.0);
        let result = -vector;

        assert_eq!(result.e, [-10.0, -15.0, -20.0]);
    }

    #[test]
    fn test_adding_and_assign(){
        let mut vector = Vec3::with_values(10.0, 15.0, 20.0);
        let vector2 = Vec3::with_values(10.0, 15.0, 20.0);
        vector += vector2;

        assert_eq!(vector.e, [20.0, 30.0, 40.0]);
    }

    #[test]
    fn test_sub_and_assign(){
        let mut vector = Vec3::with_values(10.0, 15.0, 20.0);
        let vector2 = Vec3::with_values(10.0, 15.0, 20.0);
        vector -= vector2;

        assert_eq!(vector.e, [0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_mul_and_assign(){
        let mut vector = Vec3::with_values(10.0, 15.0, 20.0);
        vector *= 5.0;

        assert_eq!(vector.e, [50.0, 75.0, 100.0]);
    }

    #[test]
    fn test_div_and_assign(){
        let mut vector = Vec3::with_values(10.0, 15.0, 20.0);
        vector /= 5.0;

        assert_eq!(vector.e, [2.0, 3.0, 4.0]);
    }

    #[test]
    fn test_index(){
        let vector = Vec3::with_values(10.0, 15.0, 20.0);
        let x = vector[0];

        assert_eq!(x, 10.0);
    }

    #[test]
    fn test_mut_index(){
        let mut vector = Vec3::with_values(10.0, 15.0, 20.0);
        vector[0] = 0.0;

        assert_eq!(vector[0], 0.0);
    }

    #[test]
    fn test_length_square(){
        let vector = Vec3::with_values(2.0, 2.0, 2.0);
        let square_length = vector.length_squared();

        assert_eq!(square_length, 12.0);
    }

    #[test]
    fn test_length(){
        let vector = Vec3::with_values(1.0, 1.0, 1.0);
        let len = vector.len();

        assert!(len - 1.7 < 0.1);
    }

}