use super::schema::nails;

#[derive(Queryable)]
pub struct Nail {
    pub sha512: String,
    pub param: String,
}

#[derive(Insertable)]
#[table_name = "nails"]
pub struct NewNail<'a> {
    pub sha512: &'a str,
    pub param: &'a str,
}
