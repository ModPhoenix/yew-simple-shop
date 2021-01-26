use crate::components::product_item::ProductItem;
use crate::entities::Product;
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

#[derive(Debug)]
pub enum Msg {
  GetProduct,
  ReceiveResponse(Result<Product, anyhow::Error>),
}

#[derive(Properties, Clone, Debug)]
pub struct Props {
  pub id: String,
}

#[derive(Debug)]
pub struct ProductPage {
  props: Props,
  fetch: Option<FetchTask>,
  product: Option<Product>,
  link: ComponentLink<Self>,
  error: Option<String>,
}

impl Component for ProductPage {
  type Message = Msg;
  type Properties = Props;

  fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
    link.send_message(Msg::GetProduct);

    Self {
      props,
      fetch: None,
      product: None,
      link,
      error: None,
    }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    use Msg::*;
    match msg {
      GetProduct => {
        let request = Request::get(format!(
          "https://yalantis-react-school-api.yalantis.com/api/v1/products/{}",
          self.props.id
        ))
        .body(Nothing)
        .expect("Could not build request.");
        let callback =
          self
            .link
            .callback(|response: Response<Json<Result<Product, anyhow::Error>>>| {
              let Json(data) = response.into_body();
              Msg::ReceiveResponse(data)
            });
        let task = FetchService::fetch(request, callback).expect("failed to start request");
        self.fetch = Some(task);
        true
      }
      ReceiveResponse(response) => {
        match response {
          Ok(product) => {
            self.product = Some(product);
          }
          Err(error) => self.error = Some(error.to_string()),
        }
        self.fetch = None;
        true
      }
    }
  }

  fn change(&mut self, _props: Self::Properties) -> ShouldRender {
    false
  }

  fn view(&self) -> Html {
    html! {
      match self.fetch {
        Some(_) => {
          html! {
            <div class="d-flex justify-content-center">
              <div class="spinner-grow" role="status">
                <span class="visually-hidden">{"Loading..."}</span>
              </div>
            </div>
          }
        }
        None => match self.product {
          Some(ref product) => {
            html! {
              <div class="row row-cols-1 row-cols-md-2 g-2">
                <div class="col">
                  <ProductItem product={product} />
                </div>
              </div>

            }
          }
          None => {
            if let Some(ref error) = self.error {
              html! { <p>{ error.clone() }</p> }
            } else {
              html! {}
            }
          }
        },
      }
    }
  }
}
