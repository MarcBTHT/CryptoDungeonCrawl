use crate::prelude::*;

//Parcourt tout les monstres et les déplace d'une case dans la direction du joueur
#[system]
#[write_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
pub fn move_towards_player(ecs: &mut SubWorld, #[resource] map: &Map) {
    let player_pos = <&Point>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .next()
        .copied()
        .unwrap();

    let mut enemies = <(&mut Point, &Enemy)>::query();
    enemies
        .iter_mut(ecs)
        .for_each(|(pos, _)| {
            let delta = Point::new(
                (player_pos.x - pos.x).signum(),
                (player_pos.y - pos.y).signum(),
            );
            let new_pos = *pos + delta;
            if map.can_enter_tile(new_pos) {
                *pos = new_pos;
            }
        }
    );
}
