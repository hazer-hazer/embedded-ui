use alloc::vec::Vec;
use embedded_graphics::{
    geometry::Point,
    primitives::{PrimitiveStyle, Rectangle, StyledDrawable},
};

use crate::{
    align::{Alignment, Axis},
    el::{El, ElId},
    event::Event,
    icons::IconKind,
    layout::{Layout, LayoutNode, Limits},
    padding::Padding,
    render::Renderer,
    size::{Length, Size},
    state::{State, StateNode, StateTag},
    widget::Widget,
};

use super::icon::{Icon, IconPicker};

pub struct Select<'a, Message, R, E, S>
where
    R: Renderer,
    E: Event,
{
    id: ElId,
    size: Size<Length>,
    icon_left: IconKind,
    icon_right: IconKind,
    options: Vec<El<'a, Message, R, E, S>>,
    chosen: usize,
}

impl<'a, Message, R, E, S> Select<'a, Message, R, E, S>
where
    R: Renderer,
    E: Event,
{
    pub fn new(options: Vec<El<'a, Message, R, E, S>>) -> Self {
        Self {
            id: ElId::unique(),
            size: Size::fill(),
            icon_left: IconKind::ArrowLeft,
            icon_right: IconKind::ArrowRight,
            options,
            chosen: 0,
        }
    }

    pub fn initial(mut self, index: impl Into<usize>) -> Self {
        self.chosen = index.into();
        self
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.size.width = width.into();
        self
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.size.height = height.into();
        self
    }

    // Helpers //
    fn current(&self) -> &El<'a, Message, R, E, S> {
        &self.options[self.chosen]
    }

    fn arrow_icon_size(&self, limits: &Limits) -> u32 {
        // limits.max().height
        // TODO
        5
    }
}

impl<'a, Message, R, E, S> Widget<Message, R, E, S> for Select<'a, Message, R, E, S>
where
    R: Renderer,
    E: Event,
{
    fn id(&self) -> Option<crate::el::ElId> {
        Some(self.id)
    }

    fn tree_ids(&self) -> Vec<crate::el::ElId> {
        // TODO: Maybe Select should hide ids of its children or we might fail on focusing them
        self.options.iter().map(|option| option.tree_ids()).flatten().collect()
    }

    fn size(&self) -> Size<Length> {
        self.size
    }

    fn state_tag(&self) -> crate::state::StateTag {
        StateTag::stateless()
    }

    fn state(&self) -> State {
        State::None
    }

    fn state_children(&self) -> Vec<StateNode> {
        vec![StateNode::new(self.current())]
    }

    fn layout(
        &self,
        ctx: &mut crate::ui::UiCtx<Message>,
        state: &mut crate::state::StateNode,
        styler: &S,
        limits: &crate::layout::Limits,
    ) -> crate::layout::LayoutNode {
        // Layout::container(limits, self.size.width, self.size.height, |limits| {
        //     // TODO: Use real icons layouts to be accurate?

        //     // Reserve some space for arrows on the sides
        //     let shrink_by_arrows = limits.max_square() * 2;
        //     self.options[self.chosen].layout(ctx, state, styler, &limits.shrink(shrink_by_arrows))
        // })

        // TODO: Smarter icon size
        let padding_for_icons = self.arrow_icon_size(limits);

        Layout::padded(
            limits,
            self.size.width,
            self.size.height,
            Padding::new_axis(0, padding_for_icons),
            Padding::zero(),
            |limits| self.options[self.chosen].layout(ctx, &mut state.children[0], styler, limits),
        )
    }

    fn draw(
        &self,
        ctx: &mut crate::ui::UiCtx<Message>,
        state: &mut crate::state::StateNode,
        renderer: &mut R,
        styler: &S,
        layout: crate::layout::Layout,
    ) {
        let bounds = layout.bounds();
        let icons_limits = Limits::new(Size::zero(), Size::new_equal(bounds.size.height));
        let icons_node = LayoutNode::new(self.arrow_icon_size(&icons_limits).into());

        Widget::<Message, R, E, S>::draw(
            &Icon::new(self.icon_left),
            ctx,
            &mut StateNode::stateless(),
            renderer,
            styler,
            Layout::with_offset(bounds.position, &icons_node),
        );

        self.current().draw(
            ctx,
            &mut state.children[0],
            renderer,
            styler,
            layout.children().next().unwrap(),
        );

        Widget::<Message, R, E, S>::draw(
            &Icon::new(self.icon_right),
            ctx,
            &mut StateNode::stateless(),
            renderer,
            styler,
            Layout::with_offset(
                bounds.position
                    + Point::new(bounds.size.width as i32 - icons_node.size().width as i32, 0),
                &icons_node,
            ),
        );
    }
}

impl<'a, Message, R, E, S> From<Select<'a, Message, R, E, S>> for El<'a, Message, R, E, S>
where
    Message: Clone + 'a,
    R: Renderer + 'a,
    E: Event + 'a,
    S: 'a,
{
    fn from(value: Select<'a, Message, R, E, S>) -> Self {
        El::new(value)
    }
}
