# CSSugar
A CSS library for Rust, focusing on ergnomic abstractions.

# Goal
The goal is to wrap all CSS data types (units, functions, values) in an ergonomic, rusty way.
Read more: [https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Types](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Types)

This is primarily for ecosystem tooling built around Yew and Stylist.

# Example

## Colors
TODO

## Functions
TODO

## Values
TODO

## Units
```rs
use cssugar::units::Em;

#[derive(Properties, PartialEq)]
pub struct Props { scale: f32 }

#[function_component(SizedButton)]
pub fn sized_button(props: &Props) -> Html {
    let font_size = Em(1.0) * &props.scale;
    let button_css = css!("text-size: ${font_size}");
    html! {
        <button class={style}>{ "CLICK ME!" }</button>
    }
}
```

Expectated Output:
> `<button style="text-size: calc!(1.0em * 2.7)">CLICK ME!</button>`