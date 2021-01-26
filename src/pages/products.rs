use crate::components::product_item::ProductItem;
use crate::entities::ProductsList;
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

#[derive(Debug)]
pub enum Msg {
  GetProducts,
  ReceiveResponse(Result<ProductsList, anyhow::Error>),
}

#[derive(Debug)]
pub struct ProductsPage {
  fetch: Option<FetchTask>,
  products: Option<ProductsList>,
  link: ComponentLink<Self>,
  error: Option<String>,
}

impl Component for ProductsPage {
  type Message = Msg;
  type Properties = ();

  fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
    link.send_message(Msg::GetProducts);

    Self {
      fetch: None,
      products: None,
      link,
      error: None,
    }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    use Msg::*;
    match msg {
      GetProducts => {
        // 1. build the request
        let request =
          Request::get("https://yalantis-react-school-api.yalantis.com/api/v1/products")
            .body(Nothing)
            .expect("Could not build request.");
        // 2. construct a callback
        let callback = self.link.callback(
          |response: Response<Json<Result<ProductsList, anyhow::Error>>>| {
            let Json(data) = response.into_body();
            Msg::ReceiveResponse(data)
          },
        );
        // 3. pass the request and callback to the fetch service
        let task = FetchService::fetch(request, callback).expect("failed to start request");
        // 4. store the task so it isn't canceled immediately
        self.fetch = Some(task);
        // we want to redraw so that the page displays a 'fetching...' message to the user
        // so return 'true'
        true
      }
      ReceiveResponse(response) => {
        match response {
          Ok(products) => {
            self.products = Some(products);
          }
          Err(error) => self.error = Some(error.to_string()),
        }
        self.fetch = None;
        // we want to redraw so that the page displays the location of the ISS instead of
        // 'fetching...'
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
        None => match self.products {
          Some(ref products) => {
            html! {
              <div class="row row-cols-1 row-cols-md-2 g-2">
                { for products.items.iter().map(|product| html! {
                  <div class="col">
                    <ProductItem product={product} />
                  </div>
                }) }
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
