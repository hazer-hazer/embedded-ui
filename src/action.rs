use crate::el::ElId;

pub enum WidgetAction {
    Focus(ElId),
}

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
    fn from(value: ()) -> Self {
        Self::None
    }
}
