#[derive(Clone)]
pub enum PopupEvent {
    ShowPopup,
    HidePopup,
    Action(PopupAction),
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum PopupAction {
    Confirm,
    Cancel,
}


