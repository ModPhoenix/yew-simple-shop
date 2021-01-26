use crate::entities::Product;
use yew::prelude::*;

use crate::app::{AppAnchor, AppRoute};

pub struct ProductItem {
  props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
  pub product: Product,
}

impl Component for ProductItem {
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
      <div class="card">
        <div class="card-body">
          <h5 class="card-title">{self.props.product.name.to_string()}</h5>
          <div class="card-subtitle mb-2 text-muted">{format!("{}$", self.props.product.price)}</div>
          <p class="card-text">{format!("Origin: {}", self.props.product.origin.to_uppercase())}</p>
          <button type="button" class="btn btn-outline-primary">{ "Buy" }</button>
        </div>
      </div>
    }
  }
}
