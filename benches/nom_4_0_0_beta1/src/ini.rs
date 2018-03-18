use nom::{alphanumeric, multispace, space};

use std::str;
use std::collections::HashMap;

named!(
    category<&str>,
    map_res!(
        delimited!(char!('['), take_while!(call!(|c| c != b']')), char!(']')),
        str::from_utf8
    )
);

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

named!(keys_and_values<&[u8], HashMap<&str, &str> >,
  map!(
    many0!(terminated!(key_value, opt!(multispace))),
    |vec: Vec<_>| vec.into_iter().collect()
  )
);

named!(category_and_keys<&[u8],(&str,HashMap<&str,&str>)>,
  do_parse!(
    category: category         >>
              opt!(multispace) >>
    keys: keys_and_values      >>
    (category, keys)
  )
);

named!(categories<&[u8], HashMap<&str, HashMap<&str,&str> > >,
  map!(
    many0!(
      separated_pair!(
        category,
        opt!(multispace),
        map!(
          many0!(terminated!(key_value, opt!(multispace))),
          |vec: Vec<_>| vec.into_iter().collect()
        )
      )
    ),
    |vec: Vec<_>| vec.into_iter().collect()
  )
);

wrap_libtest! {
  fn bench_ini(b: &mut test::Bencher) {
    let str = "[owner]
  name=John Doe
  organization=Acme Widgets Inc.

  [database]
  server=192.0.2.62
  port=143
  file=payroll.dat
  ";

    b.iter(|| categories(str.as_bytes()).unwrap());
  }
}

wrap_libtest! {
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
  fn bench_ini_key_value(b: &mut test::Bencher) {
    let str = "server=192.0.2.62\n";

    b.iter(|| key_value(str.as_bytes()).unwrap());
  }
}
