use crate::builder::StyledStr;
use crate::util::color::ColorChoice;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum Stream {
    Stdout,
    Stderr,
}

#[derive(Clone, Debug)]
pub(crate) struct Colorizer {
    stream: Stream,
    #[allow(unused)]
    color_when: ColorChoice,
    content: StyledStr,
}

impl Colorizer {
    pub(crate) fn new(stream: Stream, color_when: ColorChoice) -> Self {
        Colorizer {
            stream,
            color_when,
            content: Default::default(),
        }
    }

    pub(crate) fn with_content(mut self, content: StyledStr) -> Self {
        self.content = content;
        self
    }
}

/// Color-unaware printing. Never uses coloring.
impl std::fmt::Display for Colorizer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.content.fmt(f)
    }
}
