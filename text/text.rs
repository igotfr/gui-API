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
  .font_style(font_style: FontStyle)
  .font_weight(font_weight: impl Into<FontWeight>)
  .font_family(font_family: FontFamily)

  // core Rust
  .into()
  .try_into()
  .to_owned()
  .clone()
  .clone_into()
  .clone_from()

tuple FontStyle(u32);

impl FontStyle {
  const Normal: FontStyle = FontStyle(0);
  const Italic: FontStyle = FontStyle(1);
}

tuple FontWeight(u32);

impl FontWeight {
  const W100: FontWeight = FontWeight(100);
  const W200: FontWeight = FontWeight(200);
  const W300: FontWeight = FontWeight(300);
  const W400: FontWeight = FontWeight(400);
  const W500: FontWeight = FontWeight(500);
  const W600: FontWeight = FontWeight(600);
  const W700: FontWeight = FontWeight(700);
  const W800: FontWeight = FontWeight(800);
  const W900: FontWeight = FontWeight(900);
  const Thin: FontWeight = FontWeight(100);
  const ExtraLight: FontWeight = FontWeight(200);
  const Light: FontWeight = FontWeight(300);
  const Normal: FontWeight = FontWeight(400);
  const Medium: FontWeight = FontWeight(500);
  const SemiBold: FontWeight = FontWeight(600);
  const Bold: FontWeight = FontWeight(700);
  const ExtraBold: FontWeight = FontWeight(800);
  const Black: FontWeight = FontWeight(900);
}
