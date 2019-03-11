use std::path::{Path, PathBuf};

pub fn pyckitup_init(matches: &clap::ArgMatches) {
    let project_name = matches.value_of("project").unwrap_or("new_pyckitup_project");
    if Path::new(&format!("./{}", project_name)).exists() {
        println!("Path ./{} already exists. Doing nothing.", project_name);
        std::process::exit(1);
    }

    println!("Initializing pyckitup project in directory `./{}`", project_name);
    std::fs::create_dir(&format!("./{}/", project_name));
    std::fs::create_dir(&format!("./{}/static/", project_name));
    std::fs::write(&format!("./{}/static/click.wav", project_name), include_bytes!("../../include/click.wav").to_vec());
    std::fs::write(&format!("./{}/run.py", project_name), include_bytes!("../../examples/clock.py").to_vec());
    std::fs::write(&format!("./{}/common.py", project_name), include_bytes!("../../examples/common.py").to_vec());
    std::fs::write(&format!("./{}/.gitignore", project_name), include_bytes!("../../include/gitignore").to_vec());
    println!("Initialized. To run: `pyckitup`");
}