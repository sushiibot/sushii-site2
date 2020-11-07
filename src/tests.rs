
use rocket::http::Method::*;
use rocket::http::Status;
use rocket::local::blocking::Client;
use rocket_contrib::templates::Template;

use crate::rocket;
use crate::assets::ASSETS;
use crate::model::context::TemplateContext;

macro_rules! dispatch {
    ($method:expr, $path:expr, |$client:ident, $response:ident| $body:expr) => {{
        let $client = Client::tracked(rocket()).unwrap();
        let $response = $client.req($method, $path).dispatch();
        $body
    }};
}

#[test]
fn test_root() {
    dispatch!(Get, "/", |client, response| {
        let context = TemplateContext {
            title: "Home",
            name: None,
            commands: None,
            assets: &ASSETS,
            parent: "layout",
        };

        let expected = Template::show(client.rocket(), "index", &context).unwrap();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string(), Some(expected));
    });

    // Check that other request methods are not accepted (and instead caught).
    for method in &[Post, Put, Delete, Options, Trace, Connect, Patch] {
        dispatch!(*method, "/", |client, response| {
            let mut map = std::collections::HashMap::new();
            map.insert("path", "/");
            let expected = Template::show(client.rocket(), "error/404", &map).unwrap();

            assert_eq!(response.status(), Status::NotFound);
            assert_eq!(response.into_string(), Some(expected));
        });
    }
}

#[test]
fn test_404() {
    // Check that the error catcher works.
    dispatch!(Get, "/helloworld/", |client, response| {
        let mut map = std::collections::HashMap::new();
        map.insert("path", "/helloworld/");

        let expected = Template::show(client.rocket(), "error/404", &map).unwrap();
        assert_eq!(response.status(), Status::NotFound);
        assert_eq!(response.into_string(), Some(expected));
    });
}
