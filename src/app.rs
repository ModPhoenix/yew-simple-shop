use yew::prelude::*;
use yew_router::{components::RouterAnchor, prelude::*, route::Route, switch::Permissive};

use crate::components::layout::Layout;
use crate::pages::{cart::Cart, home::Home, page_not_found::PageNotFound};

#[derive(Clone, Debug, Switch)]
pub enum AppRoute {
  #[to = "/!"]
  Home,
  #[to = "/cart!"]
  Cart,
  #[to = "/page-not-found"]
  PageNotFound(Permissive<String>),
}

pub enum Msg {}

pub struct App {
  link: ComponentLink<Self>,
}
impl Component for App {
  type Message = Msg;
  type Properties = ();

  fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
    Self { link }
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
      AppRoute::Home => {
        html! { <Home /> }
      }
      AppRoute::Cart => {
        html! { <Cart /> }
      }
      AppRoute::PageNotFound(Permissive(route)) => {
        html! { <PageNotFound route=route /> }
      }
    }
  }
}
