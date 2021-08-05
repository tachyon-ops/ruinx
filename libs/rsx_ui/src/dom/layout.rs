use std::rc::Rc;

use super::node::StyledNode;

#[derive(Clone, Copy, Debug, Default)]
pub struct Dimensions {
    pub content: Rect,
    pub padding: EdgeSizes,
    pub border: EdgeSizes,
    pub margin: EdgeSizes,
}

impl Dimensions {
    fn padding_box(self) -> Rect {
        self.content.expanded_by(self.padding)
    }
    fn border_box(self) -> Rect {
        self.padding_box().expanded_by(self.border)
    }
    fn margin_box(self) -> Rect {
        self.border_box().expanded_by(self.margin)
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    pub fn with_inset(self, val: f32) -> Rect {
        Rect {
            x: (self.x + val).floor() + 0.5,
            y: (self.y + val).floor() + 0.5,
            width: (self.width - val - val).floor(),
            height: (self.height - val - val).floor(),
        }
    }
    fn expanded_by(self, edge: EdgeSizes) -> Rect {
        Rect {
            x: self.x - edge.left,
            y: self.y - edge.top,
            width: self.width + edge.left + edge.right,
            height: self.height + edge.top + edge.bottom,
        }
    }
    pub fn contains(self, x: f32, y: f32) -> bool {
        self.x <= x && self.x + self.width >= x && self.y <= y && self.y + self.height > y
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct EdgeSizes {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

#[derive(Debug)]
pub struct LayoutBox {
    pub dimensions: Dimensions,
    pub box_type: BoxType,
    pub children: Vec<LayoutBox>,
}

#[derive(Debug)]
pub enum BoxType {
    BlockNode(Rc<StyledNode>),
    InlineNode(Rc<StyledNode>),
    InlineBlockNode(Rc<StyledNode>),
    AnonymousBlock(Rc<StyledNode>),
    TableNode(Rc<StyledNode>),
    TableRowGroupNode(Rc<StyledNode>),
    TableRowNode(Rc<StyledNode>),
    TableCellNode(Rc<StyledNode>),
    ListItemNode(Rc<StyledNode>),
}

// pub fn build_layout_tree<'a>(style_node: &Rc<StyledNode>, doc: &Document) -> LayoutBox {
//     let mut root = LayoutBox::new(match style_node.display() {
//         Display::Block => BlockNode(Rc::clone(style_node)),
//         Display::Inline => InlineNode(Rc::clone(style_node)),
//         Display::InlineBlock => InlineBlockNode(Rc::clone(style_node)),
//         Display::ListItem => BoxType::ListItemNode(Rc::clone(style_node)),
//         Display::Table => TableNode(Rc::clone(style_node)),
//         Display::TableRowGroup => TableRowGroupNode(Rc::clone(style_node)),
//         Display::TableRow => TableRowNode(Rc::clone(style_node)),
//         Display::TableCell => TableCellNode(Rc::clone(style_node)),
//         Display::None => panic!("Root node has display none."),
//     });

//     for child in style_node.children.borrow().iter() {
//         match child.display() {
//             Display::Block => root.children.push(build_layout_tree(child, doc)),
//             Display::ListItem => root.children.push(build_layout_tree(child, doc)),
//             Display::Inline => root
//                 .get_inline_container()
//                 .children
//                 .push(build_layout_tree(&child, doc)),
//             Display::InlineBlock => root
//                 .get_inline_container()
//                 .children
//                 .push(build_layout_tree(&child, doc)),
//             Display::Table => root.children.push(build_layout_tree(&child, doc)),
//             Display::TableRowGroup => root.children.push(build_layout_tree(&child, doc)),
//             Display::TableRow => root.children.push(build_layout_tree(&child, doc)),
//             Display::TableCell => root.children.push(build_layout_tree(&child, doc)),
//             Display::None => {}
//         }
//     }
//     root
// }
