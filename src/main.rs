use std::io;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};

use example_api::x_outside_feature::_api::init::new_app_state;

use example_api::auth::_api::x_actix_web::route::scope_auth;

#[actix_web::main]
async fn main() -> io::Result<()> {
    let (state, setting) = new_app_state();

    let data = web::Data::new(state);
    let origin = setting.origin;

    HttpServer::new(move || {
        let cors = Cors::default()
            .supports_credentials()
            .allowed_origin(&origin)
            .allowed_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE"])
            .allowed_headers(vec!["X-GETTO-EXAMPLE-NONCE"])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(data.clone())
            .service(route::root)
            .service(demo::aws_sms)
            .service(scope_auth())
    })
    .bind(format!("0.0.0.0:{}", setting.port))?
    .run()
    .await
}

mod route {
    use actix_web::{get, Responder};

    #[get("/")]
    async fn root() -> impl Responder {
        "GETTO-EXAMPLE-API: OK"
    }
}

mod demo {
    use actix_web::{get, Responder};
    use rusoto_core::Region;
    use rusoto_ses::{Body, Content, Destination, Message, SendEmailRequest, Ses, SesClient};

    #[get("/aws-sms")]
    async fn aws_sms() -> impl Responder {
        println!("{}", "init client");
        let client = SesClient::new(Region::ApNortheast1);
        println!("{}", "init request");
        println!("{}", CONTENT);
        let request = SendEmailRequest {
            destination: Destination {
                bcc_addresses: None,
                cc_addresses: None,
                to_addresses: Some(vec!["shun@getto.systems".into()]),
            },
            message: Message {
                subject: Content {
                    charset: Some("UTF-8".into()),
                    data: "rusoto で aws ses を利用してみるテスト".into(),
                },
                body: Body {
                    html: None,
                    text: Some(Content {
                        charset: Some("UTF-8".into()),
                        data: CONTENT.into(),
                    }),
                },
            },
            source: "labo@message.getto.systems".into(),
            ..Default::default()
        };
        println!("{}", "send email");
        match client.send_email(request).await {
            Ok(response) => {
                format!("send email success; message-id: {}", response.message_id)
            }
            Err(err) => {
                format!("send email error!; {}", err)
            }
        }
    }

    const CONTENT: &'static str = r#########"よろしくお願いいたします

####
send by rusoto / aws ses
labo@message.getto.systems
"#########;
}
