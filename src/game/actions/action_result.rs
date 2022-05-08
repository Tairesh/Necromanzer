use tetra::graphics::Color;

// TODO: mutate world instead
#[non_exhaustive]
pub enum ActionResult {
    LogMessage(String),
    ColoredLogMessage(String, Color),
    CancelAction(String),
    // TODO: draw stamina
}
