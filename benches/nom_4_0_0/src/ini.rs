use nom::{alphanumeric, space};

use std::str;

named!(key_value    <&[u8],(&str,&str)>,
  do_parse!(
     key: map_res!(alphanumeric, str::from_utf8)
  >>      opt!(space)
  >>      char!('=')
  >>      opt!(space)
  >> val: map_res!(
           take_while!(call!(|c| c != b'\n' && c != b';')),
           str::from_utf8
         )
  >>      opt!(pair!(char!(';'), take_while!(call!(|c| c != b'\n'))))
  >>      (key, val)
  )
);

wrap_libtest! {
  ini,
  fn bench_ini_keys_and_values(b: &mut test::Bencher) {
    let str = "server=192.0.2.62
  port=143
  file=payroll.dat
  ";

    named!(acc< Vec<(&str,&str)> >, many0!(key_value));

    b.iter(|| acc(str.as_bytes()).unwrap());
  }
}

wrap_libtest! {
  ini,
  fn bench_ini_key_value(b: &mut test::Bencher) {
    let str = "server=192.0.2.62\n";

    b.iter(|| key_value(str.as_bytes()).unwrap());
  }
}
