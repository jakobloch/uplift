use application::run;
use instance::AshInstance;

fn main() {
    let app_name = "Ash GUI";
    let ash_instance = AshInstance::new(app_name).expect("Failed to create Ash instance");

    run();
}
