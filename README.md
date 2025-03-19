# I just want to have query parameters in the OpenAPI spec

**Status**: resolved. RTFM.

I use [`openapi`](https://www.npmjs.com/package/openapi) NPM package for the purpose of generation of JavaScript client code and TypeScript type definitions from the OpenAPI spec,
which in turn is generated from the `ApiDoc` structure with `utoipa::OpenApi` derive.

```rust
#[utoipauto]
#[derive(OpenApi)]
#[openapi(info(description = "My Api description"))]
pub struct ApiDoc;

// ...

pub fn gen_api_doc() -> String {
    ApiDoc::openapi()
        .to_pretty_json()
        .expect("Failed generated OpenAPI doc")
}

pub fn generate_openapi_spec() {
    const SPEC_PATH: &str = "openapi-spec.json";
    let spec = gen_api_doc();
    let path = std::path::Path::new(SPEC_PATH);
    std::fs::write(path, spec).expect("Failed to write OpenAPI spec");
}
```

Without [`utoipauto`](https://crates.io/crates/utoipauto), I would have to manually add all [DTO](https://learn.microsoft.com/en-us/aspnet/web-api/overview/data/using-web-api-with-entity-framework/part-5)s and endpoints.

When I run the code in [`src/main.rs`](https://github.com/JohnScience/maybe-axum-utoipa-bug/blob/main/src/main.rs), I get the following output of `openapi-spec.json`

```json
{
  "openapi": "3.1.0",
  "info": {
    "title": "maybe-axum-utoipa-bug",
    "description": "My Api description",
    "license": {
      "name": ""
    },
    "version": "0.1.0"
  },
  "paths": {
    "/hello-with-name": {
      "get": {
        "tags": [
          "crate"
        ],
        "operationId": "hello_with_name",
        "responses": {}
      }
    },
    "/hello-world": {
      "get": {
        "tags": [
          "crate"
        ],
        "operationId": "hello_world",
        "responses": {}
      }
    }
  },
  "components": {}
}
```

which is not what I want.

Instead of

```json
"/hello-with-name": {
      "get": {
        "tags": [
          "crate"
        ],
        "operationId": "hello_with_name",
        "responses": {}
      }
}
```

I would like to see

```json
"/hello-with-name": {
      "get": {
        "tags": [
          "crate"
        ],
        "operationId": "hello_with_name",
        "parameters": [
          {
            "name": "name",
            "in": "query",
            "required": false,
            "schema": {
              "type": "string"
            }
          }
        ],
        "responses": {}
      }
}
```

*Note: notice the new `"parameters"` property*.

At this point, I start suspecting that the undesired behavior is caused by a bug in one of the libraries

* [`utoipa`](https://crates.io/crates/utoipa) (with the `axum_extra` feature),
* [`utoipauto`](https://crates.io/crates/utoipauto).

## Workaround solution

Just add

```rust
#[utoipa::path(
  params(
        MyQueryParams
  ),
)]
```

## Source of problem

There is presumably unnecessary redundancy because `MyQueryParams` appears both in the `Query<...>` and in the `params(...)`.

## Fundamental solution

Maybe, [`utoipa`](https://crates.io/crates/utoipa) (with the `axum_extra` feature) should infer the query parameters.
