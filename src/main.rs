use std::fs;
use std::env;
use tera::{Context, Tera};

fn main() {
    let args: Vec<String> = env::args().collect();
    let template_path = &args[1];
    let values_path = &args[2];
    let output_path = &args[3];

    let template = fs::read_to_string(template_path).expect("template input not found");
    let values: serde_yaml::Value = serde_yaml::from_str(
        &fs::read_to_string(values_path).expect("values input not found"),
    ).expect("unable to parse values");

    let mut context = Context::new();
    for (k, v) in values.as_mapping().unwrap().iter() {
        let k = k.as_str().unwrap();
        context.insert(k, v);
    }

    let output = Tera::one_off(template.as_str(), &context, false).expect("unable to render template");
    fs::write(output_path, output.as_str()).expect("unable to write output");
}

