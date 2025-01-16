// use std::sync::mpsc;
use tray_item::{IconSource, TrayItem};
use std::process;
use actix_web::{web, App, HttpResponse, HttpServer, Responder, post, get};
use serde::Deserialize;
use win_toast_notify::{WinToastNotify,CropCircle};
use tokio::runtime::Runtime;
use std::env;

// enum Message {
//     Quit,
// }

#[derive(Deserialize)]
struct NotificationData {
    title: Option<String>,
    body_message: Option<String>,
}

fn show_notification(title: String, body_message: String) {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let logo_path = current_dir.join("assets/notificationlogo.png");
    WinToastNotify::new()
        .set_title(&title)
        .set_logo(logo_path.to_str().expect("Path is an invalid unicode"), CropCircle::True)
        .set_messages(vec![&body_message])
        .show()
        .expect("Failed to show toast notification");
}


#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("Notification server is online")
}
#[get("/notification")]
async fn get_notification() -> impl Responder {
    HttpResponse::Ok().body("Wrong method")
}
#[post("/notification")]
async fn post_notification(notification: web::Json<NotificationData>) -> impl Responder {
    let title: String = notification.title.clone().unwrap_or_else(|| "Notification request received".to_string());
    let body_message: String = notification.body_message.clone().unwrap_or_else(|| "Check calendar for more information".to_string());

    // Run the notification in a separate thread
    show_notification(title, body_message);

    HttpResponse::Ok().body("Notification triggered")
}

fn start_http_server() {
    // Create a tokio runtime and block on the Actix server
    let rt: Runtime = Runtime::new().unwrap();
    rt.block_on(async {
        HttpServer::new(|| {
            App::new()
            .service(home)
            .service(get_notification)
            .service(post_notification)
        })
        .bind("0.0.0.0:8081")
        .expect("Failed to bind server")
        .run()
        .await
        .unwrap();
    });
}



fn main() {
    // Start the Actix HTTP server in a separate thread
    std::thread::spawn(move || {
        start_http_server();
    });

    let mut tray: TrayItem = TrayItem::new(
        "Notify Listener",
        IconSource::Resource("tray-icon"),
    )
    .unwrap();

    // for multiple options
    // let (tx, rx) = mpsc::sync_channel(1);
    // let quit_tx = tx.clone();

    tray.add_menu_item("Quit", move || {
        // quit_tx.send(Message::Quit).unwrap();
        process::exit(0); // Exit the program
    })
    .unwrap();

    // loop {
    //     match rx.recv() {
    //         Ok(Message::Quit) => {
    //             println!("Quit");
    //             process::exit(0); // Exit the program
    //         }
    //         _ => {}
    //     }
    // }
    //
    // Park the main thread to keep the tray icon active
    std::thread::park(); // This will keep the main thread alive

}