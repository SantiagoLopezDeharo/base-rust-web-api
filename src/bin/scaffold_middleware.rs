use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

fn main() -> io::Result<()> {
    let mut args = env::args().skip(1).collect::<Vec<_>>();
    if args.is_empty() {
        eprintln!("Usage: cargo run --bin scaffold_middleware -- <MiddlewareName>");
        std::process::exit(1);
    }

    let raw_name = args.remove(0);
    let module_name = to_snake_case(&raw_name);
    let fn_name = module_name.clone();

    let workspace_root = env::current_dir()?;
    let middlewares_dir = workspace_root.join("src/middlewares");
    fs::create_dir_all(&middlewares_dir)?;

    let middleware_file = middlewares_dir.join(format!("{}.rs", module_name));
    let template = format!(
        "use crate::primitives::http::request::Request;\nuse crate::primitives::http::response::Response;\nuse crate::routing::{{next_handler, Handler, RouteParams}};\n\npub async fn {fn_name}(request: &mut Request, params: &RouteParams, handlers: &mut Vec<Handler>) -> Response {{\n    // Pre-processing logic here\n    let response = next_handler(request, params, handlers).await;\n    // Post-processing logic here\n    response\n}}\n",
        fn_name = fn_name
    );

    write_file_if_missing(middleware_file, &template)?;
    update_middlewares_mod(&middlewares_dir.join("mod.rs"), &module_name)?;

    println!(
        "Middleware '{}' scaffolded at src/middlewares/{}.rs",
        raw_name, module_name
    );
    Ok(())
}

fn to_snake_case(input: &str) -> String {
    let mut out = String::new();
    for (i, ch) in input.chars().enumerate() {
        if ch.is_uppercase() {
            if i > 0 {
                out.push('_');
            }
            out.extend(ch.to_lowercase());
        } else if ch == '-' || ch == ' ' {
            out.push('_');
        } else {
            out.push(ch.to_ascii_lowercase());
        }
    }
    out
}

fn write_file_if_missing(path: PathBuf, content: &str) -> io::Result<()> {
    if path.exists() {
        return Ok(());
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut file = fs::File::create(path)?;
    file.write_all(content.as_bytes())
}

fn update_middlewares_mod(mod_path: &Path, module_name: &str) -> io::Result<()> {
    let line = format!("pub mod {};", module_name);
    let mut content = fs::read_to_string(mod_path).unwrap_or_default();
    if !content.lines().any(|l| l.trim() == line) {
        if !content.ends_with('\n') && !content.is_empty() {
            content.push('\n');
        }
        content.push_str(&line);
        content.push('\n');
        fs::write(mod_path, content)?;
    }
    Ok(())
}
