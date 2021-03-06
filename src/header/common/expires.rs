use header::HttpDate;

/// The `Expires` header field.
#[derive(Copy, PartialEq, Clone, Debug)]
pub struct Expires(pub HttpDate);
impl_header!(Expires, "Expires", HttpDate);

bench_header!(imf_fixdate, Expires, { vec![b"Sun, 07 Nov 1994 08:48:37 GMT".to_vec()] });
bench_header!(rfc_850, Expires, { vec![b"Sunday, 06-Nov-94 08:49:37 GMT".to_vec()] });
bench_header!(asctime, Expires, { vec![b"Sun Nov  6 08:49:37 1994".to_vec()] });
