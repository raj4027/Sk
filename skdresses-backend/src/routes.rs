use actix_web::{web, HttpResponse, Responder};
use crate::models::{SignupData, LoginData, AuthResponse};
use bcrypt::{hash, verify};
use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Serialize};
use std::env;

#[derive(Serialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/signup", web::post().to(signup))
        .route("/login", web::post().to(login));
}

async fn signup(data: web::Json<SignupData>) -> impl Responder {
    let hashed = hash(&data.password, 4).unwrap();
    println!("Saving user: {} with hashed password: {}", data.email, hashed);
    HttpResponse::Ok().body("User registered")
}

async fn login(data: web::Json<LoginData>) -> impl Responder {
    let secret = env::var("SECRET_KEY").unwrap();
    let token = encode(
        &Header::default(),
        &Claims {
            sub: data.email.clone(),
            exp: 10000000000,
        },
        &EncodingKey::from_secret(secret.as_ref()),
    ).unwrap();

    HttpResponse::Ok().json(AuthResponse { token })
}

//Add cart endpoint

#[derive(Deserialize)]
pub struct CartItem {
    pub product_id: String,
    pub age_range: String,
    pub gender: String,
}

async fn add_to_cart(item: web::Json<CartItem>) -> impl Responder {
    let email = env::var("ALERT_EMAIL").unwrap();
    let phone = env::var("ALERT_PHONE").unwrap();

    // Simulate email
    println!(
        "New Cart Alert!\nProduct ID: {}\nAge: {}\nGender: {}\n-> Email: {}\n-> Phone: {}",
        item.product_id, item.age_range, item.gender, email, phone
    );

    send_email_alert(&item).await;

    HttpResponse::Ok().body("Item added and alert sent")
}

//Implement email sending
use lettre::{Message, SmtpTransport, Transport, transport::smtp::authentication::Credentials};

async fn send_email_alert(item: &CartItem) {
    let smtp_user = env::var("SMTP_USERNAME").unwrap();
    let smtp_pass = env::var("SMTP_PASSWORD").unwrap();

    let email = Message::builder()
        .from(smtp_user.parse().unwrap())
        .to(env::var("ALERT_EMAIL").unwrap().parse().unwrap())
        .subject("SKDresses - Cart Alert")
        .body(format!(
            "User added dress for Age: {}, Gender: {}, ID: {}",
            item.age_range, item.gender, item.product_id
        ))
        .unwrap();

    let creds = Credentials::new(smtp_user, smtp_pass);

    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    let _ = mailer.send(&email);
}

