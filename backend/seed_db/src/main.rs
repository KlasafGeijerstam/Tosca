use db_connector::queue::*;
use db_connector::signup::*;
use db_connector::workspace::*;
use db_connector::{create_db_pool, create_db_pool_env};

use chrono::naive::NaiveDateTime;
use fake::faker::company::en::*;
use fake::faker::internet::en::*;
use fake::faker::lorem::en::*;
use fake::faker::name::en::*;
use fake::Fake;
use rand::thread_rng;
use rand::{RngCore, Rng};
use std::collections::{HashMap, HashSet};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use user_format::UserWithID;
use user_format::*;

use progressing::{mapping::Bar, Baring};

use structopt::StructOpt;

#[derive(StructOpt)]
struct Args {
    /// The output (toml) file
    output_file: String,

    /// Connect to database via this URL instead of loading from
    /// the DATABASE_URL environment variable.
    #[structopt(long)]
    database_url: Option<String>,

    /// The amount of workspaces to generate
    #[structopt(long, default_value = "10")]
    workspace_count: usize,
}

const SUPER: u8 = 0;
const ADMIN: u8 = 1;
const NORMAL: u8 = 2;

fn generate_user(level: u8) -> UserWithID {
    UserWithID {
        user_id: Username().fake(),
        first_name: FirstName().fake(),
        last_name: LastName().fake(),
        user_level: level,
        workspaces: vec![],
    }
}

trait RandomEntry<T> {
    fn random(&self) -> &T;
}

impl<T> RandomEntry<T> for Vec<T> {
    fn random(&self) -> &T {
        let mut random = thread_rng();
        &self[random.gen_range(0..self.len())]
    }
}

fn time() -> Duration {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap()
}

fn get_random_date() -> NaiveDateTime {
    let mut rng = thread_rng();
    // Generate date between now and 4 days ahead
    let start = time().as_secs() + rng.gen_range(0..(24 * 4)) * 60 * 60;
    NaiveDateTime::from_timestamp(start as i64, 0)
}

fn get_past_time() -> NaiveDateTime {
    let mut rng = thread_rng();
    let start = time().as_secs() - rng.gen_range(0..(60 * 60));
    NaiveDateTime::from_timestamp(start as i64, 0)
}

fn main() {
    let args = Args::from_args();

    let pool = if let Some(db_url) = args.database_url {
        create_db_pool(&db_url).expect("Failed to connect to database")
    } else {
        create_db_pool_env().expect("Failed to connect to database")
    };

    let connection = pool.get().expect("Failed to get db-handle");

    let mut rng = thread_rng();
    let super_user = {
        let mut user = generate_user(SUPER);
        user.user_id = String::from("admin");

        user
    };
    let super_user = super_user;

    let admins: Vec<_> = (0..5).map(|_| generate_user(ADMIN)).collect();
    let normals: Vec<_> = (0..80).map(|_| generate_user(NORMAL)).collect();
    println!("Generating {} workspaces", args.workspace_count);
    let mut bar = Bar::with_range(0, args.workspace_count);

    let workspaces: Vec<_> = (0..args.workspace_count)
        .map(|i| {
            bar.set(i);
            println!("{}", bar);

            let name = Industry().fake();
            let wspace = NewWorkspace {
                name,
                info: &Paragraph(20..100).fake::<String>(),
                creator: &admins.random().user_id,
                remote_workspace_id: Some(name),
            };

            let wspace = add_workspace(&connection, &wspace).unwrap();

            for _ in (0..).take(rng.gen_range(1..=5)) {
                let q = NewQueue {
                    workspace_id: wspace.id,
                    name: Buzzword().fake(),
                    info: &Paragraph(5..50).fake::<String>(),
                };

                let q = add_queue(&connection, &q).unwrap();

                for _ in (0..).take(rng.gen_range(2..10)) {
                    let qs = NewQueueSlot {
                        queue_id: q.id,
                        duration: rng.gen_range((60 * 60)..(60 * 60 * 4)),
                        open_before: rng.gen_range(0..(60 * 60)),
                        start_time: get_random_date(),
                    };

                    let qs = add_queue_slot(&connection, &qs).unwrap();

                    let indices: HashSet<_> = (0..)
                        .take(rng.gen_range(2..=10))
                        .map(|_| rng.gen_range(0..normals.len()))
                        .collect();

                    for user in indices.iter().map(|&i| &normals[i]) {
                        let qsu = NewQueueSlotUser {
                            queue_slot_id: qs.id,
                            user_id: &user.user_id,
                            message: &Sentence(0..50).fake::<String>(),
                            moderator_message: &Sentence(0..50).fake::<String>(),
                            q_time: get_past_time(),
                        };

                        add_queue_slot_user(&connection, &qsu).unwrap();
                    }
                }
            }

            for _ in (0..).take(rng.gen_range(1..=5)) {
                let s = NewSignup {
                    workspace_id: wspace.id,
                    name: Buzzword().fake(),
                    info: &Paragraph(5..50).fake::<String>(),
                    max_slot_signup: rng.gen_range(1..3),
                };

                let s = add_signup(&connection, &s).unwrap();

                for _ in (0..).take(rng.gen_range(2..15)) {
                    let time = if rng.next_u32() % 2 == 0 {
                        Some(get_random_date())
                    } else {
                        None
                    };

                    let ss = NewSignupSlot {
                        signup_id: s.id,
                        max_users: rng.gen_range(1..5),
                        info: &Sentence(0..10).fake::<String>(),
                        time,
                    };

                    let ss = add_signup_slot(&connection, &ss).unwrap();

                    for _ in (0..).take(rng.gen_range(0..ss.max_users) as usize) {
                        let ssu = NewSignupSlotUser {
                            user_id: &normals.random().user_id,
                            signup_slot_id: ss.id,
                        };

                        add_signup_slot_user(&connection, &ssu).ok();
                    }
                }
            }

            wspace
        })
        .collect();

    let workspaces = workspaces
        .iter()
        .map(|wspace| {
            let users = (5..rng.gen_range(6..50))
                .map(|_| normals.random().user_id.clone())
                .collect();
            (
                wspace.remote_workspace_id.as_ref().unwrap().clone(),
                user_format::Workspace { users },
            )
        })
        .collect();

    let mut users = HashMap::new();
    users.insert(super_user.user_id.clone(), super_user.to_user());

    let mut admins = admins;
    admins.drain(..).for_each(|admin| {
        users.insert(admin.user_id.clone(), admin.to_user());
    });

    let mut normals = normals;
    normals.drain(..).for_each(|user| {
        users.insert(user.user_id.clone(), user.to_user());
    });

    let cfg = Config { users, workspaces };

    let data = toml::to_string_pretty(&cfg).unwrap();
    std::fs::write(args.output_file, data).unwrap();
}
