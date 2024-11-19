pub mod execute_strategy;

pub use execute_strategy::*;


// pub async fn setup_jobs() -> Result<(), Box<dyn std::error::Error>> {
//     let sched = JobScheduler::new().await?;

//     // Create job to check strategies every 2 seconds
//     sched.add(Job::new_async("2/2 * * * * *", move |_, _| {
//         Box::pin(async move {
//             match check_strategies(&app_state).await {
//                 Ok(_) => println!("Successfully checked strategies"),
//                 Err(e) => eprintln!("Error checking strategies: {}", e),
//             }
//         })
//     })?).await?;

//     // Start the scheduler
//     sched.start().await?;

//     Ok(())
// }
