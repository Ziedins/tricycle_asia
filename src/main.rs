use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .run();
}

fn hello_world() {
    println!("hello world!");
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("John Goodman".to_string())));
    commands.spawn((Person, Name("Santa Claus".to_string())));
    commands.spawn((Person, Name("Robert Deniro".to_string())));
}

fn greet_people(query: Query<&Name, With<Person>>)
{
   for name in &query {
        print!("Hello {}", name.0);
   }

}

pub struct HelloPlugin;
impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(add_people)
        .add_system(hello_world)
        .add_system(greet_people);
    }
}
