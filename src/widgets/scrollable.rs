use crate::{
    block::BoxModel,
    color::UiColor,
    el::{El, ElId},
    event::Event,
    layout::{Layout, Limits},
    palette::PaletteColor,
    render::Renderer,
    size::{Length, Size},
    style::component_style,
    theme::Theme,
    widget::Widget,
};

pub struct ScrollableStatus;

component_style! {
    pub ScrollableStyle: ScrollableStyler(ScrollableStatus) default {default} {
        background: background,
    }
}

pub fn default<C: PaletteColor>(theme: &Theme<C>, _status: ScrollableStatus) -> ScrollableStyle<C> {
    ScrollableStyle::new(&theme.palette())
}

#[derive(Clone, Copy)]
pub enum ScrollDir {
    None,
    Horizontal,
    Vertical,
    Both,
}

impl ScrollDir {
    pub fn limits(&self, limits: &Limits) -> Limits {
        let (new_width, new_height) = match self {
            ScrollDir::None => (None, None),
            ScrollDir::Horizontal => (Some(u32::MAX), None),
            ScrollDir::Vertical => (None, Some(u32::MAX)),
            ScrollDir::Both => (Some(u32::MAX), Some(u32::MAX)),
        };

        let new_max = limits.max();
        let new_max =
            if let Some(new_width) = new_width { new_max.new_width(new_width) } else { new_max };
        let new_max = if let Some(new_height) = new_height {
            new_max.new_height(new_height)
        } else {
            new_max
        };

        limits.with_max(new_max)
    }
}

pub struct Scrollable<'a, Message, C, E, S>
where
    C: UiColor,
    E: Event,
    S: ScrollableStyler<C>,
{
    id: ElId,
    content: El<'a, Message, C, E, S>,
    size: Size<Length>,
    dir: ScrollDir,
    class: S::Class<'a>,
}

impl<'a, Message, C, E, S> Scrollable<'a, Message, C, E, S>
where
    C: UiColor,
    E: Event,
    S: ScrollableStyler<C>,
{
    pub fn new(content: El<'a, Message, C, E, S>) -> Self {
        Self {
            id: ElId::unique(),
            content,
            size: Size::fill(),
            dir: ScrollDir::Vertical,
            class: S::default(),
        }
    }
}

impl<'a, Message, C, E, S> Widget<Message, C, E, S> for Scrollable<'a, Message, C, E, S>
where
    C: UiColor,
    E: Event,
    S: ScrollableStyler<C>,
{
    fn id(&self) -> Option<ElId> {
        Some(self.id)
    }

    fn tree_ids(&self) -> alloc::vec::Vec<ElId> {
        self.content.tree_ids()
    }

    fn size(&self, _viewport: &crate::layout::Viewport) -> Size<Length> {
        self.size
    }

    fn layout(
        &self,
        ctx: &mut crate::ui::UiCtx<Message>,
        state: &mut crate::state::StateNode,
        styler: &S,
        limits: &crate::layout::Limits,
        viewport: &crate::layout::Viewport,
    ) -> crate::layout::LayoutNode {
        Layout::container(
            limits,
            self.size,
            crate::layout::Position::Relative,
            viewport,
            BoxModel::new(),
            crate::align::Align::Start,
            crate::align::Align::Start,
            |limits| {
                self.content.layout(
                    ctx,
                    &mut state.children[0],
                    styler,
                    &self.dir.limits(limits),
                    viewport,
                )
            },
        )
    }

    fn draw(
        &self,
        ctx: &mut crate::ui::UiCtx<Message>,
        state: &mut crate::state::StateNode,
        renderer: &mut Renderer<C>,
        styler: &S,
        layout: crate::layout::Layout,
        viewport: &crate::layout::Viewport,
    ) {
        let bounds = layout.bounds();

        // let renderer = renderer.clipped(bounds);

        self.content.draw(
            ctx,
            &mut state.children[0],
            renderer,
            styler,
            layout.children().next().unwrap(),
            viewport,
        );
    }
}
