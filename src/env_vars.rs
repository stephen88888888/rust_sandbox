use std::env;

fn exec() {
    if let Ok(env_var) = env::var("MY_ENV_VAR") {
        println!("MY_ENV_VAR: {}", env_var);
    } else {
        println!("Environment variable MY_ENV_VAR is not set.");
    }
}
