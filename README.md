# CSSugar
A CSS library for Rust, focusing on ergnomic abstractions.

# Goal
The goal is to wrap all CSS values and units in an ergonomic, rusty way.
Read more:
- https://developer.mozilla.org/en-US/docs/Learn/CSS/Building_blocks/Values_and_units

This is primarily for ecosystem tooling built around Yew and Stylist.

# Examples

## Numbers, lengths, and percentages
**Read more**: https://developer.mozilla.org/en-US/docs/Learn/CSS/Building_blocks/Values_and_units#numbers_lengths_and_percentages
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

## Colors
**Read more**: https://developer.mozilla.org/en-US/docs/Learn/CSS/Building_blocks/Values_and_units#color
TODO

## Images
**Read more**: https://developer.mozilla.org/en-US/docs/Learn/CSS/Building_blocks/Values_and_units#images
TODO

## Strings and Identifiers
**Read more**: https://developer.mozilla.org/en-US/docs/Learn/CSS/Building_blocks/Values_and_units#strings_and_identifiers
TODO


## Functions
**Read more**: https://developer.mozilla.org/en-US/docs/Learn/CSS/Building_blocks/Values_and_units#functions
TODO