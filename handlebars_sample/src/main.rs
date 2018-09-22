extern crate handlebars;

use handlebars::Handlebars;
use std::error::Error;
use std::collections::BTreeMap;

fn main() -> Result<(), Box<Error>> {
    let mut handlebars = Handlebars::new();
    
    handlebars.register_template_file("layout", "./templates/layout.hbs")?;
    handlebars.register_template_file("index", "./templates/index.hbs")?;

    let mut data = BTreeMap::new();
    data.insert("title".to_string(), "Title: Index".to_string());
    println!("{}", handlebars.render("index", &data)?);

    Ok(())
}
