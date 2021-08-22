use std::convert::Infallible;
use warp::reply::Response;
use warp::{Filter, Rejection, reject};
use warp::reject::Reject;
use std::fmt::Debug;
use futures::{future, TryFutureExt};

#[tokio::main]
async fn main() {
    let filter_obj = warp::path("hello").and(auth()).and_then(hello);

    warp::serve(filter_obj).run(([127, 0, 0, 1], 3030)).await;
}

fn auth() -> impl Filter<Extract = (), Error = Rejection> + Clone {
    warp::any()
        .and(warp::filters::header::header("token"))
        .or(warp::any().map(|| "".to_string()))
        .unify()
        .and_then(|token_id: String|{
            println!("token:{}", token_id);
            //future::err(crate::reject::method_not_allowed())
            //warp::reject::custom(ErrMsg("你好a ".to_string())) as Rejection
            if token_id.is_empty(){
               return future::err(warp::reject::custom(ErrMsg("你好a ".to_string())) as Rejection);
            }

            return future::ok(());
        }).untuple_one()
}

async fn hello() -> Result<impl warp::Reply, warp::Rejection> {
    Ok("hello world")
}

struct Context {
    user_id: Option<String>,
}

impl Clone for Context {
    fn clone(&self) -> Self {
        Context {
            user_id: self.user_id.clone(),
        }
    }
}

unsafe impl Send for Context {}

impl warp::Reply for Context {
    fn into_response(self) -> Response {
        Response::new(format!("message").into())
    }
}

struct ErrMsg(String);
impl Debug for ErrMsg{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result{
        f.debug_tuple(self.0.clone().as_str()).finish()
    }
}
impl Reject for ErrMsg {

}
