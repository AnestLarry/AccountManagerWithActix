use std::borrow::Borrow;
use std::env::args as std_args;

use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use serde::Serialize;

mod data_generator;
mod sql_operator;

#[derive(Debug, Serialize)]
struct MessageResponse<'a> {
    message: &'a str,
}
static VERSION:&'static str = "5.0.0/20210628";
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
                let hm = [String::from("account"), String::from("passwordLv1"),
                    String::from("passwordLv2"), String::from("passwordLv3"), String::from("passwordLvMax")];

                if !hm.contains(&item_name) {
                    HttpResponse::Ok().json(Response { message: "Mismatch your item name.", value: String::from("") })
                } else {
                    let res = match item_name.as_str() {
                        "account" => random_generator.get_account(),
                        "passwordLv1" => random_generator.get_password_1(),
                        "passwordLv2" => random_generator.get_password_2(),
                        "passwordLv3" => random_generator.get_password_3(),
                        "passwordLvMax" => random_generator.get_password_max(),
                        _ => String::from("")
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

            use actix_web::{guard, HttpResponse, Responder, web};
            use serde::Serialize;

            use crate::MessageResponse;
            use crate::sql_operator::{Data as SQLOperatorData, SQLOperator};

            #[derive(Debug, Serialize)]
            struct Response<'a> {
                message: &'a str,
                changed: usize,
            }

            pub async fn save_item(paras: web::Form<SQLOperatorData>) -> impl Responder {
                let sql_oper = SQLOperator::new();

                match sql_oper.add_item(
                    &SQLOperator::Data_of(
                        base64::encode(paras.address.clone().as_bytes()),
                        base64::encode(paras.account.clone().as_bytes()),
                        base64::encode(paras.password.clone().as_bytes()),
                        String::from(""),
                        paras.text.clone())
                ) {
                    Ok(d) => { HttpResponse::Ok().json(Response { message: "succ", changed: d }) }
                    Err(_) => { HttpResponse::Ok().json(Response { message: "err", changed: 0 }) }
                }
            }

            pub async fn search_item(paras: web::Form<SQLOperatorData>) -> impl Responder {
                let sql_oper = SQLOperator::new();
                #[derive(Debug, Serialize)]
                struct Response<'a> {
                    message: &'a str,
                    result: Vec<SQLOperatorData>,
                }
                let mut key = "".to_string();
                let mut keyword = "".to_string();
                let mut status = false;
                if paras.address != String::from("") {
                    keyword = base64::encode(&paras.address);
                    key = String::from("Address");
                    status = true;
                } else if paras.account != String::from("") {
                    keyword = base64::encode(&paras.account);
                    key = String::from("Account");
                    status = true;
                } else if paras.password != String::from("") {
                    keyword = base64::encode(&paras.password);
                    key = String::from("Password");
                    status = true;
                } else if paras.text != String::from("") {
                    keyword = paras.text.clone().replace("%", "");
                    key = String::from("Text");
                    if keyword.len() != 0 {
                        status = true;
                    }
                }
                if status {
                    match sql_oper.search_item(key, keyword) {
                        Ok(mut r) => {
                            for i in 0..r.len() {
                                let x = r[i].borrow();
                                r[i] = SQLOperatorData {
                                    address: String::from_utf8(base64::decode(&x.address).unwrap()).unwrap(),
                                    account: String::from_utf8(base64::decode(&x.account).unwrap()).unwrap(),
                                    password: String::from_utf8(base64::decode(&x.password).unwrap()).unwrap(),
                                    date: x.date.clone(),
                                    text: x.text.clone(),
                                };
                            }
                            HttpResponse::Ok().json(Response { message: "succ", result: r })
                        }
                        Err(_) => { HttpResponse::InternalServerError().json(Response { message: "server have some problems.", result: Default::default() }) }
                    }
                } else {
                    HttpResponse::Forbidden().json(Response { message: "No legal argument(address, account, password, text) in request.", result: Default::default() })
                }
            }

            pub async fn delete_item(paras: web::Form<SQLOperatorData>) -> impl Responder {
                let sql_oper = SQLOperator::new();
                let date = paras.date.clone();
                if date == String::from("") {
                    HttpResponse::BadRequest().json(Response { message: "miss date.", changed: 0 })
                } else {
                    match sql_oper.remove_item(date) {
                        Ok(d) => { HttpResponse::Ok().json(Response { message: "succ", changed: d.0 }) }
                        Err(_) => { HttpResponse::Ok().json(Response { message: "err", changed: 0 }) }
                    }
                }
            }

            pub async fn update_item(paras: web::Form<SQLOperatorData>) -> impl Responder {
                let sql_oper = SQLOperator::new();
                if paras.date == String::from("") {
                    HttpResponse::BadRequest().json(MessageResponse { message: "miss date." })
                } else {
                    match sql_oper.update_item(paras.text.clone(), paras.date.clone()) {
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = std_args().collect();
    let mut ip = String::from("127.0.0.1");
    let mut port = 8000u16;
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
    drop(i);
    HttpServer::new(
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
    )
        .bind(format!("{0}:{1}", ip, port))?
        .run()
        .await
}

