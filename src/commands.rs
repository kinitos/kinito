use config::Config;
use termcmd::TermCmd;

/// Builds the android shell, ergo kicks off the gradle build and apk compile.
pub fn build(config: &Config) {
    let project_path = config.project_path_str();
    let mut cargo_build_targets = String::new();

    for arch in &config.build_targets {
        let s = if cargo_build_targets.len() > 0 {
            format!(", {}", arch)
        } else {
            format!("{}", arch)
        };

        cargo_build_targets.push_str(&*s);
    }

    println!("  rumo: building initiated");
    TermCmd::new("build", "target/android-shell/bin/build")
        .current_dir(project_path)
        .inherit_stdouterr()
        .env("CARGO_BUILD_TARGETS", cargo_build_targets)
        .env("RUST_APP_ROOT", project_path)
        .env("RUST_APP_NAME", config.project_name_underscore.clone())
        .execute();
}

/// Copies project specific assets into the android-shell
pub fn assets(config: &Config) {
    let project_path = config.project_path_str();
    println!("  rumo: installing assets");
    TermCmd::new("install", "target/android-shell/bin/assets")
        .current_dir(project_path)
        .inherit_stdouterr()
        .env("RUST_APP_RESOURCES", config.resources_dir.clone())
        .env("RUST_APP_ROOT", project_path)
        .env("RUST_APP_NAME", config.project_name_underscore.clone())
        .execute();
}


pub fn install(config: &Config) {
    let project_path = config.project_path_str();
    println!("  rumo: install initiated");
    TermCmd::new("install", "target/android-shell/bin/install")
        .current_dir(project_path)
        .inherit_stdouterr()
        .env("RUST_APP_ROOT", project_path)
        .env("RUST_APP_NAME", config.project_name_underscore.clone())
        .execute();
}

pub fn clean(config: &Config) {
    let project_path = config.project_path_str();
    println!("  rumo: clean initiated");
    TermCmd::new("clean", "target/android-shell/bin/clean")
        .current_dir(project_path)
        .inherit_stdouterr()
        .env("RUST_APP_ROOT", project_path)
        .env("RUST_APP_NAME", config.project_name_underscore.clone())
        .execute();
}


fn r(src: &str, a: &str, b: &str) -> String {
    str::replace(src, a, b)
}
