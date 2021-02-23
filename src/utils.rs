use actix_web::client::ClientResponse;

pub trait ClientResponseEx {
    fn content_length(&self) -> Option<usize>;
}

impl<T> ClientResponseEx for ClientResponse<T> {
    fn content_length(&self) -> Option<usize> {
        let mut len = None;
        if let Some(l) = self.headers().get(&actix_web::http::header::CONTENT_LENGTH) {
            if let Ok(s) = l.to_str() {
                if let Ok(l) = s.parse::<usize>() {
                    len = Some(l);
                }
            }
        }
        len
    }
}
