// like iced https://docs.rs/iced_native/0.10.1/iced_native/widget/text/struct.Text.html
// https://docs.rs/iced/latest/iced/
text(content: impl ToString) -> widget::Text<'a, Renderer>
  .width(width: impl Into<Length>)
  .height(height: impl Into<Length>)
  .size(size: impl Into<Pixels>)
  .font(font: impl Into<Renderer::Font>)
  .horizontal_alignment(alignment: alignment::Horizontal)
  .vertical_alignment(alignment: alignment::Vertical)
  .style(style: impl Into<<Renderer::Theme as StyleSheet>::Style>)

  // core Rust
  .into()
  .try_into()
  .to_owned()
  .clone()
  .clone_into()
  .clone_from()
