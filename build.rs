extern crate embed_resource;
use copy_to_output::copy_to_output;  
use std::env;

fn main() {
    let _ = embed_resource::compile("resources.rc", embed_resource::NONE);

    // COPY RESOURCES: 
    println!("cargo:rerun-if-changed=res/*");  
    copy_to_output("src/assets", &env::var("PROFILE").unwrap()).expect("Could not copy");  
}
