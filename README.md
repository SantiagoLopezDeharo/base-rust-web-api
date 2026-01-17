# Base Rust Web API

A lightweight, single-thread async Rust web API with a NestJS-like folder structure and a simple routing system. Controllers declare their own routes, and the router registers them at startup.

## Features

- Single-thread async runtime (Tokio current-thread)
- Strongly inspired on NestJS proven development patterns 
- Request/Response primitives with automatic status lines
- Controller-first route declarations (NestJS-like)
- Domain-driven structure: controller, service, repo, DTO
- CLI scaffold to generate new entities

## Project Structure

```
src/
  bin/
    scaffold_entity.rs
  domain/
    dog/
      controller.rs
      service.rs
      repo.rs
      dto.rs
      mod.rs
    mod.rs
  primitives/
    http/
      request.rs
      response.rs
      mod.rs
    mod.rs
  routing/
    init.rs
    mod.rs
  main.rs
```

## Running the Server

```bash
cargo run
```

The server listens on `127.0.0.1:8080`.

## Routing Flow

1. The router is initialized at startup via `init(init_routes())`.
2. Each controller exposes a `routes()` method returning `Route` definitions.
3. The router matches method/path and invokes the controller handler.
4. The controller returns a `Response`, which is written back to the client.

## Creating a New Entity

Use the scaffold CLI to generate a new domain entity:

```bash
cargo run --bin scaffold_entity -- Dog
```

This will:
- Create `src/domain/dog/` with controller/service/repo/dto files.
- Add the entity to `src/domain/mod.rs`.
- Register the controller routes in `src/routing/init.rs`.

## Adding Controller Routes

Routes are declared inside each controller’s `routes()` method using the `route!` macro:

```rust
Route::new("GET", &["dog"], route!(DogController::get_all))
```

Handlers receive:
- `Request`
- `RouteParams` (path params like `:id` are available via `params.get("id")`)

## Example Request

```bash
curl http://127.0.0.1:8080/dog
```

The default Dog controller responds with:

```
Woof
```

## Notes

- Responses automatically include `Content-Length` and `Connection: close` if not provided.
- The router is a singleton registry initialized before the server starts listening.
