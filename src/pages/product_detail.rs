use crate::api;
use crate::types::Product;
use anyhow::Error;
use yew::format::Json;
use yew::prelude::*;
use yew::services::fetch::FetchTask;

pub struct State {
    product: Option<Product>,
    get_products_error: Option<Error>,
    get_products_loaded: bool,
}

pub struct ProductDetail {
    props: Props,
    state: State,
    link: ComponentLink<Self>,
    task: Option<FetchTask>,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub id: i32,
}

pub enum Msg {
    GetProduct,
    GetProductSuccess(Product),
    GetProductError(Error),
}

impl Component for ProductDetail {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::GetProduct);

        Self {
            props,
            state: State {
                product: None,
                get_product_error: None,
                get_products_loaded: false,
            },
            link,
            task: None,
        }
    }
}

