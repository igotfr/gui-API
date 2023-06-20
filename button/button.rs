// https://docs.rs/iced/latest/iced/
// like iced https://docs.rs/iced_native/latest/iced_native/widget/button/struct.Button.html
button<'a, Message, Renderer>(content: impl Into<Element<'a, Message, Renderer>>) -> widget::Button<'a, Message, Renderer>
where
    Renderer: crate::Renderer,
    Renderer::Theme: widget::button::StyleSheet,
    <Renderer::Theme as widget::button::StyleSheet>::Style: Default,

  .width(width: impl Into<Length>)
  .height(height: impl Into<Length>)
  .padding(padding: impl Into<Padding>)
  .style(style: <Renderer::Theme as StyleSheet>::Style)

  .on_press(msg: Message) // Observer Pattern

  // core Rust
  .into()
  .try_into()
