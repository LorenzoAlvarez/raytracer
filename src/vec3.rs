use std::ops::*;

pub struct Vec3 {
    pub e : [f64; 3]
}

impl Vec3 {

    pub fn new() -> Vec3{
        Vec3{e : [0.0, 0.0, 0.0] }
    }

    pub fn with_values(x : f64, y: f64, z: f64) -> Vec3{
        Vec3{ e : [x, y, z] }
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

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, t : f64){
        self.e[0] *= t;
        self.e[1] *= t;
        self.e[2] *= t;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t : f64){
        self.e[1] /= t;
        self.e[2] /= t;
        self.e[3] /= t;
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



}