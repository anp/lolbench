use criterion::Bencher;
use nom::{alphanumeric, recognize_float};

use std::collections::HashMap;
use std::str;

#[derive(Debug, PartialEq)]
pub enum JsonValue {
    Str(String),
    Num(f32),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

named!(float<f32>, flat_map!(recognize_float, parse_to!(f32)));

//FIXME: verify how json strings are formatted
named!(
    string<&str>,
    delimited!(
        char!('\"'),
        map_res!(
            escaped!(call!(alphanumeric), '\\', one_of!("\"n\\")),
            str::from_utf8
        ),
        //map_res!(escaped!(take_while1!(is_alphanumeric), '\\', one_of!("\"n\\")), str::from_utf8),
        char!('\"')
    )
);

named!(
    array<Vec<JsonValue>>,
    ws!(delimited!(
        char!('['),
        separated_list!(char!(','), value),
        char!(']')
    ))
);

named!(
    key_value<(&str, JsonValue)>,
    ws!(separated_pair!(string, char!(':'), value))
);

named!(
    hash<HashMap<String, JsonValue>>,
    ws!(map!(
        delimited!(
            char!('{'),
            separated_list!(char!(','), key_value),
            char!('}')
        ),
        |tuple_vec| {
            tuple_vec
                .into_iter()
                .map(|(k, v)| (String::from(k), v))
                .collect()
            /*let mut h: HashMap<String, JsonValue> = HashMap::new();
        for (k, v) in tuple_vec {
          h.insert(String::from(k), v);
        }
        h*/
        }
    ))
);

named!(
    value<JsonValue>,
    ws!(alt!(
      hash   => { |h|   JsonValue::Object(h)            } |
      array  => { |v|   JsonValue::Array(v)             } |
      string => { |s|   JsonValue::Str(String::from(s)) } |
      float  => { |num| JsonValue::Num(num)             }
    ))
);

#[bench]
fn json_bench(b: &mut Bencher) {
    let data = &b"  { \"a\"\t: 42,
  \"b\": [ \"x\", \"y\", 12 ] ,
  \"c\": { \"hello\" : \"world\"
  }
  }  \0";

    //println!("data:\n{:?}", value(&data[..]));
    b.iter(|| value(&data[..]).unwrap());
}

#[bench]
fn recognize_float_bytes(b: &mut Bencher) {
    use nom::recognize_float;
    b.iter(|| recognize_float(&b"-1.234E-12"[..]));
}

#[bench]
fn recognize_float_str(b: &mut Bencher) {
    use nom::recognize_float;
    b.iter(|| recognize_float("-1.234E-12"));
}

#[bench]
fn float_bytes(b: &mut Bencher) {
    use nom::float;
    b.iter(|| float(&b"-1.234E-12"[..]));
}

#[bench]
fn float_str(b: &mut Bencher) {
    use nom::float_s;
    b.iter(|| float_s("-1.234E-12"));
}
