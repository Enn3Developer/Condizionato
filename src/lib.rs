use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Review {
    user: String,
    rating: u8,
    text: String,
}

#[derive(Serialize, Deserialize)]
pub struct ACUnit {
    name: String,
    desc: String,
    short_desc: String,
    image: String,
    price: f64,
    reviews: Vec<Review>,
}

#[derive(Serialize, Deserialize)]
pub struct AppState {
    units: Vec<ACUnit>,
}

impl Review {
    pub fn html(&self) -> String {
        format!(
            "<p>{}      {:0.2}⭐</p><br><p>{}</p><br>",
            self.user, self.rating, self.text
        )
    }
}

impl ACUnit {
    pub fn new(name: String, desc: String, short_desc: String, image: String, price: f64) -> Self {
        Self {
            name,
            desc,
            short_desc,
            image,
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

    pub fn short_desc(&self) -> &str {
        &self.short_desc
    }

    pub fn image(&self) -> &str {
        &self.image
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

    pub fn card(&self, index: usize) -> String {
        format!(
            "<a href=\"/units/{}\" class=\"card_link\">
                <div class=\"card\">
                    <div class=\"card_img_container\">
                        <img src=\"images/{}\" alt=\"{}\" class=\"card_img\">
                        <div class=\"top_right_card_img\">
                            <p>{:0.2}⭐</p>
                        </div>
                    </div>
                    <div class=\"container\">
                        <p>{}</p>
                        <br>
                        <p>{}</p>
                        <br>
                        <p>€{:0.2}</p>
                    </div>
                </div>
            </a>",
            index,
            self.image,
            self.name,
            self.rating(),
            self.name,
            self.short_desc,
            self.price,
        )
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
