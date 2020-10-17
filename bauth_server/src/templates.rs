use tera::{Error, Tera};

pub fn create_templates() -> Result<Tera, Error> {
    let mut tera = Tera::default();
    tera.add_raw_templates(vec![
        ("login.html", include_str!("./templates/login.html")),
        ("signup.html", include_str!("./templates/signup.html")),
        ("admin.html", include_str!("./templates/admin.html")),
    ])?;
    Ok(tera)
}
