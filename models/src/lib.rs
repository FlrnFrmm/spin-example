#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq)]
pub struct Todo {
		pub id: String,
		pub title: String,
}

impl Todo {
		pub fn new(title: String) -> Todo {
				let id = uuid::Uuid::new_v4().to_string();
				Todo { id, title }
		}
}