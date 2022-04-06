use mongodb::{
    bson::doc,
    sync::Client, event::command::CommandEventHandler,
};

use serde::{Deserialize, Serialize};
use actix_cors::Cors;
use actix_web::{get, web, App, HttpServer, Responder, http};
use bson::{Bson};

#[derive(Debug, Serialize, Deserialize)]
struct Date{
    gameid: i64,
    players: i64,
    winner:  String,
    gamename:  String,
    queue:  String,
    fecha:  String,
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[derive(Serialize)]
struct Country {
    country_code: String,
    country_name: String
}

async fn get_country_list()  -> impl Responder {
    let mut vec:Vec<Date> = Vec::new();

    // CREO LA CONEXION CON EL SERVER
    let client = Client::with_uri_str("mongodb://admin:pass123@34.125.197.46:27017").unwrap();
    // OBTENGO LA BASE DE DATOS
    let db = client.database("SO1_Proyecto1_Fase2");
    let coll= db.collection::<Date>("Game_Logs"); 
    let cursor = coll.find(None, None).unwrap();

    //vec.push(Date{code: "MY".to_string(), name: "Malaysia".to_string(), Ver:"holi".to_string()});
    for result in cursor{
        if let Ok(item) = result{
            vec.push(item);
        }
        //println!("{:?}",result);
    }
 
    //vec.push(Country{country_code: "PH".to_string(), country_name: "Philippines".to_string()});
    //vec.push(Country{country_code: "MY".to_string(), country_name: "Malaysia".to_string()});
    //vec.push(Country{country_code: "ID".to_string(), country_name: "Indonesia".to_string()});
 
    return web::Json(vec);
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    //println!("{:?}",cursor);
    /*for result in cursor{
        println!("{:?}",result);
    }*/
    //println!("Hello, world!");
    HttpServer::new(|| {

        let cors = Cors::default()//.supports_credentials() 
                .allowed_origin("http://localhost:8080")
                .allowed_methods(vec!["GET", "POST"])
                .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                .allowed_header(http::header::CONTENT_TYPE)
                .max_age(3600);

        App::new()
            .wrap(cors)
            .route("/hello", web::get().to(get_country_list))
            .service(greet)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

/*fn build_data(
    code: String,
    name: String,
) -> Date {
    Date {
        code,
        name,
    }
}

fn user_from_document(document: Document) -> Date {
    let mut _first_name = "".to_string();
    let mut _last_name = "".to_string();
    let mut _email = "".to_string();
    let mut _user_name = "".to_string();
    let mut _password = "".to_string();
    if let Some(&Bson::String(ref first_name)) = document.get("firstName") {
        _first_name = first_name.to_string();
    }
    if let Some(&Bson::String(ref last_name)) = document.get("lastName") {
        _last_name = last_name.to_string();
    }
    if let Some(&Bson::String(ref email)) = document.get("email") {
        _email = email.to_string();
    }
    if let Some(&Bson::String(ref user_name)) = document.get("username") {
        _user_name = user_name.to_string();
    }
    if let Some(&Bson::String(ref password)) = document.get("password") {
        _password = password.to_string();
    }

 build_user(_first_name, _last_name, _email, _user_name, _password)
}*/