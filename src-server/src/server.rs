use askama::Template;
use itertools::Itertools;
use salvo::prelude::*;

#[handler]
async fn root(res: &mut Response) {
    res.render(Text::Plain("Hello world from root (/)"))
}

#[handler]
async fn hello(res: &mut Response) {
    res.render(Text::Html("<div>General Kenobi</div>"));
}

#[handler]
async fn get_test(res: &mut Response) {
    res.render(Text::Html("<div>Get /test, new version 2</div>"));
}

#[handler]
async fn post_test(req: &mut Request, res: &mut Response) {
    println!("{:?}", (req.uri()));
    match req.query::<String>("text") {
        Some(text) => {
            let list = text.lines().map(|line| format!("<li>{line}</li>")).join("");
            res.render(Text::Html(format!("<ul>{}</ul>", list)));
        }
        None => {
            let debug = format!("{:?}", req.queries());
            res.render(Text::Html(format!(
                "Failed to provide `text` in query<br/>{}",
                debug
            )));
        }
    }
}

#[derive(Template)]
#[template(path = "startup.html.j2")]
struct Startup<'render> {
    servers: Vec<ServerInfo<'render>>,
}

struct ServerInfo<'render> {
    name: &'render str,
}

#[handler]
fn get_startup(res: &mut Response) {
    // TODO: Authentication
    let fake_data = Startup {
        servers: vec![ServerInfo { name: "ServerU" }, ServerInfo { name: "ATM" }],
    };

    res.render(Text::Html(fake_data.render().unwrap()))
}

pub async fn run_server() {
    let router = Router::new()
        .get(root)
        .push(Router::with_path("hello").get(hello))
        .push(Router::with_path("test").get(get_test).post(post_test))
        .push(Router::with_path("startup").get(get_startup));
    let acceptor = TcpListener::new("0.0.0.0:4040").bind().await;

    // println!("{:?}", router);
    Server::new(acceptor).serve(router).await;
}
