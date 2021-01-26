use yew::prelude::*;
use yew_router::{components::RouterAnchor, prelude::*, route::Route, switch::Permissive};

use crate::components::layout::Layout;
use crate::pages::{
  cart::Cart, page_not_found::PageNotFound, product::ProductPage, products::ProductsPage,
};

#[derive(Clone, Debug, Switch)]
pub enum AppRoute {
  #[to = "/cart!"]
  Cart,
  #[to = "/!"]
  Products,
  #[to = "/products/{id}"]
  Product(String),
  #[to = "/page-not-found"]
  PageNotFound(Permissive<String>),
}

pub type AppAnchor = RouterAnchor<AppRoute>;

pub enum Msg {}

pub struct App {}

impl Component for App {
  type Message = Msg;
  type Properties = ();

  fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
    Self {}
  }

  fn update(&mut self, _msg: Self::Message) -> ShouldRender {
    false
  }

  fn change(&mut self, _props: Self::Properties) -> ShouldRender {
    false
  }

  fn view(&self) -> Html {
    html! {
      <>
        <Layout>
          <Router<AppRoute, ()>
            render = Router::render(Self::switch)
            redirect = Router::redirect(|route: Route<()>| {
                AppRoute::PageNotFound(Permissive(Some(route.route)))
            })
          />
        </Layout>
      </>
    }
  }
}

impl App {
  fn switch(switch: AppRoute) -> Html {
    match switch {
      AppRoute::Cart => {
        html! { <Cart /> }
      }
      AppRoute::Products => {
        html! { <ProductsPage /> }
      }
      AppRoute::Product(id) => {
        html! { <ProductPage id={id} /> }
      }
      AppRoute::PageNotFound(Permissive(route)) => {
        html! { <PageNotFound route=route /> }
      }
    }
  }
}
