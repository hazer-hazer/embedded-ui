use crate::el::ElId;

#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub enum WidgetAction {
    Focus(ElId),
}

#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub enum Action {
    None,
    Widget(WidgetAction),
}

impl Action {
    pub fn widget(action: WidgetAction) -> Self {
        Self::Widget(action)
    }

    pub fn focus(id: ElId) -> Self {
        Self::widget(WidgetAction::Focus(id))
    }
}

impl From<()> for Action {
    fn from(_value: ()) -> Self {
        Self::None
    }
}
