#[derive(Queryable)]
pub struct Task {
    pub done: bool,
    pub id: i32,
    pub task: String
}
use super::schema::tasks;

#[derive(Insertable)]
#[table_name="tasks"]
pub struct NewTask<'a> {
    pub task: &'a str
}