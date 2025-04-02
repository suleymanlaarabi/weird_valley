use inventory_ui::components::InventoryUi;

use crate::prelude::*;

pub fn handle_inventory_toggle(
    mut query: Query<&mut Node, With<InventoryUi>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::KeyI) {
        for mut node in &mut query {
            if node.display == Display::None {
                node.display = Display::Flex
            } else {
                node.display = Display::None
            }
        }
    }
}
