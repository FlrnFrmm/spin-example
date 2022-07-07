use yew::prelude::*;
use models::Todo;

pub enum Msg {
	Add(Todo),
	Remove(Todo)
}

pub struct App {
	todos: Vec<Todo>
}

impl Component for App {
	type Message = Msg;
	type Properties = ();

	fn create(_ctx: &Context<Self>) -> Self {
		Self {
			todos: vec![
				Todo::new("Learn Rust".to_string()),
				Todo::new("Learn Yew".to_string())
			]
		}
	}

	fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
		match msg {
			Msg::Add(todo) => {
				self.todos.push(todo);
				true
			},
			Msg::Remove(todo) => {
				if let Some(index) = self.todos.iter().position(|t| t.id == todo.id) {
					self.todos.remove(index);
					return true;
				}
				false
			},
		}
	}

	fn view(&self, ctx: &Context<Self>) -> Html {
		let link = ctx.link();
		html! {
			<div class="todos">
				{
					self.todos.iter().map(|todo| {
						html! {
							<crate::todo::Todo todo={todo.clone()} />
						}
					}).collect::<Html>()
				}
			</div>
		}
	}

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        true
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {}

    fn destroy(&mut self, ctx: &Context<Self>) {}
}
