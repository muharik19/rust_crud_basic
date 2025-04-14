use actix_web::{dev::{Service, ServiceRequest, ServiceResponse, Transform}, body::{BoxBody, MessageBody}, Error, HttpMessage};
use futures_util::{future::{ok, Ready, LocalBoxFuture}, StreamExt};
use slog::{Logger, Drain, info, o};
use slog_async::Async;
use slog_term::{PlainDecorator, FullFormat, TermDecorator};
use std::{rc::Rc, cell::RefCell};
use std::fs::OpenOptions;

pub struct SlogMiddleware {
    logger: Logger,
}

impl SlogMiddleware {
    pub fn new(logger: Logger) -> Self {
        SlogMiddleware { logger }
    }
}

impl<S, B> Transform<S, ServiceRequest> for SlogMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Transform = SlogMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(SlogMiddlewareService {
            service: Rc::new(RefCell::new(service)),
            logger: self.logger.clone(),
        })
    }
}

pub struct SlogMiddlewareService<S> {
    service: Rc<RefCell<S>>,
    logger: Logger,
}

impl<S, B> Service<ServiceRequest> for SlogMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &self,
        ctx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.borrow_mut().poll_ready(ctx)
    }

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let logger = self.logger.clone();
        let method = req.method().clone();
        let path = req.path().to_owned();
        let headers = format!("{:?}", req.headers());

        let srv = Rc::clone(&self.service);

        Box::pin(async move {
            // Log request body
            let body_bytes = req.take_payload()
                .fold(Ok(bytes::BytesMut::new()), |acc, chunk| async move {
                    match (acc, chunk) {
                        (Ok(mut acc), Ok(data)) => {
                            acc.extend_from_slice(&data);
                            Ok(acc)
                        }
                        _ => Err(()),
                    }
                })
                .await
                .unwrap_or_else(|_| bytes::BytesMut::new())
                .freeze();
            let body_string = String::from_utf8_lossy(&body_bytes);

            info!(logger, "Request received";
                "method" => method.to_string(),
                "path" => path.clone(),
                "headers" => headers.clone(),
                "body" => body_string.to_string()
            );

            // Re-insert body so the handler can use it
            req.set_payload(actix_web::web::Bytes::from(body_bytes).into());

            // Call inner service
            let res = srv.borrow_mut().call(req).await?;

            let status = res.status().clone();
            let res_headers = res.headers().clone();

            // Buffer the response body to log it
            let (req, res_inner) = res.into_parts();
            let body_bytes = match actix_web::body::to_bytes(res_inner.into_body()).await {
                Ok(bytes) => bytes,
                Err(_) => bytes::Bytes::new(),
            };
            let response_body = String::from_utf8_lossy(&body_bytes);

            info!(logger, "Response sent";
                "status" => status.as_u16(),
                "headers" => format!("{:?}", res_headers),
                "body" => response_body.to_string()
            );

            // Rebuild full response with buffered body using headers from res_headers
            let mut builder = actix_web::HttpResponse::build(status);
            for (key, value) in res_headers.iter() {
                builder.insert_header((key.clone(), value.clone()));
            }
            let new_response = builder.body(BoxBody::new(body_bytes));
            let rebuilt_response = ServiceResponse::new(req, new_response);

            Ok(rebuilt_response)
        })
    }
}

pub fn init_logger() -> (Logger, Logger) {
    let log_path = "target/your_log_file_path.log";
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(log_path)
        .unwrap();

    // File logger with custom timestamp
    let decorator_file = PlainDecorator::new(file);
    let drain_file = FullFormat::new(decorator_file)
        .use_custom_timestamp(|w| {
            let now = chrono::Local::now();
            write!(w, "{}", now.format("%Y-%m-%d %H:%M:%S%.3f"))
        })
        .build()
        .fuse();
    let drain_file = Async::new(drain_file).build().fuse();
    let logger_file = Logger::root(drain_file, o!());

    // Terminal logger with custom timestamp
    let decorator = TermDecorator::new().build();
    let drain = FullFormat::new(decorator)
        .use_custom_timestamp(|w| {
            let now = chrono::Local::now();
            write!(w, "{}", now.format("%Y-%m-%d %H:%M:%S%.3f"))
        })
        .build()
        .fuse();
    let drain = Async::new(drain).build().fuse();
    let logger_terminal = Logger::root(drain, o!());

    (logger_file, logger_terminal)
}
