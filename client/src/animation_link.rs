use bevy::prelude::*;

#[derive(Component, Reflect, Debug)]
pub struct AnimationEntityLink(pub Entity);

pub struct AnimationEntityLinkPlugin;

impl Plugin for AnimationEntityLinkPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<AnimationEntityLink>()
            .add_systems(PostUpdate, link_animations);
    }
}

fn get_top_parent(mut curr_entity: Entity, parent_query: &Query<&ChildOf>) -> Entity {
    //Loop up all the way to the top parent
    while let Ok(parent) = parent_query.get(curr_entity) {
        curr_entity = parent.parent();
    }

    curr_entity
}

fn link_animations(
    player_query: Query<Entity, Added<AnimationPlayer>>,
    parent_query: Query<&ChildOf>,
    animations_entity_link_query: Query<&AnimationEntityLink>,
    mut commands: Commands,
) {
    // Get all the Animation players which can be deep and hidden in the heirachy
    for entity in player_query.iter() {
        let top_entity = get_top_parent(entity, &parent_query);

        // If the top parent has an animation config ref then link the player to the config
        if animations_entity_link_query.get(top_entity).is_ok() {
            warn!("Problem with multiple animationsplayers for the same top parent");
        } else {
            commands
                .entity(top_entity)
                .insert(AnimationEntityLink(entity));
        }
    }
}
