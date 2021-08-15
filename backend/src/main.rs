use nanoid::nanoid;
use serde::Serialize;
use std::{collections::HashMap, convert::Infallible, sync::Arc};
use tokio::sync::RwLock;
use warp::Filter;

pub struct Room {}

impl Room {
    fn new() -> Self {
        Self {}
    }
}

pub struct GameData {
    rooms: HashMap<String, Room>,
}

impl GameData {
    fn new() -> Self {
        Self {
            rooms: HashMap::new(),
        }
    }
}

type Game = Arc<RwLock<GameData>>;

fn with_game(game: Game) -> impl Filter<Extract = (Game,), Error = Infallible> + Clone {
    warp::any().map(move || game.clone())
}

#[derive(Serialize)]
struct CreateRoomResponse {
    id: String,
}

async fn create_room(game: Game) -> Result<warp::reply::Json, Infallible> {
    let id = nanoid!();
    game.write().await.rooms.insert(id.clone(), Room::new());
    Ok(warp::reply::json(&CreateRoomResponse { id }))
}

#[tokio::main]
async fn main() {
    let game = Arc::new(RwLock::new(GameData::new()));

    let index = warp::path::end().map(|| "ferristype server v0.1.0");
    let room_create_route = warp::path!("room" / "create")
        .and(warp::post())
        .and(with_game(game))
        .and_then(create_room);

    let routes = index.or(room_create_route);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
