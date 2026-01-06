use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=problems/");

    let problems_dir = Path::new("problems");
    let src_dir = Path::new("src");
    fs::create_dir_all(src_dir).ok();

    let mut modules = Vec::new();

    if let Ok(entries) = fs::read_dir(problems_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "rs") {
                if let Some(stem) = path.file_stem() {
                    let filename = stem.to_string_lossy();
                    let mod_name = format!("p{}", filename.replace(['.', '-'], "_"));
                    modules.push((mod_name, filename.to_string()));
                }
            }
        }
    }

    modules.sort_by(|a, b| {
        let num_a: u32 = a.1.split('.').next().unwrap_or("0").parse().unwrap_or(0);
        let num_b: u32 = b.1.split('.').next().unwrap_or("0").parse().unwrap_or(0);
        num_a.cmp(&num_b)
    });

    let mut content = String::from("#![allow(dead_code)]\npub struct Solution;\n\n");

    for (mod_name, filename) in &modules {
        content.push_str(&format!(
            "mod {} {{\n    use super::Solution;\n    include!(\"../problems/{}.rs\");\n}}\n\n",
            mod_name, filename
        ));
    }

    fs::write(src_dir.join("lib.rs"), content).expect("Failed to write lib.rs");
}
