// <-- importation des librairies
use bevy::prelude::*;
use rand::prelude::random;
use bevy::core::FixedTimestep;

// <-- Declaration des structures et des constantes
#[derive(Component)]
struct Snake1Head {
    direction: Direction,
}

struct GameOver1Event;

struct GameOver2Event;


#[derive(Component)]
struct Snake2Head {
    direction: Direction,
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Food;

#[derive(Component)]
struct Size {
    width: f32,
    height: f32,
}
impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}


impl Direction {
    fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
}

#[derive(Default)]
struct LastTailPosition1(Option<Position>);

#[derive(Default)]
struct LastTailPosition2(Option<Position>);

#[derive(Component)]
struct Snake1Segment;

#[derive(Component)]
struct Snake2Segment;


#[derive(Default)]
struct Snake1Segments(Vec<Entity>);


#[derive(Default)]
struct Snake2Segments(Vec<Entity>);

#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub enum Snake1Movement {
    Input,
    Movement,
    Eating,
    Growth,
}

#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub enum Snake2Movement {
    Input,
    Movement,
    Eating,
    Growth,
}

struct GrowthEvent1;

struct GrowthEvent2;

const ARENA_WIDTH: u32 = 10;
const ARENA_HEIGHT: u32 = 10;

const SNAKE1_HEAD_COLOR: Color = Color::rgb(1.0, 0.0, 0.0);
const SNAKE2_HEAD_COLOR: Color = Color::rgb(0.0, 0.0, 1.0);
const SNAKE1_SEGMENT_COLOR: Color = Color::rgb(1.0, 0.0, 0.0);
const SNAKE2_SEGMENT_COLOR: Color = Color::rgb(0.0, 0.0, 1.0);


const FOOD_COLOR: Color = Color::rgb(1.0, 0.0, 1.0); 


// <-- Affichage Snake 1
fn spawn_snake1(mut commands: Commands, mut segments: ResMut<Snake1Segments>) {
    segments.0 = vec![
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: SNAKE1_HEAD_COLOR,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Snake1Head {
                direction: Direction::Up,
            })
            .insert(Snake1Segment)
            .insert(Position { x: 4, y: 4 })
            .insert(Size::square(0.8))
            .id(),
        spawn_segment1(commands, Position { x: 4, y: 3 }),
    ];
}

// <-- Affichage Snake 2
fn spawn_snake2(mut commands: Commands, mut segments: ResMut<Snake2Segments>) {
    segments.0 = vec![
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: SNAKE2_HEAD_COLOR,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Snake2Head {
                direction: Direction::Up,
            })
            .insert(Snake2Segment)
            .insert(Position { x: 2, y: 2 })
            .insert(Size::square(0.8))
            .id(),
        spawn_segment2(commands, Position { x: 2, y: 1 }),
    ];
}

// <-- Gestion des mouvement Snake 1
fn snake1_movement(
    segments: ResMut<Snake1Segments>,
    mut heads: Query<(Entity, &Snake1Head)>,
    mut positions: Query<&mut Position>,
    mut last_tail_position: ResMut<LastTailPosition1>,
    mut game_over_writer1: EventWriter<GameOver1Event>,
    mut game_over_writer2: EventWriter<GameOver2Event>,
) {
    if let Some((head_entity, head)) = heads.iter_mut().next() {
        let segment_positions = segments
            .0
            .iter()
            .map(|e| *positions.get_mut(*e).unwrap())
            .collect::<Vec<Position>>();
        let mut head_pos = positions.get_mut(head_entity).unwrap();
        match &head.direction {
            Direction::Left => {
                head_pos.x -= 1;
            }
            Direction::Right => {
                head_pos.x += 1;
            }
            Direction::Up => {
                head_pos.y += 1;
            }
            Direction::Down => {
                head_pos.y -= 1;
            }
        };
        if head_pos.x < 0
            || head_pos.y < 0
            || head_pos.x as u32 >= ARENA_WIDTH
            || head_pos.y as u32 >= ARENA_HEIGHT
        {
            game_over_writer1.send(GameOver1Event);
        }
        if segment_positions.contains(&head_pos) {
            game_over_writer1.send(GameOver1Event);
        }

        segment_positions
            .iter()
            .zip(segments.0.iter().skip(1))
            .for_each(|(pos, segment)| {
                *positions.get_mut(*segment).unwrap() = *pos;
            });
            last_tail_position.0 = Some(*segment_positions.last().unwrap());
    }
}

// <-- Gestion des mouvement Snake 2
fn snake2_movement(
    segments: ResMut<Snake2Segments>,
    mut heads: Query<(Entity, &Snake2Head)>,
    mut positions: Query<&mut Position>,
    mut last_tail_position: ResMut<LastTailPosition2>,
    mut game_over_writer2: EventWriter<GameOver2Event>,
) {
    if let Some((head_entity, head)) = heads.iter_mut().next() {
        let segment_positions = segments
            .0
            .iter()
            .map(|e| *positions.get_mut(*e).unwrap())
            .collect::<Vec<Position>>();
        let mut head_pos = positions.get_mut(head_entity).unwrap();
        match &head.direction {
            Direction::Left => {
                head_pos.x -= 1;
            }
            Direction::Right => {
                head_pos.x += 1;
            }
            Direction::Up => {
                head_pos.y += 1;
            }
            Direction::Down => {
                head_pos.y -= 1;
            }
        };
        if head_pos.x < 0
            || head_pos.y < 0
            || head_pos.x as u32 >= ARENA_WIDTH
            || head_pos.y as u32 >= ARENA_HEIGHT
        {
            game_over_writer2.send(GameOver2Event);
        }
        if segment_positions.contains(&head_pos) {
            game_over_writer2.send(GameOver2Event);
        }
        segment_positions
            .iter()
            .zip(segments.0.iter().skip(1))
            .for_each(|(pos, segment)| {
                *positions.get_mut(*segment).unwrap() = *pos;
            });
            last_tail_position.0 = Some(*segment_positions.last().unwrap());
    }
    
}

// <-- Recuperation des mouvement voulu par l'utilisateur par les touches clavier pour le Snake 1
fn snake1_movement_input(keyboard_input: Res<Input<KeyCode>>, mut heads: Query<&mut Snake1Head>) {
    if let Some(mut head) = heads.iter_mut().next() {
        let dir: Direction = if keyboard_input.pressed(KeyCode::Left) {
            Direction::Left
        } else if keyboard_input.pressed(KeyCode::Down) {
            Direction::Down
        } else if keyboard_input.pressed(KeyCode::Up) {
            Direction::Up
        } else if keyboard_input.pressed(KeyCode::Right) {
            Direction::Right
        } else {
            head.direction
        };
        if dir != head.direction.opposite() {
            head.direction = dir;
        }
    }
}

// <-- Recuperation des mouvement voulu par l'utilisateur par les touches clavier pour le Snake 2
fn snake2_movement_input(keyboard_input: Res<Input<KeyCode>>, mut heads: Query<&mut Snake2Head>) {
    if let Some(mut head) = heads.iter_mut().next() {
        let dir: Direction = if keyboard_input.pressed(KeyCode::Q) {
            Direction::Left
        } else if keyboard_input.pressed(KeyCode::S) {
            Direction::Down
        } else if keyboard_input.pressed(KeyCode::Z) {
            Direction::Up
        } else if keyboard_input.pressed(KeyCode::D) {
            Direction::Right
        } else {
            head.direction
        };
        if dir != head.direction.opposite() {
            head.direction = dir;
        }
    }
}

// <-- Apparition de la queue du Snake 1
fn spawn_segment1(mut commands: Commands, position: Position) -> Entity {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: SNAKE1_SEGMENT_COLOR,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Snake1Segment)
        .insert(position)
        .insert(Size::square(0.65))
        .id()
}

// <-- Apparition de la queue du Snake 2
fn spawn_segment2(mut commands: Commands, position: Position) -> Entity {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: SNAKE2_SEGMENT_COLOR,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Snake2Segment)
        .insert(position)
        .insert(Size::square(0.65))
        .id()
}

// <-- Fonction Manger du Snake 1
fn snake1_eating(
    mut commands: Commands,
    mut growth_writer: EventWriter<GrowthEvent1>,
    food_positions: Query<(Entity, &Position), With<Food>>,
    head_positions: Query<&Position, With<Snake1Head>>,
) {
    for head_pos in head_positions.iter() {
        for (ent, food_pos) in food_positions.iter() {
            if food_pos == head_pos {
                commands.entity(ent).despawn();
                growth_writer.send(GrowthEvent1);
            }
        }
    }
}

// <-- Fonction Manger du Snake 2
fn snake2_eating(
    mut commands: Commands,
    mut growth_writer: EventWriter<GrowthEvent2>,
    food_positions: Query<(Entity, &Position), With<Food>>,
    head_positions: Query<&Position, With<Snake2Head>>,
) {
    for head_pos in head_positions.iter() {
        for (ent, food_pos) in food_positions.iter() {
            if food_pos == head_pos {
                commands.entity(ent).despawn();
                growth_writer.send(GrowthEvent2);
            }
        }
    }
}

// <-- recuperation de la position de la pomme pour l'IA
fn cooPomme(
    food_positions: Query<(Entity, &Position), With<Food>>,
){
    for (ent, food_pos) in food_positions.iter() {
            println!("position pomme : {}, {}",food_pos.x, food_pos.y);
        }
    
} 

// <-- recuperation de la position du serpent pour l'IA
fn cooSnake2(
    head_positions: Query<&Position, With<Snake2Head>>,
){
        for head_pos in head_positions.iter()  {
            println!("position serpent : {}, {}",head_pos.x, head_pos.y);
        }
    
} 

// <-- tentative de deplacement du serpent2 pour l'IA
// fn snake2_movement_input(head_positions: Query<&Position, With<Snake2Head>>,
//     food_positions: Query<(Entity, &Position), With<Food>>, mut heads: Query<&mut Snake2Head>,) {
//     if let Some(mut head) = heads.iter_mut().next() {
//         for (ent, food_pos) in food_positions.iter() {
//             for head_pos in head_positions.iter()  {
//                 let dir: Direction = 
//                 if (head_pos.x > food_pos.x) {
//                     Direction::Left
//                 } else if (head_pos.x < food_pos.x) {
//                     Direction::Right
//                 } 
//                 else {
//                     if (head_pos.y < food_pos.y) {
//                         Direction::Down
//                     } else {
//                         Direction::Up
//                     }
//                 };
                    
//         }
//     }
//     }
// } 


// <-- Fonction qui fait grandire le Snake 1 quand il mange une pomme
fn snake1_growth(
    commands: Commands,
    last_tail_position: Res<LastTailPosition1>,
    mut segments: ResMut<Snake1Segments>,
    mut growth_reader: EventReader<GrowthEvent1>,
) {
    if growth_reader.iter().next().is_some() {
        segments
            .0
            .push(spawn_segment1(commands, last_tail_position.0.unwrap()));
    }
}

// <-- Fonction qui fait grandire le Snake 2 quand il mange une pomme
fn snake2_growth(
    commands: Commands,
    last_tail_position: Res<LastTailPosition2>,
    mut segments: ResMut<Snake2Segments>,
    mut growth_reader: EventReader<GrowthEvent2>,
) {
    if growth_reader.iter().next().is_some() {
        segments
            .0
            .push(spawn_segment2(commands, last_tail_position.0.unwrap()));
    }
}

// <-- Fonction qui fait apparaitre aléatoirement des pomme à intervalles de temps régulieres
fn food_spawner(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: FOOD_COLOR,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Food)
        .insert(Position {
            x: (random::<f32>() * ARENA_WIDTH as f32) as i32,
            y: (random::<f32>() * ARENA_HEIGHT as f32) as i32,
        })
        .insert(Size::square(0.8));
}

// <-- Gestion de la taille de l'arene
fn size_scaling(windows: Res<Windows>, mut q: Query<(&Size, &mut Transform)>) {
    let window = windows.get_primary().unwrap();
    for (sprite_size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(
            sprite_size.width / ARENA_WIDTH as f32 * window.width() as f32,
            sprite_size.height / ARENA_HEIGHT as f32 * window.height() as f32,
            1.0,
        );
    }
}

// <-- Gestion des mouvement et positions dans l'arene
fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }
    let window = windows.get_primary().unwrap();
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, ARENA_WIDTH as f32),
            convert(pos.y as f32, window.height() as f32, ARENA_HEIGHT as f32),
            0.0,
        );
    }
}
// <-- Fonction gameOver/Replay pour le Snake 1
fn game_over1(
    mut commands: Commands,
    mut reader: EventReader<GameOver1Event>,
    segments_res: ResMut<Snake1Segments>,
    food: Query<Entity, With<Food>>,
    segments: Query<Entity, With<Snake1Segment>>,
) {
    if reader.iter().next().is_some() {
        for ent in food.iter().chain(segments.iter()) {
            commands.entity(ent).despawn();
        }
        spawn_snake1(commands, segments_res);
    }
}

// <-- Fonction gameOver/Replay pour le Snake 2
fn game_over2(
    mut commands: Commands,
    mut reader: EventReader<GameOver2Event>,
    segments_res: ResMut<Snake2Segments>,
    food: Query<Entity, With<Food>>,
    segments: Query<Entity, With<Snake2Segment>>,
) {
    if reader.iter().next().is_some() {
        for ent in food.iter().chain(segments.iter()) {
            commands.entity(ent).despawn();
        }
        spawn_snake2(commands, segments_res);
    }
}

// <-- Fonction main qui permet le lancement de toute les autres fonction avec leur declaration 
fn main() {
    App::new()
    .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
    .insert_resource(WindowDescriptor {
        title: "Snake de Julien & Marie !".to_string(), 
        width: 500.0,                 
        height: 500.0,                
        ..Default::default()         
    })
    .insert_resource(Snake1Segments::default()) // <-- Ajout queue serpent 1
    .insert_resource(Snake2Segments::default()) // <-- Ajout queue serpent 2
    .insert_resource(LastTailPosition1::default())
    .insert_resource(LastTailPosition2::default())
    .add_event::<GrowthEvent1>()
    .add_event::<GrowthEvent2>()
    .add_event::<GameOver1Event>()
    .add_event::<GameOver2Event>()
    .add_startup_system(setup_camera) 
    .add_startup_system(spawn_snake1) // <-- faire apparaitre le snake 1
    .add_startup_system(spawn_snake2) // <- faire apparaitre le snake 2
    .add_system(
        snake1_movement_input
            .label(Snake1Movement::Input)
            .before(Snake1Movement::Movement),
    )
    .add_system_set(
        SystemSet::new()
            .with_run_criteria(FixedTimestep::step(0.150))
            .with_system(snake1_movement.label(Snake1Movement::Movement))
            .with_system(
                snake1_eating
                    .label(Snake1Movement::Eating)
                    .after(Snake1Movement::Movement),
            )
            .with_system(
                snake1_growth
                    .label(Snake1Movement::Growth)
                    .after(Snake1Movement::Eating),
            )
    ) // <-- mouvement du snake 1
    .add_system(
        snake2_movement_input
            .label(Snake2Movement::Input)
            .before(Snake2Movement::Movement),
    )
    .add_system_set(
        SystemSet::new()
            .with_run_criteria(FixedTimestep::step(0.150))
            .with_system(snake2_movement.label(Snake2Movement::Movement))
            .with_system(
                snake2_eating
                    .label(Snake2Movement::Eating)
                    .after(Snake2Movement::Movement),
            )
            .with_system(
                snake2_growth
                    .label(Snake2Movement::Growth)
                    .after(Snake2Movement::Eating),
            )
            
    ) // <-- mouvement du snake 2
    .add_system(
        cooPomme
    )
    .add_system(
        cooSnake2
    )
            

    .add_plugins(DefaultPlugins)
    .add_system_set_to_stage(
        CoreStage::PostUpdate,
        SystemSet::new()
            .with_system(position_translation)
            .with_system(size_scaling),
    )
    .add_system(game_over1.after(Snake1Movement::Movement))
    .add_system(game_over2.after(Snake2Movement::Movement))
    .add_system_set(
        SystemSet::new()
            .with_run_criteria(FixedTimestep::step(1.0))
            .with_system(food_spawner),
    )
    .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}



