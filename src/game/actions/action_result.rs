use tetra::graphics::Color;

#[non_exhaustive]
pub enum ActionResult {
    LogMessage(String),
    ColoredLogMessage(String, Color),
    CancelAction(String),
    // TODO: draw stamina
}
