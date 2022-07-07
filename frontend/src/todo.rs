use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct TodoProps {
	pub todo: models::Todo,
}

#[function_component(Todo)]
pub fn todo(props: &TodoProps) -> Html {

	html!{
		<div>
			<span>{props.todo.title.clone()}</span>
			<button>{"Remove"}</button>
		</div>
	}
}