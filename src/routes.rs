use rocket_contrib::Template;
use rocket::State;

use diesel::prelude::*;
use failure::Error;
use lettre::{EmailTransport, SmtpTransport};
use lettre_email::EmailBuilder;

use db_conn::DbConn;
use models::*;
use action::*;
use config::Config;
use util::url_with_query;

#[get("/")]
fn index(config: State<Config>) -> Template {
    Template::render("index", &json!({"config": &*config}))
}

#[derive(Serialize,FromForm)]
struct ListForm {
    email: String,
}

#[get("/list?<form>")]
fn list(form: ListForm, db: DbConn) -> Result<Template, Error> {
    use schema::monitors::dsl::*;

    let nodes = monitors
        .filter(email.eq(form.email.as_str()))
        .load::<Monitor>(&*db)?;
    Ok(Template::render("list", &json!({"form": form, "nodes": nodes})))
}

#[get("/prepare_action?<action>")]
fn prepare_action(action: Action, config: State<Config>) -> Result<Template, Error> {
    // Build email
    //let url = url_with_query("list".to_owned(), &[("email", action.email.as_str())]);
    let email = EmailBuilder::new()
        // Addresses can be specified by the tuple (email, alias)
        .to(action.email.as_str())
        // ... or by an address only
        .from(config.email_from.as_str())
        .subject("Hi, Hello world")
        .text("Hello world.")
        .build()?;
    // Send email
    let mut mailer = SmtpTransport::builder_unencrypted_localhost()?.build();
    mailer.send(&email)?;

    Ok(Template::render("prepare_action", &json!({"action": action})))
}

pub fn routes() -> Vec<::rocket::Route> {
    routes![index, list, prepare_action]
}
