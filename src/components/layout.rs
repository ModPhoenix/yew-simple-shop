use yew::prelude::*;

use crate::app::{AppAnchor, AppRoute};

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
      <>
        <nav class="navbar navbar-expand-lg navbar-light bg-light">
          <div class="container">
            <AppAnchor classes="navbar-brand" route=AppRoute::Products>
                { "Simple Shope" }
            </AppAnchor>
            <div class="collapse navbar-collapse">
              <div class="navbar-nav">
                // <a class="nav-link active" aria-current="page" href="#">{"Home"}</a>
                <AppAnchor classes="nav-link" route=AppRoute::Products>
                { "Home" }
                </AppAnchor>

              </div>
            </div>
            <div class="navbar-nav">
              <AppAnchor classes="nav-link" route=AppRoute::Cart>
                { "Cart" }
              </AppAnchor>
            </div>
          </div>
        </nav>
        <div class="container pt-4">
          { self.props.children.clone() }
        </div>
      </>
    }
  }
}
