use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Review {
    user: String,
    rating: u8,
    text: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ACUnit {
    name: String,
    desc: String,
    price: f64,
    reviews: Vec<Review>,
}

#[derive(Clone)]
pub struct AppState {
    units: Vec<ACUnit>,
}

impl ACUnit {
    pub fn new(name: String, desc: String, price: f64) -> Self {
        Self {
            name,
            desc,
            price,
            reviews: vec![],
        }
    }

    pub fn add_review(&mut self, user: String, rating: u8, text: String) {
        self.reviews.push(Review { user, rating, text });
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn desc(&self) -> &str {
        &self.desc
    }

    pub fn price(&self) -> f64 {
        self.price
    }

    pub fn rating(&self) -> f32 {
        let mut rating = 0.0;

        for review in &self.reviews {
            rating += review.rating as f32;
        }

        rating / self.reviews.len() as f32
    }

    pub fn reviews(&self) -> &Vec<Review> {
        &self.reviews
    }
}

impl AppState {
    pub fn new(units: Vec<ACUnit>) -> Self {
        Self { units }
    }

    pub fn units(&self) -> &Vec<ACUnit> {
        &self.units
    }
}
