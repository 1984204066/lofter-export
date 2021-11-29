use std::{fs::OpenOptions, io::Write};

use salvo::prelude::*;
use urlencoding::decode;

#[fn_handler]
async fn hello_world(res: &mut Response) {
    res.render_plain_text("Hello World!");
}

#[tokio::main]
async fn main() {
    let router = Router::new()
	.get(hello_world)
	.push(
	    Router::with_path("<id>")
		.post(create_article)
	);
    let listener = TcpListener::bind("127.0.0.1:7878");
    Server::new(listener).serve(router).await;
}

#[fn_handler]
async fn create_article(req: &mut Request, res: &mut Response) {
    // println!("{:?}", req);
    let msg =req.read_text().await.unwrap();
    // let msg =  msg.to_string();
    let msg = decode(msg).unwrap();
    println!("{:#?}", msg);
    let mut fid= "".to_string();
    if let Some(id) = req.get_param::<String>("id") {
	println!("{}", id);
	fid = id;
    } else {
	println!("get nil");
    }
    let mut file = std::fs::File::create(fid).expect("create failed");
    file.write_all(msg.as_bytes());
    res.set_status_code(StatusCode::OK);
}
