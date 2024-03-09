use bevy::{ prelude::*};

fn main() {
    //test app run
    App::new()
    .add_plugins(PeoplePlugin)
    .run();
}

pub struct PeoplePlugin;

impl Plugin for PeoplePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, setup)
        .add_systems(Update, people_with_jobs)
        .add_systems(Update,people_jobs)
        .add_systems(Update, people_without_jobs)
        .add_systems(Update, prints_name);    
    }
}

//system test 
pub fn setup(mut commands:Commands) {

    //bundle pack of component
    commands.spawn((Person{
        name: "Yoann".to_string(),
    },
    Employed {job: Jobs::Doctor} 
    ));
     
    //no access to employees not in the new bundle
    commands.spawn(Person {
        name: "Thomas".to_string(),
    });
}

pub fn people_with_jobs(person_query:Query<&Person, With<Employed>> ) {
    for person in person_query.iter() {
        println!("{} have a job.", person.name);
    }
} 

pub fn people_without_jobs(person_query: Query<&Person, Without<Employed>>) {
    for people in person_query.iter() {
        println!("{} is ready to be hired",people.name);
    }
}


pub fn people_jobs(person_query: Query<(&Person, &Employed)>) {   
    for (person, employed) in person_query.iter() {
       let job_name:&str= match employed.job {
           Jobs::Cooker => "Cooker",
           Jobs::Doctor => "Doctor",
           Jobs::FireFighter => "Firefighter",
           Jobs::Lawyer => "lawyer",
           Jobs::Police => "Policeman"
       }; 
       println!("{0} job is {1}", person.name, job_name)
    }
}
// test query (query give access to a component and gice right to act them)
pub fn prints_name(person_query:Query<&Person>) {
    for person in person_query.iter() {
        println!("Name of the person is {}", person.name);
    }
}

 //component test
 #[derive(Component)]
pub struct Person {
    name:String,
}
#[derive(Component)]


pub struct Employed {
    job: Jobs
}


pub enum Jobs {
    Doctor, 
    Cooker,
    Lawyer, 
    Police,
    FireFighter
}