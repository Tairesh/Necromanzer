use tetra::graphics::Color;

pub enum ActionResult {
    LogMessage(String),
    ColoredLogMessage(String, Color),
    // TODO: draw stamina
}
