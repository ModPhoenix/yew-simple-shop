use yew::prelude::*;

pub struct Layout {
  props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
  pub children: Children,
}

impl Component for Layout {
  type Message = ();
  type Properties = Props;

  fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
    Self { props }
  }

  fn update(&mut self, _msg: Self::Message) -> ShouldRender {
    unimplemented!()
  }

  fn change(&mut self, _props: Self::Properties) -> ShouldRender {
    false
  }

  fn view(&self) -> Html {
    html! {
        <div>
            { "Layout"}
            { self.props.children.clone() }
        </div>
    }
  }
}
