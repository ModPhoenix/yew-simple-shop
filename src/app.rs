use yew::{html, services, Component, ComponentLink, Html, ShouldRender};

pub enum Msg {
  Increment,
  Decrement,
}

pub struct App {
  link: ComponentLink<Self>,
  value: i64,
}

impl Component for App {
  type Message = Msg;
  type Properties = ();

  fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
    Self { link, value: 0 }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::Increment => {
        self.value += 1;
        services::console::ConsoleService::log("plus one");
        true
      }
      Msg::Decrement => {
        self.value -= 1;
        services::console::ConsoleService::log("minus one");
        true
      }
    }
  }

  fn change(&mut self, _props: Self::Properties) -> ShouldRender {
    false
  }

  fn view(&self) -> Html {
    html! {
        <div>
            <nav class="menu">
                <button onclick=self.link.callback(|_| Msg::Increment)>
                    { "Increment" }
                </button>
                <button onclick=self.link.callback(|_| Msg::Decrement)>
                    { "Decrement" }
                </button>
                <button onclick=self.link.batch_callback(|_| vec![Msg::Increment, Msg::Increment])>
                    { "Increment Twice" }
                </button>
            </nav>
            <p>
                <b>{ "Current value: " }</b>
                { self.value }
            </p>
            <p>
                <b>{ "Rendered at: " }</b>
            </p>
        </div>
    }
  }
}
