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
use cssugar::prelude::*;

#[function_component(SizedButton)]
pub fn sized_button() -> Html {
    let size = Vw(100) - Px(300);
    let button_css = format!("width: ${size};");
    html! {
        <button style={button_css}>{ "CLICK ME!" }</button>
    }
}
```

Expected Output:
> `<button style="width: calc(100vw - 300px);">CLICK ME!</button>`

## Colors
[Read more](https://developer.mozilla.org/en-US/docs/Learn/CSS/Building_blocks/Values_and_units#color)
```rs
use cssugar::prelude::*;

#[function_component(SizedButton)]
pub fn sized_button() -> Html {
    let color = Color::rgb(255, 0, 0).blend(BLACK);
    let label_css = format!("color: ${color};");
    html! {
        <label style={label_css}>{ "I am dark red!" }</label>
    }
}
```
Expected Output:
> `<label style="color: rgba(128, 0, 0, 1.0);">I am dark red!</label>`

## Images
[Read more](https://developer.mozilla.org/en-US/docs/Learn/CSS/Building_blocks/Values_and_units#images)
TODO

## Functions
[Read more](https://developer.mozilla.org/en-US/docs/Learn/CSS/Building_blocks/Values_and_units#functions)
```rs
use cssugar::prelude::*;

#[function_component(SizedButton)]
pub fn sized_button() -> Html {
    let l1 = Length::Vw(100.);
    let l2 = Length::Px(300.);
    let size = l1.min(l2);
    let button_css = format!("width: ${size};");
    html! {
        <button style={button_css}>{ "CLICK ME!" }</button>
    }
}
```
Expected Output:
> `<button style="width: min(100vw, 300px);">CLICK ME!</button>`