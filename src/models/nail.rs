use crate::schema::nails;
use diesel::PgConnection;

#[derive(Serialize, Deserialize)]
pub struct NailList(pub Vec<Nail>);

#[derive(Queryable, Serialize, Deserialize)]
pub struct Nail {
    pub id: i32,
    pub sha512: String,
    pub body: String,
}

#[derive(Insertable, Deserialize, AsChangeset)]
#[table_name = "nails"]
pub struct NewNail {
    pub sha512: String,
    pub body: String,
}

impl NailList {
    pub fn list(connection: &PgConnection) -> Self {
        use diesel::RunQueryDsl;
        use diesel::QueryDsl;
        use crate::schema::nails::dsl::*;

        let result = 
            nails
                .limit(10)
                .load::<Nail>(connection)
                .expect("Error loading nails");

        NailList(result)
    }
}

impl NewNail {
    pub fn create(&self, connection: &PgConnection) -> Result<Nail, diesel::result::Error> {
        use diesel::RunQueryDsl;

        diesel::insert_into(nails::table)
            .values(self)
            .get_result(connection)
    }
}

impl Nail {
    pub fn find(id: &i32, connection: &PgConnection) -> Result<Nail, diesel::result::Error> {
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;

        nails::table.find(id).first(connection)
    }

    pub fn destroy(id: &i32, connection: &PgConnection) -> Result<(), diesel::result::Error> {
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;
        use crate::schema::nails::dsl;

        diesel::delete(dsl::nails.find(id)).execute(connection)?;
        Ok(())
    }

    pub fn update(id: &i32, new_nail: &NewNail, connection: &PgConnection) ->
     Result<(), diesel::result::Error> {
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;
        use crate::schema::nails::dsl;

        diesel::update(dsl::nails.find(id))
            .set(new_nail)
            .execute(connection)?;
        Ok(())
    }
}
