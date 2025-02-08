use ohkami::prelude::*;
use ohkami::format::{JSON, Query};

#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error> {
    let o = Ohkami::new((
        "/".GET(|| async {"Hello, AWS Lambda!"}),
        "/hello"
            .GET(hello_by_query)
            .POST(hello_by_json),
    ));

    lambda_runtime::run(o).await
}

#[derive(Deserialize)]
struct HelloRequest<'req> {
    name: Option<&'req str>,
    repeat: Option<usize>,
}
impl HelloRequest<'_> {
    fn into_message(self) -> String {
        let name = self.name.unwrap_or("world");
        let repeat = self.repeat.unwrap_or(1);
        vec![format!("Hello, {name}!"); repeat].join(" ")
    }
}

async fn hello_by_query(
    Query(req): Query<HelloRequest<'_>>,
) -> String {
    req.into_message()
}

async fn hello_by_json(
    JSON(req): JSON<HelloRequest<'_>>,
) -> String {
    req.into_message()
}
