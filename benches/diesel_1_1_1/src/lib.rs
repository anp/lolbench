#![allow(non_snake_case)]
#![recursion_limit = "128"]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate lolbench_support;

mod backend_specifics;
mod schema;

use self::schema::{posts, users, NewUser, Post, TestConnection, User};
use diesel::*;

fn connection() -> TestConnection {
    schema::connection()
}

macro_rules! bench_trivial_query {
    ($n:expr, $name:ident, $name_boxed:ident) => {
        wrap_libtest! {
            fn $name(b: &mut Bencher) {
                let conn = connection();

                let data: Vec<_> = (0..$n).map(|i| {
                    NewUser::new(&format!("User {}", i), None)
                }).collect();
                insert_into(users::table).values(&data).execute(&conn).unwrap();

                b.iter(|| {
                    users::table.load::<User>(&conn).unwrap()
                })
            }
        }

        wrap_libtest! {
            fn $name_boxed(b: &mut Bencher) {
                let conn = connection();

                let data: Vec<_> = (0..$n).map(|i| {
                    NewUser::new(&format!("User {}", i), None)
                }).collect();
                insert_into(users::table).values(&data).execute(&conn).unwrap();

                b.iter(|| {
                    users::table.into_boxed().load::<User>(&conn).unwrap()
                })
            }
        }
    };
}

bench_trivial_query!(
    1,
    bench_trivial_query_selecting______1_row,
    bench_trivial_query_selecting______1_row_boxed
);
bench_trivial_query!(
    10,
    bench_trivial_query_selecting_____10_rows,
    bench_trivial_query_selecting_____10_rows_boxed
);
bench_trivial_query!(
    100,
    bench_trivial_query_selecting____100_rows,
    bench_trivial_query_selecting____100_rows_boxed
);
bench_trivial_query!(
    1_000,
    bench_trivial_query_selecting__1_000_rows,
    bench_trivial_query_selecting__1_000_rows_boxed
);
bench_trivial_query!(
    10_000,
    bench_trivial_query_selecting_10_000_rows,
    bench_trivial_query_selecting_10_000_rows_boxed
);

macro_rules! bench_medium_complex_query {
    ($n:expr, $name:ident, $name_boxed:ident) => {
        wrap_libtest! {
            fn $name(b: &mut Bencher) {
                let conn = connection();

                let data: Vec<_> = (0..$n).map(|i| {
                    let hair_color = if i % 2 == 0 { "black" } else { "brown" };
                    NewUser::new(&format!("User {}", i), Some(hair_color))
                }).collect();
                insert_into(users::table).values(&data).execute(&conn).unwrap();

                b.iter(|| {
                    use schema::users::dsl::*;
                    let target = users.left_outer_join(posts::table)
                        .filter(hair_color.eq("black"))
                        .order(name.desc());
                    target.load::<(User, Option<Post>)>(&conn).unwrap()
                })
            }
        }

        wrap_libtest! {
            fn $name_boxed(b: &mut Bencher) {
                let conn = connection();

                let data: Vec<_> = (0..$n).map(|i| {
                    let hair_color = if i % 2 == 0 { "black" } else { "brown" };
                    NewUser::new(&format!("User {}", i), Some(hair_color))
                }).collect();
                insert_into(users::table).values(&data).execute(&conn).unwrap();

                b.iter(|| {
                    use schema::users::dsl::*;
                    let target = users.left_outer_join(posts::table)
                        .filter(hair_color.eq("black"))
                        .order(name.desc())
                        .into_boxed();
                    target.load::<(User, Option<Post>)>(&conn).unwrap()
                })
            }
        }
    };
}

bench_medium_complex_query!(
    1,
    bench_medium_complex_query_selecting______1_row,
    bench_medium_complex_query_selecting______1_row_boxed
);
bench_medium_complex_query!(
    10,
    bench_medium_complex_query_selecting_____10_rows,
    bench_medium_complex_query_selecting_____10_rows_boxed
);
bench_medium_complex_query!(
    100,
    bench_medium_complex_query_selecting____100_rows,
    bench_medium_complex_query_selecting____100_rows_boxed
);
bench_medium_complex_query!(
    1_000,
    bench_medium_complex_query_selecting__1_000_rows,
    bench_medium_complex_query_selecting__1_000_rows_boxed
);
bench_medium_complex_query!(
    10_000,
    bench_medium_complex_query_selecting_10_000_rows,
    bench_medium_complex_query_selecting_10_000_rows_boxed
);
