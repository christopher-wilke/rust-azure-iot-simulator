use std::fmt;

use rand::{thread_rng, Rng};

const MIN_TEMP: f32 = 20.0;
const MAX_TEMP: f32 = 25.0;

#[derive(Debug, Clone)]
pub struct Temperature {
    pub value: f64,
}

impl Default for Temperature {
    fn default() -> Self {
        Self { value: 0. }
    }
}

impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", &self.value)
    }
}

pub fn get_new_item() -> Temperature {
    let mut rng = thread_rng();
    let value = rng.gen_range(MIN_TEMP..MAX_TEMP).into();

    Temperature { value }
}
