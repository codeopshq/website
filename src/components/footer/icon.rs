use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub url: AttrValue,
    pub svg: AttrValue,
}

#[function_component(Icon)]
pub fn icon(props: &Props) -> Html {
    let svg_content = match props.svg.as_str() {
        "youtube" => include_str!("../../../assets/svg//youtube.svg"),
        "github" => include_str!("../../../assets/svg/github.svg"),
        "twitter" => include_str!("../../../assets/svg/twitter.svg"),
        "discord" => include_str!("../../../assets/svg/discord.svg"),
        "facebook" => include_str!("../../../assets/svg/facebook.svg"),
        "tiktok" => include_str!("../../../assets/svg/tiktok.svg"),
        "instagram" => include_str!("../../../assets/svg/instagram.svg"),
        "linkedin" => include_str!("../../../assets/svg/linkedin.svg"),
        // Add more cases as needed
        _ => "",
    };

    //let svg_content = include_str!(format!("../../../{}", props.svg.as_str()));

    html! {
        <a href={props.url.clone()}>
            <i class="w-6 inline-block mx-2">
                { Html::from_html_unchecked(svg_content.into()) }
            </i>
        </a>
    }
}
