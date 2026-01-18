use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

fn main() -> io::Result<()> {
    let mut args = env::args().skip(1).collect::<Vec<_>>();
    if args.is_empty() {
        eprintln!("Usage: cargo run --bin scaffold_entity -- <EntityName>");
        std::process::exit(1);
    }

    let raw_name = args.remove(0);
    let module_name = to_snake_case(&raw_name);
    let entity_name = to_camel_case(&raw_name);

    let workspace_root = env::current_dir()?;
    let domain_dir = workspace_root.join("src/domain");
    let entity_dir = domain_dir.join(&module_name);

    fs::create_dir_all(&entity_dir)?;

    write_file_if_missing(
        entity_dir.join("mod.rs"),
        &format!("pub mod controller;\npub mod dto;\npub mod repo;\npub mod service;\n"),
    )?;

    write_file_if_missing(
        entity_dir.join("dto.rs"),
        &format!(
            "pub struct {}Dto {{\n    pub message: String,\n}}\n\nimpl {}Dto {{\n    pub fn new(message: String) -> Self {{\n        Self {{ message }}\n    }}\n}}\n",
            entity_name, entity_name
        ),
    )?;

    write_file_if_missing(
        entity_dir.join("repo.rs"),
        &format!(
            "pub struct {}Repo;\n\nimpl {}Repo {{\n    pub fn new() -> Self {{\n        Self\n    }}\n}}\n",
            entity_name, entity_name
        ),
    )?;

    write_file_if_missing(
        entity_dir.join("service.rs"),
        &format!(
            "use super::repo::{}Repo;\n\npub struct {}Service {{\n    _repo: {}Repo,\n}}\n\nimpl {}Service {{\n    pub fn new(repo: {}Repo) -> Self {{\n        Self {{ _repo: repo }}\n    }}\n\n    pub fn respond(&self) -> String {{\n        \"{}\".to_string()\n    }}\n}}\n",
            entity_name, entity_name, entity_name, entity_name, entity_name, entity_name
        ),
    )?;

    let controller_template = r#"use std::collections::HashMap;

use crate::primitives::http::request::Request;
use crate::primitives::http::response::Response;
use crate::route;
use crate::routing::{Route, RouteParams};

use super::repo::{{ENTITY}}Repo;
use super::service::{{ENTITY}}Service;

pub struct {{ENTITY}}Controller;

impl {{ENTITY}}Controller {
    pub fn routes() -> Vec<Route> {
        vec![
            Route::new("GET", &["{{MODULE}}"], vec![route!({{ENTITY}}Controller::get_all)]),
            Route::new("POST", &["{{MODULE}}"], vec![route!({{ENTITY}}Controller::create)]),
            Route::new("GET", &["{{MODULE}}", ":id"], vec![route!({{ENTITY}}Controller::get_one)]),
            Route::new("PUT", &["{{MODULE}}", ":id"], vec![route!({{ENTITY}}Controller::update)]),
            Route::new("DELETE", &["{{MODULE}}", ":id"], vec![route!({{ENTITY}}Controller::delete)]),
        ]
    }

    pub async fn get_all(_request: &mut Request, _params: &RouteParams) -> Response {
        let service = {{ENTITY}}Service::new({{ENTITY}}Repo::new());
        let body = service.respond();
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "text/plain".to_string());
        Response {
            status_code: 200,
            headers,
            body,
        }
    }

    pub async fn get_one(_request: &mut Request, params: &RouteParams) -> Response {
        let _id = params.get("id").unwrap_or("");
        let service = {{ENTITY}}Service::new({{ENTITY}}Repo::new());
        let body = service.respond();
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "text/plain".to_string());
        Response {
            status_code: 200,
            headers,
            body,
        }
    }

    pub async fn create(_request: &mut Request, _params: &RouteParams) -> Response {
        let service = {{ENTITY}}Service::new({{ENTITY}}Repo::new());
        let body = service.respond();
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "text/plain".to_string());
        Response {
            status_code: 201,
            headers,
            body,
        }
    }

    pub async fn update(_request: &mut Request, params: &RouteParams) -> Response {
        let _id = params.get("id").unwrap_or("");
        let service = {{ENTITY}}Service::new({{ENTITY}}Repo::new());
        let body = service.respond();
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "text/plain".to_string());
        Response {
            status_code: 200,
            headers,
            body,
        }
    }

    pub async fn delete(_request: &mut Request, params: &RouteParams) -> Response {
        let _id = params.get("id").unwrap_or("");
        let service = {{ENTITY}}Service::new({{ENTITY}}Repo::new());
        let body = service.respond();
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "text/plain".to_string());
        Response {
            status_code: 200,
            headers,
            body,
        }
    }
}
"#;

    let controller_content = controller_template
        .replace("{{ENTITY}}", &entity_name)
        .replace("{{MODULE}}", &module_name);

    write_file_if_missing(entity_dir.join("controller.rs"), &controller_content)?;

    update_domain_mod(&domain_dir.join("mod.rs"), &module_name)?;
    update_routing_init(
        &workspace_root.join("src/routing/init.rs"),
        &module_name,
        &entity_name,
    )?;

    println!(
        "Entity '{}' scaffolded at src/domain/{}",
        entity_name, module_name
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

fn to_camel_case(input: &str) -> String {
    let mut out = String::new();
    let mut capitalize = true;
    for ch in input.chars() {
        if ch == '_' || ch == '-' || ch == ' ' {
            capitalize = true;
            continue;
        }
        if capitalize {
            out.extend(ch.to_uppercase());
            capitalize = false;
        } else {
            out.push(ch);
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

fn update_domain_mod(mod_path: &Path, module_name: &str) -> io::Result<()> {
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

fn update_routing_init(init_path: &Path, module_name: &str, entity_name: &str) -> io::Result<()> {
    let use_line = format!(
        "use crate::domain::{}::controller::{}Controller;",
        module_name, entity_name
    );
    let extend_line = format!("    routes.extend({}Controller::routes());", entity_name);

    let mut content = fs::read_to_string(init_path).unwrap_or_default();
    if !content.contains(&use_line) {
        let mut lines: Vec<String> = content.lines().map(|l| l.to_string()).collect();
        let insert_pos = lines
            .iter()
            .rposition(|l| l.starts_with("use "))
            .map(|i| i + 1)
            .unwrap_or(0);
        lines.insert(insert_pos, use_line);
        content = lines.join("\n");
        content.push('\n');
    }

    if !content.contains(&extend_line) {
        if let Some(idx) = content.find("routes\n}") {
            let (head, tail) = content.split_at(idx);
            let mut new_content = String::new();
            new_content.push_str(head);
            new_content.push_str(&format!("{}\n", extend_line));
            new_content.push_str(tail);
            content = new_content;
        } else {
            content.push_str(&format!("{}\n", extend_line));
        }
    }

    fs::write(init_path, content)?;
    Ok(())
}
