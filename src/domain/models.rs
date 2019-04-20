use super::schema::tasks;

#[derive(Deserialize, Serialize, Queryable)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub is_complete: bool,
}

#[derive(Insertable)]
#[table_name = "tasks"]
pub struct NewTask<'a> {
    pub title: &'a String,
    pub is_complete: &'a bool,
}