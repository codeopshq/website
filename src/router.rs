use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/tutorials")]
    Tutorials,
    #[at("/about")]
    About,
    #[at("/contact")]
    Contact,
    #[not_found]
    #[at("/404")]
    NotFound,
}
