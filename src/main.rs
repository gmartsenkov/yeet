use handlebars::Handlebars;
use serde_json::Value;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template = fs::read_to_string("design/template.html.mustache").expect("Could not load template");

    // render without register
    let data = fs::read_to_string("data.json")?;
    let data_json : &Value = &serde_json::from_str(&data)?;

    fs::write("output.html", reg.render_template(template.as_str(), data_json)?)?;

    Ok(())
}