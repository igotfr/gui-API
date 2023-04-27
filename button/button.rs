// like iced https://docs.rs/iced/latest/iced/
button(content: impl Into<Element<'a, Message, Renderer>>) -> widget::Button<'a, Message, Renderer>
  .width(width: impl Into<Length>)
  .height(height: impl Into<Length>)
  .padding(padding: impl Into<Padding>)
  .style()

  .on_press() // Observer Pattern

  .into()
  .try_into()
