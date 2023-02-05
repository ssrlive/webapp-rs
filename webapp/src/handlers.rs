use crate::{
    models::{TeacherRegisterForm, TeacherResponse},
    state::AppState,
};
use actix_web::{web, Error, HttpResponse, Result};
use serde_json::json;

pub async fn get_all_teachers(
    state: web::Data<AppState>,
    tmpl: web::Data<tera::Tera>,
) -> Result<HttpResponse, Error> {
    let url = format!("{}/teachers/", state.backend_url);
    let awc_client = awc::Client::default();
    let res = awc_client
        .get(url)
        .send()
        .await
        .unwrap()
        .json::<Vec<TeacherResponse>>()
        .await
        .unwrap();
    let mut ctx = tera::Context::new();
    ctx.insert("teachers", &res);
    ctx.insert("error", &false);

    let s = tmpl.render("teachers.html", &ctx).unwrap();

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub async fn show_register_form(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("error", &false);
    ctx.insert("current_name", &"");
    ctx.insert("current_imageurl", &"");
    ctx.insert("current_profile", &"");
    let s = tmpl.render("register.html", &ctx).unwrap();

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub async fn handle_register(
    state: web::Data<AppState>,
    tmpl: web::Data<tera::Tera>,
    form: web::Form<TeacherRegisterForm>,
) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    let s;
    if form.name == "Dave" {
        ctx.insert("error", "Dave is already registered");
        ctx.insert("current_name", &form.name);
        ctx.insert("current_imageurl", &form.imageurl);
        ctx.insert("current_profile", &form.profile);
        s = tmpl.render("register.html", &ctx).unwrap();
    } else {
        let new_teacher = json!({
            "name": form.name,
            "picture_url": form.imageurl,
            "profile": form.profile,
        });
        let awc_client = awc::Client::default();
        let url = format!("{}/teachers/", state.backend_url);
        let res = awc_client
            .post(url)
            .send_json(&new_teacher)
            .await
            .unwrap()
            .json::<TeacherResponse>()
            .await
            .unwrap();
        s = format!(
            "Congratulations! You have registered as a teacher. {}",
            res.id
        );
    }
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}
