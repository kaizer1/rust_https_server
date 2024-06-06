//#![deny(warnings)]
//#![deny(warnings)]
#![warn(rust_2018_idioms)]
//#[deny(unused_variables)]


//use std::convert::Infallible;
use bytes::Bytes;
use std::fs::File;
use std::net::SocketAddr;
//use std::ptr::read;
//use futures_util::future::join;
// use futures_util::TryStreamExt;
use hyper::service::{ service_fn }; // make_service_fn
use http_body_util::{BodyExt, combinators::BoxBody, Full, StreamBody};
use hyper::body::Frame;
use hyper::server::conn::http1;
use hyper::{ Method, Request, Response, StatusCode}; // was Body, Server
use tokio_util::io::ReaderStream;
extern crate timer;
extern crate chrono;

use std::sync::mpsc::channel;
use futures_util::task::SpawnExt;
use futures_util::TryFutureExt;
//use serde_json::StreamDeserializer;
use tokio::net::TcpListener;
#[path = "../support/mod.rs"]
mod support;
use support::TokioIo;

static mut DF_K : bool = false;
static INDEX2: &[u8] = b"../my.html";
static NOTFOUND: &[u8] = b"Not Found";

async fn echo(req: Request<hyper::body::Incoming>) -> Result<Response<BoxBody<Bytes, std::io::Error>>>{
              println!(" request start ! ");
      match (req.method(), req.uri().path()) {
        // Serve some instructions at /
        // (&Method::GET, "/") => Ok(Response::new(Body::from(
        //     "Try POSTing data to see yandex maps  to /echo such as: `curl localhost:3000/echo -XPOST -d 'hello world'`",
        // ))),

          (&Method::GET, "/") => send_my_main_file(INDEX2).await,

              // let file = File::open("../my.html");
              // if(file.is_err()){
              //     println!("error open file ");
              //    return Ok(Response::new(BoxBody::from("error open FIle ")))
              // }
              //
              // let file : File = file.unwrap();
              // let reader_stream = ReaderStream::new(file);
              //
              // let stream_body = StreamBody::new(reader_stream.map_ok(Frame::data));
              //
              // let boxed_body = stream_body.boxed();
              //
              // let response = Response::builder()
              //     .status(StatusCode::OK)
              //     .body(boxed_body)
              //     .unwrap();
              //
              // return Ok(response)



          // (&Method::GET, "/2") => Ok(Response::new(BoxBody::from(
          //     "  <div id='map' style='width: 600px; height: 400px'></div>\n\
          //           <script src='https://api-yandex.ru/v3/?apikey=c0d92da7-f2bb-456b-8ca1-9e9d90ddb00e&lang=ru_RU></script>' \n
          //           initMap();    \n
          //                   \n
          //           async function initMap() {     \n
          //              const {YMap, YMapDefaultSchemeLayer} = ymaps3;      \n
          //               const map = new YMap( \n
          //               document.getElementById('map'), \n
          //               { \n
          //                 location: { \n
          //                 center: [37.588144, 55.733842],\n
          //                 zoom: 10  \n
          //                   }\n
          //                }\n
          //       ); \n
          //   map.addChild(new YMapDefaultSchemeLayer()); \n
          //      } ",
          // ))),

          // (&Method::GET, "/1") => {
          //     println!(" 1 call  ");
          //
          //     let mut builder = Response::builder()
          //         .header("dsf", "sldjf")
          //         .body(Body::from(""));
          //
          //     Ok(builder)
          // }

        // Simply echo the body back to the client.

      //   (&Method::POST, "/echo") => {
      //      println!(" calling simple echo ! ");
      // Ok(Response::new(req.into_body()))
      //    },
       
           // (&Method::POST, "/ehi") => {
           //
           //         unsafe {
           //            if DF_K {
           //                println!(" ok TRUE my values !");
           //            } else {
           //
           //    println!(" False (default )calliung EHI is besting's ");
           //              }
           //
           //           }
           //
           //     // let mut builder = Response::builder()
           //       //     .header("LOSDFF", "BASDRAN");
           //            //.status(STATUS::OK);
           //
           //              // builder.body()
           //
           //      // let mut respom = Response::builder();
           //      //respom.header("Ff", "ddslkfjwk jjakskw ")
           //      // .status(StatusCode::OK);
           //
           //       // respom.body(())
           //
           //    Ok(Response::new(BoxBody::from("sdkfwlkejflksdjflksjdf")))
           //
           //  },



        // (&Method::POST, "/echo/uppercase") => {
        //      println!(" in a post/echo/uppercase KSD !");
        //     let chunk_stream = req.into_body().map_ok(|chunk| {
        //         chunk
        //             .iter()
        //             .map(|byte| byte.to_ascii_uppercase())
        //             .collect::<Vec<u8>>()
        //     });
        //     Ok(Response::new(BoxBody::wrap_stream(chunk_stream)))
        // }


          // (&Method::POST, "/") => {
          //
          //     println!(" calling empty string's ");
          //     //let whole_body = hyper::body::to_bytes(req.into_body()).await?;
          //     let whole_boy = Bytes::from(req.into_body());
          //     //let reversed_body = whole_body.iter().rev().cloned().collect::<Vec<u8>>();
          //     Ok(Response::new(BoxBody::from(whole_boy)))
          // }



        // (&Method::POST, "/echo/reversed") => {
        //     //let whole_body = hyper::body::to_bytes(req.into_body()).await?;
        //
        //     let whole_boy = Bytes::from(req.into_body());
        //     //let reversed_body = whole_body.iter().rev().cloned().collect::<Vec<u8>>();
        //     Ok(Response::new(BoxBody::from(whole_boy)))  //was reversed_body
        // }

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

fn not_found() -> Response<BoxBody<Bytes, std::io::Error>> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Full::new(NOTFOUND.into()).map_err(|e| match e {}).boxed())
        .unwrap()
}

async fn send_my_main_file(filename: &str ) -> Result<Response<BoxBody<Bytes, std::io::Error>>>
{
    let file = File::open(filename).await;
    if(file.is_err()){
        println!("error open file ");
       return Ok(not_found());
    }

    let file : File = file.unwrap();
    let reader_stream = ReaderStream::new(file);

    let stream_body = StreamBody::new(reader_stream.map_ok(Frame::data));

    let boxed_body = stream_body.boxed();

    let response = Response::builder()
        .status(StatusCode::OK)
        .body(boxed_body)
        .unwrap();

    return Ok(response)
}

#[tokio::main]
//pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
  async fn main() -> std::result::Result<(), Box<dyn std::error::Error>>
{
    pretty_env_logger::init();

    // For every connection, we must make a `Service` to handle all
    // incoming HTTP requests on said connection.
    //let make_svc = make_service_fn(|_conn| {
        // This is the `Service` that will handle the connection.
        // `service_fn` is a helper to convert a function that
        // returns a Response into a `Service`.
    //     println!(" make connection "); 
  //      async { Ok::<_, Infallible>(service_fn(hello)) }
//    });
     
 //   let timer = timer::Timer::new();
 //   let (tx, rx) = channel();
 //
 //   let _guard = timer.schedule_with_delay(chrono::Duration::seconds(17), move || {
 //    let _ignored = tx.send(());
 // });
  
 
   // let service = make_service_fn(|_| async { Ok::<_, hyper::Error>(service_fn(echo))});

   // was [127, 0, 0, 1], 3000)    
   // let addr = ([127, 0, 0, 1], 9000).into();
   let addr: SocketAddr = ([127, 0, 0, 1], 9000).into();


    //let servMain = async move {
         let listener = TcpListener::bind(addr).await.unwrap();
        loop {
            let (stream, _ ) = listener.accept().await.unwrap();
            let io = TokioIo::new(stream);

            tokio::task::spawn(async  move {
                if let Err(err) = http1::Builder::new()
                    .serve_connection(io, service_fn(echo))
                    .await
                {
                    println!(" error my server connection {:?} ", err)
                }
            });
        }
   // };


   // let server = Server::bind(&addr).serve(service);


    println!("Listening on http://{}", addr);

 

   // server.await?;
    //let _ret = join(servMain).await;

    //rx.recv().unwrap();



   unsafe {
    DF_K = true;
 }
    //df_k1 = true;
 
 
    Ok(())
}
