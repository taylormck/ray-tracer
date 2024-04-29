//! Perlin noise generator

use crate::vector::Vec3;
use rand::Rng;

#[derive(Debug, Copy, Clone)]
pub struct Perlin<const N: usize> {
    pool: [f64; N],
    perm_x: [usize; N],
    perm_y: [usize; N],
    perm_z: [usize; N],
}

impl<const N: usize> Perlin<N> {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();

        let mut new_generator = Self {
            pool: [0.0; N],
            perm_x: [0; N],
            perm_y: [0; N],
            perm_z: [0; N],
        };

        for i in 0..N {
            new_generator.pool[i] = rng.gen();
        }

        Self::perlin_generate_perm(&mut new_generator.perm_x);
        Self::perlin_generate_perm(&mut new_generator.perm_y);
        Self::perlin_generate_perm(&mut new_generator.perm_z);
        new_generator
    }

    pub fn noise(self: &Self, point: &Vec3) -> f64 {
        let point = *point * 4.0;

        let i = (point.x as i32 & 255) as usize;
        let j = (point.y as i32 & 255) as usize;
        let k = (point.z as i32 & 255) as usize;

        let index = self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k];

        self.pool[index]
    }

    fn perlin_generate_perm(perm: &mut [usize; N]) {
        for i in 0..N {
            perm[i] = i;
        }

        Self::permute(perm);
    }

    fn permute(perm: &mut [usize; N]) {
        let mut rng = rand::thread_rng();

        for i in 0..N {
            let target: usize = rng.gen_range(0..N);
            perm.swap(i, target);
        }
    }
}

pub type Perlin256 = Perlin<256>;
