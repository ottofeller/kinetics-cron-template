use aws_sdk_sqs::operation::send_message::builders::SendMessageFluentBuilder;
use kinetics_macro::cron;
use std::collections::HashMap;

// Learn more about schedule expression here
// https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-scheduler-schedule.html#cfn-scheduler-schedule-scheduleexpression
#[cron(schedule = "rate(1 hour)")]
pub async fn cron(
    _secrets: &HashMap<String, String>,
    _queues: &HashMap<String, SendMessageFluentBuilder>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Started cron job");
    Ok(())
}
