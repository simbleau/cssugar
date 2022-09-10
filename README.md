# CSSugar
A CSS values and units library for Rust, focusing on ergnomic abstractions.

# Goal
The goal is to wrap all CSS values and units in an ergonomic, rusty way. This is primarily for ecosystem tooling built around Yew and Stylist.

Read more:
- https://developer.mozilla.org/en-US/docs/Learn/CSS/Building_blocks/Values_and_units

# Examples

## Numbers, lengths, and percentages
[Read more](https://developer.mozilla.org/en-US/docs/Learn/CSS/Building_blocks/Values_and_units#numbers_lengths_and_percentages)
```rs
use cssugar::units::Em;

#[function_component(SizedButton)]
pub fn sized_button() -> Html {
    let size = Vw(100) - Px(300);
    let button_css = css!("width: ${size};");
    html! {
        <button class={button_css}>{ "CLICK ME!" }</button>
    }
}
```

Expectated Output:
> `<button style="width: calc(100vw - 300px);">CLICK ME!</button>`

## Colors
[Read more](https://developer.mozilla.org/en-US/docs/Learn/CSS/Building_blocks/Values_and_units#color)
```rs
use cssugar::colors::{Color, BLACK};

#[function_component(SizedButton)]
pub fn sized_button() -> Html {
    let color = Color::rgb(255, 0, 0).blend(BLACK);
    let label_css = css!("color: ${color};");
    html! {
        <label class={label_css}>{ "I am dark red!" }</label>
    }
}
```

## Images
[Read more](https://developer.mozilla.org/en-US/docs/Learn/CSS/Building_blocks/Values_and_units#images)
TODO

## Strings and Identifiers
[Read more](https://developer.mozilla.org/en-US/docs/Learn/CSS/Building_blocks/Values_and_units#strings_and_identifiers)
TODO


## Functions
[Read more](https://developer.mozilla.org/en-US/docs/Learn/CSS/Building_blocks/Values_and_units#functions)
TODO