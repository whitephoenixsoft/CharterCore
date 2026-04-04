use charter_legitimacy::api::engine::Engine;

fn main() {
    let manifest = Engine::specification_manifest();

    println!("Engine Version: {}", manifest.engine_version);
    println!("Spec Set Hash: {}", manifest.spec_set_hash);
}
