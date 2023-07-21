use std::sync::Arc;

use axum::extract::Path;
use axum::response::IntoResponse;
use axum::{routing::get, Router};
use axum::{Extension, Json};

use rand::rngs::SmallRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;

use tokio::sync::Mutex;

type SharedState = Arc<Mutex<SmallRng>>;

struct Lotto<'a> {
    pot: Vec<u32>,
    rng: &'a mut SmallRng,
}

impl<'a> Lotto<'a> {
    fn new(pot_size: u32, rng: &'a mut SmallRng) -> Self {
        Self {
            pot: (1..=pot_size).collect(),
            rng,
        }
    }

    fn take(&mut self, amount: usize) -> Vec<u32> {
        self.pot.shuffle(self.rng);
        self.pot
            .iter()
            .take(amount)
            .map(|el| el.to_owned())
            .collect()
    }
}

async fn generate_lotto_handler(
    Path((pot_size, amount)): Path<(u32, usize)>,
    Extension(state): Extension<SharedState>
) -> impl IntoResponse {
    let mut rng = state.lock().await;
    let mut lotto = Lotto::new(pot_size, &mut rng);
    let result = lotto.take(amount);
    Json(result)
}

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    let state = Arc::new(Mutex::new(SmallRng::from_entropy()));
    let router = Router::new()
        .route("/lotto/:pot/:amount", get(generate_lotto_handler))
        .layer(Extension(state));

    Ok(router.into())
}
