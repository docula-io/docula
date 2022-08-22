#[derive(PartialEq, Eq, Debug, Clone)]
pub struct BlockToken {
    pub line_start: usize,
    pub token: Block
}

pub type Document = Vec<BlockToken>;

pub type Text = Vec<InlineToken>;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct InlineToken {
    pub line_start: usize,
    pub position: usize,
    pub token: Inline
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Block {
    Heading {
        level: usize,
        content: Text,
        style: HeadingStyle
    },

    BlockQuote(Document),

    BlockCode {
        tag: Option<String>,
        content: String,
    },

    OrderedList {
        start_index: usize,
        items: Vec<Document>,
    },

    UnorderedList {
        items: Vec<Document>,
    },

    Paragraph(Text),

    HorizontalRule,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Inline {
    LineBreak,

    Chunk(String),

    Emphasis(Text),

    MoreEmphasis(Text),

    Code(String),

    Link {
        text: Option<Text>,  // None for automatic links
        link: Option<String>,
        title: Option<String>,
        id: Option<String>
    },

    Image {
        alt: Text,
        link: Option<String>,
        title: Option<String>,
        id: Option<String>
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum HeadingStyle {
    Atx,
    Setex,
}
