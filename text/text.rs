// https://docs.rs/iced/latest/iced/
// like iced https://docs.rs/iced_native/latest/iced_native/widget/text/struct.Text.html
fn text<'a, Renderer>(text: impl ToString) -> widget::Text<'a, Renderer>
where
    Renderer: crate::text::Renderer,
    Renderer::Theme: widget::text::StyleSheet,

  .width(width: impl Into<Length>)
  .height(height: impl Into<Length>)
  .size(size: impl Into<Pixels>)
  .font(font: impl Into<Renderer::Font>)
  .horizontal_alignment(alignment: alignment::Horizontal)
  .vertical_alignment(alignment: alignment::Vertical)
  .style(style: impl Into<<Renderer::Theme as StyleSheet>::Style>)

  // jetpack terminology
  .font_size(font_size: impl Into<Pixels>)
  .font_style(font_style: impl Into<FontStyle>)
  .font_weight(font_weight: impl Into<FontWeight>)
  .font_family(font_family: FontFamily)

  // core Rust
  .into()
  .try_into()
  .to_owned()
  .clone()
  .clone_into()
  .clone_from()
