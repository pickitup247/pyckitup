use std::path::{Path, PathBuf};
use walkdir::{WalkDir, DirEntry};

pub fn pyckitup_build() {
    println!("Deploying to `./build`");
    if !Path::new("./run.py").exists() {
        println!("File `./run.py` doesn't exist. Doing nothing.");
        std::process::exit(1);
    }
    let mut options = fs_extra::dir::CopyOptions::new();
    options.copy_inside = true;
    options.overwrite = true;
    fs_extra::dir::copy("./static", "./build", &options);
    std::fs::write("./build/pyckitup.js", include_bytes!("../../target/deploy/pyckitup.js").to_vec());
    std::fs::write("./build/pyckitup.wasm", include_bytes!("../../target/deploy/pyckitup.wasm").to_vec());
    std::fs::write("./build/server.py", include_bytes!("../../include/server.py").to_vec());

    let template = include_str!("../../include/template.html");
    let rendered = render(template);
    std::fs::write("./build/index.html", rendered);
    println!("Deployed!");
}


fn is_py(entry: &DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| s.ends_with(".py"))
        .unwrap_or(false)
}


fn read_file(path: &PathBuf) -> String {
    use std::io::Read;
    let mut f = std::fs::File::open(&path).unwrap();
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();
    buffer
}


fn render(tmpl: &str) -> String {
    let mut files = vec![];
    for entry in WalkDir::new(".").into_iter().filter_map(|e| e.ok()) {
        if is_py(&entry)
        && !entry.path().starts_with("./build")
        && !entry.path().starts_with("./static")
        {
            files.push(entry.path().to_owned());
            // println!("Python file: {:?}", entry);
        }
    }

    let mut code = String::new();

    code.push_str("console.log('Begin loading Python files...');\n");
    code.push_str("window.localStorage.clear();\n");
    for (i, (content, path)) in files.into_iter().map(|i|(read_file(&i), i)).enumerate() {
        let var_name = format!("file_{}", i);
        code.push_str(&format!("let {} = `{}`;\n", var_name, content));
        let path_stripped = path.as_path().strip_prefix("./").unwrap().to_str().unwrap();
        code.push_str(&format!("window.localStorage.setItem(\"{}\", btoa({}));\n", path_stripped, var_name));
    }
    code.push_str("console.log('Finished loading Python.');\n");
    tmpl.to_owned().replace("INSERTCODEHERE", &code)
}