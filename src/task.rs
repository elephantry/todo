#[derive(Serialize, Debug, Clone, elephantry::Entity)]
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
    pub fn all(conn: &elephantry::Connection) -> Vec<Task> {
        conn.find_all::<Model>(Some("order by id desc")).unwrap().collect()
    }

    pub fn insert(todo: Todo, conn: &elephantry::Connection) -> bool {
        let t = Task { id: None, description: todo.description, completed: false };
        conn.insert_one::<Model>(&t).is_ok()
    }

    pub fn toggle_with_id(id: i32, conn: &elephantry::Connection) -> bool {
        let mut task = match conn.find_by_pk::<Model>(&pk!(id)) {
            Ok(Some(task)) => task,
            _ => return false,
        };

        task.completed = !task.completed;

        conn.update_one::<Model>(&pk!(id), &task).is_ok()
    }

    pub fn delete_with_id(id: i32, conn: &elephantry::Connection) -> bool {
        conn.delete_by_pk::<Model>(&pk!(id)).is_ok()
    }

    #[cfg(test)]
    pub fn delete_all(conn: &elephantry::Connection) -> bool {
        conn.delete_where::<Model>("1 = 1", &[]).is_ok()
    }
}
