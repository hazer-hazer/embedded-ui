// Idea //

use embedded_graphics::pixelcolor::BinaryColor;

fn foo() {
    let styles = StyleSheet::new();

    Button::new().style(styles).background(BinaryColor::Off).border_width();
}

struct Styles {
    // All styles...
}

pub struct StyleSheet {
    ids: BTreeMap<ElId, Styles>,
    groups: BTreeMap<GroupId, Styles>,
}

impl Button {
    fn style(&mut self, styles: &mut StyleSheet) -> ButtonStyleBuilder {
        // ...
    }
}

struct ButtonStyleBuilder<'a, C: UiColor> {
    styles: &mut StyleSheet,

    button: Button,

    // Styles
    background: C,
    border_width: u32,
    // ...
}

impl<'a, C: UiColor> ButtonStyleBuilder<'a, C> {
    fn new(styles: &mut StyleSheet<'a>) -> Self {
        Self {styles}
    }

    fn background(mut self, background: C) -> Self {
        self.background = background;
        self
    }

    fn complete(mut self) -> Button {
        self.styles.ids[self.button.id] = ButtonStyles {
            background: self.background,
            // ...
        };
        self.button
    }

    // ...
}
