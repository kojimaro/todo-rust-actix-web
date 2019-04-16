use super::schema::tasks;

#[derive(Deserialize, Queryable)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub is_complete: bool,
}

#[derive(Insertable)]
#[table_name = "tasks"]
pub struct NewTask<'t> {
    pub title: &'t String,
    pub is_complete: bool,
}