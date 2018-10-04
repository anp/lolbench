use nom::IResult;

#[cfg_attr(rustfmt, rustfmt_skip)]
#[derive(Debug)]
struct Request<'a> {
  method:  &'a [u8],
  uri:     &'a [u8],
  version: &'a [u8],
}

#[derive(Debug)]
struct Header<'a> {
    name: &'a [u8],
    value: Vec<&'a [u8]>,
}

#[cfg_attr(rustfmt, rustfmt_skip)]
#[cfg_attr(feature = "cargo-clippy", allow(match_same_arms))]
fn is_token(c: u8) -> bool {
  match c {
    128...255 => false,
    0...31    => false,
    b'('      => false,
    b')'      => false,
    b'<'      => false,
    b'>'      => false,
    b'@'      => false,
    b','      => false,
    b';'      => false,
    b':'      => false,
    b'\\'     => false,
    b'"'      => false,
    b'/'      => false,
    b'['      => false,
    b']'      => false,
    b'?'      => false,
    b'='      => false,
    b'{'      => false,
    b'}'      => false,
    b' '      => false,
    _         => true,
  }
}

fn not_line_ending(c: u8) -> bool {
    c != b'\r' && c != b'\n'
}

fn is_space(c: u8) -> bool {
    c == b' '
}

fn is_not_space(c: u8) -> bool {
    c != b' '
}
fn is_horizontal_space(c: u8) -> bool {
    c == b' ' || c == b'\t'
}

fn is_version(c: u8) -> bool {
    c >= b'0' && c <= b'9' || c == b'.'
}

named!(line_ending, alt!(tag!("\r\n") | tag!("\n")));

#[cfg_attr(rustfmt, rustfmt_skip)]
fn request_line(input: &[u8]) -> IResult<&[u8], Request> {
  do_parse!(input,
    method: take_while1!(is_token)     >>
            take_while1!(is_space)     >>
    url:    take_while1!(is_not_space) >>
            take_while1!(is_space)     >>
    version: http_version              >>
    line_ending                        >>

    (Request {
        method: method,
        uri:    url,
        version: version,
    }))
}

#[cfg_attr(rustfmt, rustfmt_skip)]
named!(http_version, do_parse!(
    tag!("HTTP/")                     >>
    version: take_while1!(is_version) >>

    (version)));

#[cfg_attr(rustfmt, rustfmt_skip)]
named!(message_header_value, do_parse!(
          take_while1!(is_horizontal_space) >>
    data: take_while1!(not_line_ending)     >>
    line_ending                             >>

    (data)));

#[cfg_attr(rustfmt, rustfmt_skip)]
fn message_header(input: &[u8]) -> IResult<&[u8], Header> {
  do_parse!(input,
    name:   take_while1!(is_token)       >>
            char!(':')                   >>
    values: many1!(message_header_value) >>

    (Header {
        name: name,
        value: values,
    }))
}

fn request(input: &[u8]) -> IResult<&[u8], (Request, Vec<Header>)> {
    do_parse!(
        input,
        req: request_line >> h: many1!(message_header) >> line_ending >> (req, h)
    )
}

fn parse(data: &[u8]) -> Option<Vec<(Request, Vec<Header>)>> {
    let mut buf = &data[..];
    let mut v = Vec::new();
    loop {
        match request(buf) {
            Ok((b, r)) => {
                buf = b;
                v.push(r);

                if b.is_empty() {
                    //println!("{}", i);
                    break;
                }
            }
            Err(_) => return None,
        }
    }

    Some(v)
}

wrap_libtest! {
http,
fn one_test(b: &mut Bencher) {
        let data = &b"GET / HTTP/1.1
    Host: www.reddit.com
    User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.8; rv:15.0) Gecko/20100101 Firefox/15.0.1
    Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8
    Accept-Language: en-us,en;q=0.5
    Accept-Encoding: gzip, deflate
    Connection: keep-alive"
    [..];
    b.iter(|| parse(data));
}
}
