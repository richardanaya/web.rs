#![no_std]
use js::*;
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;

struct Random {
    rng: StdRng,
}

impl Default for Random {
    fn default() -> Self {
        let random = register_function(
            "function(){
            return Math.random();
        }",
        );
        let rng: StdRng = SeedableRng::from_seed([
            (255.0 * random.invoke_0()) as u8,
            (255.0 * random.invoke_0()) as u8,
            (255.0 * random.invoke_0()) as u8,
            (255.0 * random.invoke_0()) as u8,
            (255.0 * random.invoke_0()) as u8,
            (255.0 * random.invoke_0()) as u8,
            (255.0 * random.invoke_0()) as u8,
            (255.0 * random.invoke_0()) as u8,
            (255.0 * random.invoke_0()) as u8,
            (255.0 * random.invoke_0()) as u8,
            (255.0 * random.invoke_0()) as u8,
            (255.0 * random.invoke_0()) as u8,
            (255.0 * random.invoke_0()) as u8,
            (255.0 * random.invoke_0()) as u8,
            (255.0 * random.invoke_0()) as u8,
            (255.0 * random.invoke_0()) as u8,
            (255.0 * random.invoke_0()) as u8,
            (255.0 * random.invoke_0()) as u8,
            (255.0 * random.invoke_0()) as u8,
            (255.0 * random.invoke_0()) as u8,
            (255.0 * random.invoke_0()) as u8,
            (255.0 * random.invoke_0()) as u8,
            (255.0 * random.invoke_0()) as u8,
            (255.0 * random.invoke_0()) as u8,
            (255.0 * random.invoke_0()) as u8,
            (255.0 * random.invoke_0()) as u8,
            (255.0 * random.invoke_0()) as u8,
            (255.0 * random.invoke_0()) as u8,
            (255.0 * random.invoke_0()) as u8,
            (255.0 * random.invoke_0()) as u8,
            (255.0 * random.invoke_0()) as u8,
            (255.0 * random.invoke_0()) as u8,
        ]);
        Random { rng: rng }
    }
}

impl Random {
    pub fn gen<T>(&mut self) -> T
    where
        rand::distributions::Standard: rand::distributions::Distribution<T>,
    {
        self.rng.gen::<T>()
    }
}

pub fn random() -> f64{
    globals::get::<Random>().gen()
}