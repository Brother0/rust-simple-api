mod database;
mod handlers;
mod models;

use crate::database::Database;
use crate::handlers::*;
use crate::models::*;

use iron::prelude::Chain;
use iron::Iron;
use logger::Logger;
use router::Router;
use uuid::Uuid;

fn main() {
    env_logger::init();
    let (logger_before, logger_after) = Logger::new(None);

    let mut db = Database::new();
    let p = Post::new(
        "The First Post",
        "Hi from post Body. Blah Blah...",
        "Brother0",
        chrono::offset::Utc::now(),
        Uuid::new_v4()
    );
    db.add_post(p);

    let p2 = Post::new(
        "The Second Post",
        "Blah Blah...",
        "Brother0",
        chrono::offset::Utc::now(),
        Uuid::new_v4()
    );
    db.add_post(p2);

    let handlers = Handlers::new(db);
    let json_content_middleware = JsonAfterMiddleware;

    let mut router = Router::new();
    router.get("/post_feed", handlers.post_feed, "post_feed");
    router.post("/post", handlers.post_post, "post_post");
    router.get("/post/:id", handlers.post, "post");

    let mut chain = Chain::new(router);
    chain.link_before(logger_before);
    chain.link_after(json_content_middleware);
    chain.link_after(logger_after);

    Iron::new(chain).http("localhost:8000").unwrap();
}
