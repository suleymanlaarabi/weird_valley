use bevy::prelude::*;

pub struct NodeBuilder(Node);

impl NodeBuilder {
    pub fn width(mut self, width: f32) -> Self {
        self.0.width = Val::Px(width);
        self
    }
    pub fn display_none(mut self) -> Self {
        self.0.display = Display::None;
        self
    }
    pub fn height(mut self, height: f32) -> Self {
        self.0.height = Val::Px(height);
        self
    }
    pub fn dimension(mut self, dimension: Vec2) -> Self {
        self.0.height = Val::Px(dimension.y);
        self.0.width = Val::Px(dimension.x);
        self
    }
    pub fn build(self) -> Node {
        self.0
    }
    pub fn padding(mut self, all: f32) -> Self {
        self.0.padding = UiRect::all(Val::Px(all));
        self
    }
    pub fn flex_center(mut self) -> Self {
        self.0.justify_content = JustifyContent::Center;
        self.0.align_items = AlignItems::Center;

        self
    }
}

pub trait AsNodeBuilder {
    fn as_node_builder(self) -> NodeBuilder;

    fn node_builder() -> NodeBuilder {
        NodeBuilder(Node::DEFAULT)
    }
}

impl AsNodeBuilder for Node {
    fn as_node_builder(self) -> NodeBuilder {
        NodeBuilder(self)
    }
}
