#[derive(Debug)]
#[derive(Clone)]
pub struct Window {
  pub value: String,
  pub root: Option<String>
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Panes {
  pub layout: String,
  pub window: String,
  pub exec: Vec<String>,
  pub root: Option<String>
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Root {
  pub value: String,
  pub window: String
}

#[derive(Debug)]
#[derive(Clone)]
pub enum Command {
    Window(Window),
    Panes(Panes),
    Root(Root)
}