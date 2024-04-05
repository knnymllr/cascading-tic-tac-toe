extern crate embed_resource;

fn main() {
    let target = std::env::var("TARGET").unwrap();
    if target.contains("windows") {
        embed_resource::compile("assets/texture/icons/icon.rc", embed_resource::NONE);
    }
}