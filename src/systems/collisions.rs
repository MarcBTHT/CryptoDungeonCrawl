use crate::prelude::*;

#[system]
#[write_component(Health)]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
pub fn collisions(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut player_pos = Point::zero();
    let mut player_entity = None;

    // Trouve la position et l'entité du joueur
    let mut players = <(Entity, &Point)>::query().filter(component::<Player>());
    for (ent, pos) in players.iter(ecs) {
        player_pos = *pos;
        player_entity = Some(*ent);
        break;
    }

    // Identifie les ennemis en collision
    let mut enemies_to_remove = Vec::new();
    let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());
    for (enemy_ent, enemy_pos) in enemies.iter(ecs) {
        if *enemy_pos == player_pos {
            enemies_to_remove.push(*enemy_ent);
        }
    }

    // Traite les collisions
    for enemy_ent in enemies_to_remove {
        commands.remove(enemy_ent); // Supprimer l'ennemi

        if let Some(player_ent) = player_entity {
            // Réduire la santé du joueur
            if let Ok(mut health) = ecs.entry_mut(player_ent).unwrap().get_component_mut::<Health>() {
                health.current -= 1;
            }
        }
    }
}
