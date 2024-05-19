use std::sync::Mutex;
use actix_web::{App, HttpServer};
use actix_web::web::{Data, get, post};

struct Messenger{
    message: String
}

struct MutableState {
    messenger: Mutex<Messenger>
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = Data::new(MutableState {
        messenger: Mutex::new(Messenger {message: "Hello".to_string()})
    });

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .route("/", post().to(do_update_app_data))
            .route("/", get().to(get_app_data))
    })
        .bind(("127.0.0.1",8080))?
        .run()
        .await
}

async fn do_update_app_data(app_data: Data<MutableState>) -> &'static str {
    let mut messenger = app_data.messenger.lock().unwrap();
    messenger.message = format!("{} world!", messenger.message);
    "updated!"
}

async fn get_app_data(app_data: Data<MutableState>) -> String {
    app_data.messenger.lock().unwrap().message.clone()
}
