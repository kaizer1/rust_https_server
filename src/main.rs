#![deny(warnings)]

//use std::convert::Infallible;

use futures_util::TryStreamExt;
use hyper::service::{make_service_fn, service_fn};
use hyper::body;
use hyper::{Body, Method, Request, Response, Server, StatusCode};
extern crate timer;
extern crate chrono;





use std::sync::mpsc::channel;
//use std::fmt::Display;

static mut DF_K : bool = false;

async fn echo(req: Request<Body>) -> Result<Response<Body>, hyper::Error>{
              println!(" request start ! ");
      match (req.method(), req.uri().path()) {
        // Serve some instructions at /
        (&Method::GET, "/") => Ok(Response::new(Body::from(
            "Try POSTing data to /echo such as: `curl localhost:3000/echo -XPOST -d 'hello world'`",
        ))),

        // Simply echo the body back to the client.
        (&Method::POST, "/echo") => {
           println!(" calling simple echo ! ");

       //     println!(" call my body to send {}", req.body());
 
         println!("{:?}", req);
   
        // await!(req.into_body().compat().try_concat());     
    //         let _bb = req.into_body();
   //  let body = req.into_body().map_err(|_| ()).fold(vec![], |mut acc, chunk| {
   //  acc.extend_from_slice(&chunk);
   //   Ok(acc)
    //   }).and_then(|v| String::from_utf8(v).map_err(|_| ()));
     //println!("Body: \n{}", body.wait().unwrap());       
              
      let bytes = body::to_bytes(req.into_body()).await?;
       println!( "\n my send's == {:?} ", String::from_utf8(bytes.to_vec()));
   //             format!(" aa {} ", String::from_utf8(bytes.to_vec()));
     
         //   println!("{}", await!(req.into_body().concat2())?.to_vec());          
           
      Ok(Response::new(Body::from(" sdflkjsdklkldkjdj LOS SENDS")))
         },
       
           (&Method::POST, "/ehi") => { 
                      
                   unsafe {  
                      if DF_K {
                          println!(" ok TRUE my values !");
                      } else {
   
              println!(" False (default )calliung EHI is besting's "); 
                        }

                     }
                 
               // let mut builder = Response::builder()
                 //     .header("LOSDFF", "BASDRAN");
                      //.status(STATUS::OK);       
                           
                        // builder.body()
  
             //   let mut respom = Response::builder();
             //   respom.header("Ff", "ddslkfjwk jjakskw ")
             //    .status(StatusCode::OK);
              
             //Ok(Response::new(Body::from(respom)));             
  
               Ok(Response::new(Body::from("sdkfwlkejflksdjflksjdf")))   
 
            },
        // Convert to uppercase before sending back to client using a stream.
        (&Method::POST, "/echo/uppercase") => {
             println!(" in a post/echo/uppercase KSD !");
            let chunk_stream = req.into_body().map_ok(|chunk| {
                chunk
                    .iter()
                    .map(|byte| byte.to_ascii_uppercase())
                    .collect::<Vec<u8>>()
            });
            Ok(Response::new(Body::wrap_stream(chunk_stream)))
        }

        // Reverse the entire body before sending back to the client.
        //
        // Since we don't know the end yet, we can't simply stream
        // the chunks as they arrive as we did with the above uppercase endpoint.
        // So here we do `.await` on the future, waiting on concatenating the full body,
        // then afterwards the content can be reversed. Only then can we return a `Response`.
        (&Method::POST, "/echo/reversed") => {
            let whole_body = hyper::body::to_bytes(req.into_body()).await?;

            let reversed_body = whole_body.iter().rev().cloned().collect::<Vec<u8>>();
            Ok(Response::new(Body::from(reversed_body)))
        }

        // Return the 404 Not Found for other routes.
        _ => {
                println!(" not los found ! ");
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

//async fn hello(_: Request<Body>) -> Result<Response<Body>, Infallible> {
//    Ok(Response::new(Body::from("Hello World!")))
//}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    
     pretty_env_logger::init();
     
     // calling 
  
    // For every connection, we must make a `Service` to handle all
    // incoming HTTP requests on said connection.
    //let make_svc = make_service_fn(|_conn| {
        // This is the `Service` that will handle the connection.
        // `service_fn` is a helper to convert a function that
        // returns a Response into a `Service`.
    //     println!(" make connection "); 
  //      async { Ok::<_, Infallible>(service_fn(hello)) }
//    });
     
   let timer = timer::Timer::new();
   let (tx, rx) = channel();
  
   let _guard = timer.schedule_with_delay(chrono::Duration::seconds(17), move || {
    let _ignored = tx.send(()); 
 }); 
  
 
   let service = make_service_fn(|_| async { Ok::<_, hyper::Error>(service_fn(echo))});

   // was [127, 0, 0, 1], 3000)    
    let addr = ([127, 0, 0, 1], 9000).into();

    let server = Server::bind(&addr).serve(service);
  //  server.allow_reuse_address = true;

    println!("Listening on http://{}", addr);

 

    server.await?;
 
    rx.recv().unwrap();
   unsafe {
    DF_K = true;
 }
    //df_k1 = true;
 
 
    Ok(())
}
