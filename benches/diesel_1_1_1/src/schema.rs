use diesel::*;

table! {
    comments (id) {
        id -> Integer,
        post_id -> Integer,
        text -> Text,
    }
}

table! {
    composite_fk (id) {
        id -> Nullable<Integer>,
        post_id -> Integer,
        user_id -> Integer,
    }
}

table! {
    cyclic_fk_1 (id) {
        id -> Nullable<Integer>,
        cyclic_fk_2_id -> Nullable<Binary>,
    }
}

table! {
    cyclic_fk_2 (id) {
        id -> Nullable<Integer>,
        cyclic_fk_1_id -> Nullable<Binary>,
    }
}

table! {
    fk_doesnt_reference_pk (id) {
        id -> Nullable<Integer>,
        random -> Nullable<Text>,
    }
}

table! {
    fk_inits (id) {
        id -> Nullable<Integer>,
    }
}

table! {
    fk_tests (id) {
        id -> Nullable<Integer>,
        fk_id -> Integer,
    }
}

table! {
    followings (user_id, post_id) {
        user_id -> Integer,
        post_id -> Integer,
        email_notifications -> Bool,
    }
}

table! {
    infer_all_the_bools (col1) {
        col1 -> Bool,
        col2 -> Bool,
        col3 -> Bool,
        col4 -> Bool,
    }
}

table! {
    infer_all_the_datetime_types (dt) {
        dt -> Timestamp,
        date -> Date,
        time -> Time,
        timestamp -> Timestamp,
    }
}

table! {
    infer_all_the_floats (col1) {
        col1 -> Float,
        col2 -> Float,
        col3 -> Double,
        col4 -> Double,
        col5 -> Double,
        col6 -> Double,
    }
}

table! {
    infer_all_the_ints (col1) {
        col1 -> Integer,
        col2 -> Integer,
        col3 -> Integer,
        col4 -> Integer,
        col5 -> SmallInt,
        col6 -> SmallInt,
        col7 -> SmallInt,
        col8 -> BigInt,
        col9 -> BigInt,
        col10 -> BigInt,
        col11 -> SmallInt,
        col12 -> Integer,
        col13 -> BigInt,
    }
}

table! {
    infer_all_the_strings (col1) {
        col1 -> Text,
        col2 -> Text,
        col3 -> Text,
        col4 -> Text,
        col5 -> Text,
        col6 -> Text,
        col7 -> Text,
        col8 -> Text,
        col9 -> Binary,
        col10 -> Binary,
    }
}

table! {
    likes (comment_id, user_id) {
        comment_id -> Integer,
        user_id -> Integer,
    }
}

table! {
    multiple_fks_to_same_table (id) {
        id -> Nullable<Integer>,
        post_id_1 -> Nullable<Binary>,
        post_id_2 -> Nullable<Binary>,
    }
}

table! {
    nullable_doubles (id) {
        id -> Nullable<Integer>,
        n -> Nullable<Double>,
    }
}

table! {
    nullable_table (id) {
        id -> Integer,
        value -> Nullable<Integer>,
    }
}

table! {
    numbers (n) {
        n -> Nullable<Integer>,
    }
}

table! {
    points (x, y) {
        x -> Integer,
        y -> Integer,
    }
}

table! {
    posts (id) {
        id -> Integer,
        user_id -> Integer,
        title -> Text,
        body -> Nullable<Text>,
    }
}

table! {
    precision_numbers (n) {
        n -> Double,
    }
}

table! {
    self_referential_fk (id) {
        id -> Nullable<Integer>,
        parent_id -> Integer,
    }
}

table! {
    special_comments (id) {
        id -> Nullable<Integer>,
        special_post_id -> Integer,
    }
}

table! {
    special_posts (id) {
        id -> Nullable<Integer>,
        user_id -> Integer,
        title -> Text,
    }
}

table! {
    trees (id) {
        id -> Integer,
        parent_id -> Nullable<Integer>,
    }
}

table! {
    users (id) {
        id -> Integer,
        name -> Text,
        hair_color -> Nullable<Text>,
    }
}

table! {
    users_with_name_pk (name) {
        name -> Nullable<Text>,
    }
}

table! {
    with_keywords (fn_) {
        #[sql_name = "fn"]
        fn_ -> Integer,
        #[sql_name = "let"]
        let_ -> Integer,
        #[sql_name = "extern"]
        extern_ -> Integer,
    }
}

joinable!(comments -> posts (post_id));
joinable!(fk_tests -> fk_inits (fk_id));
joinable!(followings -> posts (post_id));
joinable!(followings -> users (user_id));
joinable!(likes -> comments (comment_id));
joinable!(likes -> users (user_id));
joinable!(posts -> users (user_id));

allow_tables_to_appear_in_same_query!(
    comments,
    composite_fk,
    cyclic_fk_1,
    cyclic_fk_2,
    fk_doesnt_reference_pk,
    fk_inits,
    fk_tests,
    followings,
    infer_all_the_bools,
    infer_all_the_datetime_types,
    infer_all_the_floats,
    infer_all_the_ints,
    infer_all_the_strings,
    likes,
    multiple_fks_to_same_table,
    nullable_doubles,
    nullable_table,
    numbers,
    points,
    posts,
    precision_numbers,
    self_referential_fk,
    special_comments,
    special_posts,
    trees,
    users,
    users_with_name_pk,
    with_keywords,
);

#[derive(PartialEq, Eq, Debug, Clone, Queryable, Identifiable, Insertable, AsChangeset,
         Associations, QueryableByName)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub name: String,
    pub hair_color: Option<String>,
}

impl User {
    pub fn new(id: i32, name: &str) -> Self {
        User {
            id: id,
            name: name.to_string(),
            hair_color: None,
        }
    }

    pub fn with_hair_color(id: i32, name: &str, hair_color: &str) -> Self {
        User {
            id: id,
            name: name.to_string(),
            hair_color: Some(hair_color.to_string()),
        }
    }

    pub fn new_post(&self, title: &str, body: Option<&str>) -> NewPost {
        NewPost::new(self.id, title, body)
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Queryable, Identifiable, Associations)]
#[belongs_to(Post)]
pub struct Comment {
    id: i32,
    post_id: i32,
    text: String,
}

impl Comment {
    pub fn new(id: i32, post_id: i32, text: &str) -> Self {
        Comment {
            id: id,
            post_id: post_id,
            text: text.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Queryable, Insertable, Associations, Identifiable)]
#[belongs_to(User)]
#[belongs_to(Post)]
#[table_name = "followings"]
#[primary_key(user_id, post_id)]
pub struct Following {
    pub user_id: i32,
    pub post_id: i32,
    pub email_notifications: bool,
}

pub use backend_specifics::*;

#[derive(Debug, PartialEq, Eq, Queryable, Clone, Insertable, AsChangeset)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
    pub hair_color: Option<String>,
}

impl NewUser {
    pub fn new(name: &str, hair_color: Option<&str>) -> Self {
        NewUser {
            name: name.to_string(),
            hair_color: hair_color.map(|s| s.to_string()),
        }
    }
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost {
    user_id: i32,
    title: String,
    body: Option<String>,
}

impl NewPost {
    pub fn new(user_id: i32, title: &str, body: Option<&str>) -> Self {
        NewPost {
            user_id: user_id,
            title: title.into(),
            body: body.map(|b| b.into()),
        }
    }
}

#[derive(Debug, Clone, Copy, Insertable)]
#[table_name = "comments"]
pub struct NewComment<'a>(
    #[column_name = "post_id"] pub i32,
    #[column_name = "text"] pub &'a str,
);

#[derive(PartialEq, Eq, Debug, Clone, Insertable, Associations)]
#[table_name = "fk_tests"]
pub struct FkTest {
    id: i32,
    fk_id: i32,
}

impl FkTest {
    pub fn new(id: i32, fk_id: i32) -> Self {
        FkTest {
            id: id,
            fk_id: fk_id,
        }
    }
}

#[derive(Queryable, Insertable)]
#[table_name = "nullable_table"]
pub struct NullableColumn {
    id: i32,
    value: Option<i32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Queryable, Insertable, Identifiable, Associations)]
#[table_name = "likes"]
#[primary_key(user_id, comment_id)]
#[belongs_to(User)]
#[belongs_to(Comment)]
pub struct Like {
    pub user_id: i32,
    pub comment_id: i32,
}

pub type TestConnection = SqliteConnection;

pub fn connection() -> TestConnection {
    let result = connection_without_transaction();
    result.execute("PRAGMA foreign_keys = ON").unwrap();
    result.begin_test_transaction().unwrap();
    result
}

embed_migrations!("migrations/sqlite");

pub fn connection_without_transaction() -> TestConnection {
    let connection = SqliteConnection::establish(":memory:").unwrap();
    embedded_migrations::run(&connection).unwrap();
    connection
}

sql_function!(_nextval, nextval_t, (a: types::VarChar) -> sql_types::BigInt);
