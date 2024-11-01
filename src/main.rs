use std::borrow::Borrow;
use std::env::args as std_args;
use std::fs::File;
use std::io::BufReader;

use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use rustls::{pki_types::PrivateKeyDer, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};
use serde::Serialize;

mod data_generator;
mod sql_operator;

#[derive(Debug, Serialize)]
struct MessageResponse<'a> {
    message: &'a str,
}

static VERSION: &'static str = "5.0.2/20241101";

mod all_methods_v1 {
    pub mod gets_v1 {
        use actix_web::{HttpResponse, Responder, web};

        use crate::MessageResponse;

        pub mod item_methods_v1 {
            use actix_web::{guard, HttpResponse, Responder, web};
            use serde::Serialize;

            use crate::data_generator::RandomGenerator;
            use crate::MessageResponse;

            pub async fn get_item(item_name: web::Path<String>) -> impl Responder {
                let mut random_generator: RandomGenerator = RandomGenerator::new();
                #[derive(Debug, Serialize)]
                struct Response<'a> {
                    message: &'a str,
                    value: String,
                }
                let hm: [String; 5] = ["account".into(), "passwordLv1".into(), "passwordLv2".into(),
                    "passwordLv3".into(), "passwordLvMax".into()];

                if !hm.contains(&item_name) {
                    HttpResponse::Ok().json(Response { message: "Mismatch your item name.", value: "".into() })
                } else {
                    let res = match item_name.as_str() {
                        "account" => random_generator.get_account(),
                        "passwordLv1" => random_generator.get_password_1(),
                        "passwordLv2" => random_generator.get_password_2(),
                        "passwordLv3" => random_generator.get_password_3(),
                        "passwordLvMax" => random_generator.get_password_max(),
                        _ => "".into()
                    };
                    HttpResponse::Ok().json(Response { message: "succ", value: res })
                }
            }

            pub async fn not_found() -> impl Responder {
                HttpResponse::NotFound().json(MessageResponse { message: "No Data found." })
            }

            pub async fn get_items() -> impl Responder {
                let mut random_generator: RandomGenerator = RandomGenerator::new();
                #[derive(Debug, Serialize)]
                struct Result {
                    account: String,
                    password_lv1: String,
                    password_lv2: String,
                    password_lv3: String,
                    password_lv_max: String,
                }
                #[derive(Debug, Serialize)]
                struct Response<'a> {
                    message: &'a str,
                    value: Result,
                }
                HttpResponse::Ok().json(Response {
                    message: "succ",
                    value: Result {
                        account: random_generator.get_account(),
                        password_lv1: random_generator.get_password_1(),
                        password_lv2: random_generator.get_password_2(),
                        password_lv3: random_generator.get_password_3(),
                        password_lv_max: random_generator.get_password_max(),
                    },
                })
            }

            pub fn get_score() -> actix_web::Scope {
                web::scope("item_methods")
                    .service(
                        web::resource("/getItem/{itemName}")
                            .guard(guard::Get())
                            .route(web::get().to(get_item))

                            .default_service(
                                web::route().to(not_found)
                            )
                    )
                    .service(
                        web::resource("/getItems")
                            .guard(guard::Get())
                            .route(web::get().to(get_items))
                    )
                    .default_service(web::route().to(not_found))
            }
        }

        pub mod base_info_v1 {
            use actix_web::{guard, HttpResponse, Responder, web};

            use crate::{MessageResponse, VERSION};

            pub async fn get_version() -> impl Responder {
                HttpResponse::Ok().json(MessageResponse { message: VERSION })
            }

            pub fn get_score() -> actix_web::Scope {
                web::scope("base_info")
                    .service(
                        web::resource("/getVersion")
                            .guard(guard::Get())
                            .route(web::get().to(get_version))
                    )
            }
        }

        pub async fn not_found() -> impl Responder {
            HttpResponse::NotFound().json(MessageResponse { message: "No module found." })
        }

        pub fn get_score() -> actix_web::Scope {
            web::scope("gets_v1")
                .service(
                    item_methods_v1::get_score()
                )
                .service(
                    base_info_v1::get_score()
                )
                .default_service(web::route().to(not_found))
        }
    }

    pub mod posts_v1 {
        use actix_web::{HttpResponse, Responder, web};

        use crate::MessageResponse;

        pub mod item_methods_v1 {
            use std::borrow::Borrow;
            use std::sync::Mutex;
            use actix_web::{guard, HttpResponse, Responder, web};
            use base64::Engine;
            use base64::prelude::BASE64_STANDARD;

            use serde::Serialize;

            use crate::MessageResponse;
            use crate::sql_operator::{Data as SQLOperatorData, SQLOperator};

            #[derive(Debug, Serialize)]
            struct Response<'a> {
                message: &'a str,
                response: ResType,
            }

            #[derive(Debug, Serialize, Clone)]
            enum ResType {
                Changed(usize),
                SearchResult(Vec<SQLOperatorData>)
            }

            static mut CACHED: Mutex<Vec<(String, usize, ResType)>> = Mutex::new(Vec::new());
            static CACHED_SIZE: usize = 15;

            pub async fn save_item(paras: web::Form<SQLOperatorData>) -> impl Responder {
                let sql_oper = SQLOperator::new();
                match sql_oper.add_item(
                    &SQLOperator::Data_of(
                        BASE64_STANDARD.encode(paras.address.as_bytes()),
                        BASE64_STANDARD.encode(paras.account.as_bytes()),
                        BASE64_STANDARD.encode(paras.password.as_bytes()),
                        paras.email.clone(),
                        "".into(),
                        paras.text.clone())
                ) {
                    Ok(d) => unsafe {
                        CACHED.lock().unwrap().clear();
                        HttpResponse::Ok().json(Response { message: "succ", response: ResType::Changed { 0: d } })
                    }
                    Err(_) => { HttpResponse::Ok().json(Response { message: "err", response: ResType::Changed { 0: 0 } }) }
                }
            }

            pub async fn search_item(paras: web::Form<SQLOperatorData>) -> impl Responder {
                let sql_oper = SQLOperator::new();
                let mut key: String = "".into();
                let mut keyword: String = "".into();
                let mut status = false;
                if paras.address != "" {
                    keyword = BASE64_STANDARD.encode(&paras.address);
                    key = "Address".into();
                    status = true;
                } else if paras.account != "" {
                    keyword = BASE64_STANDARD.encode(&paras.account);
                    key = "Account".into();
                    status = true;
                } else if paras.password != "" {
                    keyword = BASE64_STANDARD.encode(&paras.password);
                    key = "Password".into();
                    status = true;
                } else if paras.email != "" {
                    keyword = paras.email.clone();
                    key = "Email".into();
                    status = true;
                } else if paras.text != String::from("") {
                    keyword = paras.text.replace("%", "");
                    key = "Text".into();
                    if keyword.len() != 0 {
                        status = true;
                    }
                }
                let cache_str = format!("search_item-{}%{}", key, keyword);

                let (r, s): (usize, bool) = unsafe {
                    match CACHED.lock().unwrap().iter().position(|x| x.0 == cache_str) {
                        None => (0, false),
                        Some(r) => (r, true)
                    }
                };

                if s == true {
                    unsafe {
                        let mut cached_unwrap = CACHED.lock().unwrap();
                        let a: &mut (String, usize, ResType) = cached_unwrap.get_mut(r).unwrap();
                        a.1 += 1;
                        if a.1 % 10 == 0 {
                            for e in CACHED.lock().unwrap().iter_mut() {
                                (*e).1 -= 2;
                            }
                        }
                        HttpResponse::Ok().json(Response {
                            message: "ok",
                            response: (*a).2.clone(),
                        })
                    }
                } else if status {
                    match sql_oper.search_item(key, keyword) {
                        Ok(mut r) => {
                            for i in 0..r.len() {
                                let x = r[i].borrow();
                                r[i] = SQLOperatorData {
                                    address: String::from_utf8(BASE64_STANDARD.decode(&x.address).unwrap()).unwrap(),
                                    account: String::from_utf8(BASE64_STANDARD.decode(&x.account).unwrap()).unwrap(),
                                    password: String::from_utf8(BASE64_STANDARD.decode(&x.password).unwrap()).unwrap(),
                                    email: x.email.clone(),
                                    date: x.date.clone(),
                                    text: x.text.clone(),
                                };
                            }
                            unsafe {
                                if CACHED.lock().unwrap().len() < CACHED_SIZE {
                                    CACHED.lock().unwrap().push((cache_str, 1, ResType::SearchResult { 0: r.to_vec() }));
                                } else {
                                    CACHED.lock().unwrap().sort_by(|x, y| y.1.cmp(&x.1));
                                    CACHED.lock().unwrap().pop();
                                    CACHED.lock().unwrap().push((cache_str, 1, ResType::SearchResult { 0: r.to_vec() }));
                                }
                            }
                            HttpResponse::Ok().json(Response { message: "succ", response: ResType::SearchResult { 0: r } })
                        }
                        Err(e) => {
                            eprintln!("{}", e.to_string());
                            HttpResponse::InternalServerError().json(Response {
                                message: "server have some problems.",
                                response: ResType::SearchResult { 0: Vec::with_capacity(0) },
                            })
                        }
                    }
                } else {
                    HttpResponse::Forbidden().json(Response {
                        message: "No legal argument(address, account, password, text) in request.",
                        response: ResType::SearchResult { 0: Vec::with_capacity(0) },
                    })
                }
            }

            pub async fn delete_item(paras: web::Form<SQLOperatorData>) -> impl Responder {
                let sql_oper = SQLOperator::new();
                let date: String = paras.date.to_string();
                if date == "" {
                    HttpResponse::BadRequest().json(Response { message: "miss date.", response: ResType::Changed { 0: 0 } })
                } else {
                    match sql_oper.remove_item(date) {
                        Ok(d) => unsafe {
                            CACHED.lock().unwrap().clear();
                            HttpResponse::Ok().json(Response { message: "succ", response: ResType::Changed { 0: d.0 } })
                        }
                        Err(_) => { HttpResponse::Ok().json(Response { message: "err", response: ResType::Changed { 0: 0 } }) }
                    }
                }
            }

            pub async fn update_item(paras: web::Form<SQLOperatorData>) -> impl Responder {
                let sql_oper = SQLOperator::new();
                if paras.date == "" {
                    HttpResponse::BadRequest().json(MessageResponse { message: "miss date." })
                } else {
                    match sql_oper.update_item(paras.text.to_string(), paras.date.to_string()) {
                        Ok(_) => { HttpResponse::Ok().json(MessageResponse { message: "succ" }) }
                        Err(_) => { HttpResponse::Ok().json(MessageResponse { message: "err" }) }
                    }
                }
            }

            pub async fn not_found() -> impl Responder {
                HttpResponse::NotFound().json(MessageResponse { message: "No method found." })
            }

            pub fn get_score() -> actix_web::Scope {
                web::scope("item_methods")
                    .service(
                        web::resource("/saveItem")
                            .guard(guard::Post())
                            .route(web::post().to(save_item))
                    )
                    .service(
                        web::resource("/searchItem")
                            .guard(guard::Post())
                            .route(web::post().to(search_item))
                    )
                    .service(
                        web::resource("/deleteItem")
                            .guard(guard::Post())
                            .route(web::post().to(delete_item))
                    )
                    .service(
                        web::resource("/updateItem")
                            .guard(guard::Post())
                            .route(web::post().to(update_item))
                    )
                    .default_service(web::route().to(not_found))
            }
        }

        pub async fn not_found() -> impl Responder {
            HttpResponse::NotFound().json(MessageResponse { message: "No module found." })
        }

        pub fn get_score() -> actix_web::Scope {
            web::scope("posts_v1")
                .service(
                    item_methods_v1::get_score()
                )
                .default_service(web::route().to(not_found))
        }
    }
}

async fn method_not_found() -> impl Responder {
    HttpResponse::NotFound().json(MessageResponse { message: "No request method found." })
}

// #[actix_web::main]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = std_args().collect();
    let mut ip: String = "127.0.0.1".into();
    let mut port = 8000u16;
    let (mut crt_file, mut key_file) = ("", "");
    let mut i = 1;
    // argv parse
    while i < args.len() {
        match args.get(i).unwrap().borrow() {
            "ip" => {
                ip = args.get(i + 1).unwrap().clone();
                i += 2;
            }
            "port" => {
                port = args.get(i + 1).unwrap().clone().parse().unwrap();
                i += 2;
            }
            "https" => {
                crt_file = args.get(i + 1).unwrap();
                key_file = args.get(i + 2).unwrap();
                i += 3;
            }
            _ => {
                if cfg!(debug_assertions) {
                    panic!("\nargv[{nth}]: {value} is invalid.\n", nth = i, value = args.get(i).unwrap());
                } else {
                    println!("\n{seg}\nargv[{nth}]: {value} is invalid.\n{seg}\n", seg = "-".repeat(15),
                             nth = i, value = args.get(i).unwrap());
                }
            }
        }
    }
    println!("listen on:{0}:{1}", ip, port);
    let _ = i;
    let server = HttpServer::new(
        || App::new()
            .service(
                all_methods_v1::gets_v1::get_score()
            )
            .service(
                all_methods_v1::posts_v1::get_score()
            )
            .default_service(
                web::route().to(method_not_found)
            )
    );
    if crt_file == "" {
        server.bind(format!("{0}:{1}", ip, port))?
            .run()
            .await
    } else {
        let config_builder = ServerConfig::builder().with_no_client_auth();
        let cert_file = &mut BufReader::new(File::open(crt_file)?);
        let key_file = &mut BufReader::new(File::open(key_file)?);
        let cert_chain = certs(cert_file).collect::<Result<Vec<_>, _>>()?;
        let mut keys = pkcs8_private_keys(key_file)
            .map(|key| key.map(PrivateKeyDer::Pkcs8))
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        let config = config_builder.with_single_cert(cert_chain, keys.remove(0)).unwrap();
        server.bind_rustls_0_23(format!("{0}:{1}", ip, port), config)?
            .run()
            .await
    }
}

