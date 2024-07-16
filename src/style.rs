use crate::{
    color::UiColor,
    kit::{button::ButtonStyler, select::SelectStyler, slider::SliderStyler},
};

pub trait Styler<C: UiColor>:
    ButtonStyler<C> + SelectStyler<C> + SliderStyler<C> + Default
{
    fn background(&self) -> C;
}

/**
 *
 */
macro_rules! component_style {
    ($(#[$meta:meta])? $vis: vis $name: ident $(: $styler: ident ($status: ty) default {$default: expr})? {
        $($prop: ident: $prop_kind: ident $({
            $($method: ident: $method_kind: ident),* $(,)?
        })?),* $(,)?
    }) => {
        $(
            $vis trait $styler<C: $crate::color::UiColor> {
                type Class<'a>;

                fn default<'a>() -> Self::Class<'a>;
                fn style(&self, class: &Self::Class<'_>, status: $status) -> $name<C>;
            }

            impl<C: crate::palette::PaletteColor + 'static> $styler<C> for crate::theme::Theme<C> {
                type Class<'a> = alloc::boxed::Box<dyn Fn(&crate::theme::Theme<C>, $status) -> $name<C> + 'a>;

                fn default<'a>() -> Self::Class<'a> {
                    alloc::boxed::Box::new($default)
                }

                fn style(&self, class: &Self::Class<'_>, status: $status) -> $name<C> {
                    class(self, status)
                }
            }
        )?

        $vis struct $name<C: $crate::color::UiColor> {
            $($prop: $crate::style::component_style!(@field $prop_kind)),*
        }

        impl<C: $crate::color::UiColor> $name<C> {
            pub fn new(palette: &$crate::palette::Palette<C>) -> Self {
                Self {
                    $($prop: $crate::style::component_style!(@init $prop_kind palette)),*
                }
            }

            $($crate::style::component_style!{ @build $prop: $prop_kind $({ $($method: $method_kind),* })? })*
        }
    };

    // Fields //
    (@field background) => {
        C
    };

    (@field color) => {
        C
    };

    (@field border) => {
        $crate::block::Border<C>
    };

    // FIXME: Width is not the right word
    (@field width) => {
        u32
    };

    // Constructor //
    (@init background $palette: ident) => {
        $palette.background
    };

    (@init border $palette: ident) => {
        $crate::block::Border::new()
    };

    (@init color $palette: ident) => {
        $palette.foreground
    };

    (@init width $palette: ident) => {
        // TODO: Defaults
        1
    };

    // Builders //
    (@build $name: ident: background) => {
        pub fn $name(mut self, background: impl Into<C>) -> Self {
            self.$name = background.into();
            self
        }
    };

    (@build_method $field: ident . border $method: ident: border_color) => {
        pub fn $method(mut self, color: impl Into<C>) -> Self {
            self.$field.color = color.into();
            self
        }
    };

    (@build_method $field: ident . border $method: ident: border_width) => {
        pub fn $method(mut self, width: u32) -> Self {
            self.$field.width = width;
            self
        }
    };

    (@build_method $field: ident . border $method: ident: border_radius) => {
        pub fn $method(mut self, radius: impl Into<$crate::block::BorderRadius>) -> Self {
            self.$field.radius = radius.into();
            self
        }
    };

    (@build $field: ident: border) => {
        $crate::style::component_style! {@build $field: border {
            border_color: border_color,
            border_width: border_width,
            border_radius: border_radius
        }}
    };

    (@build $field: ident: border {$($method: ident: $method_kind: ident),*}) => {
        $($crate::style::component_style! {
            @build_method $field.border $method: $method_kind
        })*
    };

    (@build $name: ident: color) => {
        pub fn $name(mut self, color: impl Into<C>) -> Self {
            self.$name = color.into();
            self
        }
    };

    (@build $name: ident: width) => {
        pub fn $name(mut self, width: u32) -> Self {
            self.$name = width;
            self
        }
    };
}

pub(crate) use component_style;

// component_style!(Test: TestStyler(()) { background, border });
