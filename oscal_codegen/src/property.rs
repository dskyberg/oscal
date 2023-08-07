pub trait Property {
    fn title(&self) -> Option<String>;
    fn description(&self) -> Option<String>;
    fn reference(&self) -> Option<String>;
    fn name(&self) -> String;
    fn ref_name(&self) -> Option<String>;
}
