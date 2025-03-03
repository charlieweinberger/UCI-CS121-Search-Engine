pub mod file_skip_list;
pub mod id_book;
pub mod index_builder;
pub mod inverted_index;
pub mod lazy_merger;
pub mod postings;
pub mod query;
pub mod single_posting;
pub mod tokenizer;
use actix_cors::Cors;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

// fn main() {
//     // ! BUILD INDEX
//     // index_builder::main();
//     // ! MERGE BATCHES
//     // ! The following code snippet merges the batches of inverted indexes into a single inverted index.
//     // lazy_merger::main();

//     // ! Comes searching and ranking now

//     println!("Welcome to the Search Engine!");
//     let mut search_engine = query::SearchEngine::new();
//     loop {
//         search_engine.get_query();
//         search_engine.search();
//     }
//     // TODO: Implement search and ranking logic using the query
// }

#[derive(Deserialize)]
struct SearchRequest {
    query: String,
}

#[derive(Serialize)]
struct SearchResponse {
    results: Vec<String>,
    time: u128,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Welcome to the Search Engine!");

    let search_engine = Arc::new(Mutex::new(query::SearchEngine::new()));

    HttpServer::new(move || {
        // Configure CORS middleware
        let cors = Cors::default()
            .allowed_origin("http://localhost:4321")
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(search_engine.clone()))
            .route("/", web::get().to(index))
            .route("/search", web::post().to(handle_search))
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Search Engine API")
}

async fn handle_search(
    payload: web::Json<SearchRequest>,
    search_engine: web::Data<Arc<Mutex<query::SearchEngine>>>,
) -> impl Responder {
    let mut engine = search_engine.lock().unwrap();

    // Set the query and perform search
    engine.set_query(payload.query.clone());
    let (results, time) = engine.search();

    // Limit to 5 results
    let limited_results = results.into_iter().take(5).collect();

    HttpResponse::Ok().json(SearchResponse {
        results: limited_results,
        time: time,
    })
}
