use elephantry::pk;
use rocket::serde::Serialize;
use crate::DbConn;

#[derive(Serialize, Debug, Clone, elephantry::Entity)]
#[serde(crate = "rocket::serde")]
#[elephantry(model = "Model", structure = "Structure", relation = "tasks")]
pub struct Task {
    #[elephantry(pk)]
    pub id: Option<i32>,
    pub description: String,
    pub completed: bool
}

#[derive(FromForm)]
pub struct Todo {
    pub description: String,
}

impl Task {
    pub async fn all(conn: &DbConn) -> elephantry::Result<Vec<Task>> {
        conn.run(|c| {
            c.find_all::<Model>(Some("order by id desc")).map(|x| x.collect())
        }).await
    }

    pub async fn insert(todo: Todo, conn: &DbConn) -> elephantry::Result<usize> {
        conn.run(|c| {
            let t = Task { id: None, description: todo.description, completed: false };
            c.insert_one::<Model>(&t).map(|_| 1)
        }).await
    }

    pub async fn toggle_with_id(id: i32, conn: &DbConn) -> elephantry::Result<usize> {
        conn.run(move |c| {
            let mut task = match c.find_by_pk::<Model>(&pk!(id)) {
                Ok(Some(task)) => task,
                _ => return Ok(0),
            };

            task.completed = !task.completed;

            c.update_one::<Model>(&pk!(id), &task).map(|_| 1)
        }).await
    }

    pub async fn delete_with_id(id: i32, conn: &DbConn) -> elephantry::Result<usize> {
        conn.run(move |c| {
            c.delete_by_pk::<Model>(&pk!(id)).map(|_| 1)
        }).await
    }

    #[cfg(test)]
    pub async fn delete_all(conn: &DbConn) -> elephantry::Result<usize> {
        conn.run(|c| {
            c.delete_where::<Model>("1 = 1", &[]).map(|x| x.len())
        }).await
    }
}
