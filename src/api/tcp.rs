use crate::api::gripper;
use crate::model::positions::Position;
use std::error::Error;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::time::Duration;

pub struct RobotArm {
    pub stream: TcpStream,
}

impl RobotArm {
    pub async fn start_tcp(address: &str) -> Result<Self, Box<dyn Error>> {
        let mut stream = TcpStream::connect(address).await?;
        // Send Gripper Reset Request
        println!("Resetting Gripper RQ");
        let command = "rq_reset_and_wait()\n";
        stream
            .write_all(gripper::generate_gripper_command(command.to_string()).as_bytes())
            .await
            .unwrap();
        tokio::time::sleep(Duration::from_secs(3)).await;

        // Send Gripper Activation Request
        println!("Activating Gripper RQ");
        let command = "rq_activate_and_wait()\n";
        stream
            .write_all(gripper::generate_gripper_command(command.to_string()).as_bytes())
            .await
            .unwrap();
        tokio::time::sleep(Duration::from_secs(3)).await;

        println!("Starting TCP connection with {}", address);
        Ok(RobotArm { stream })
    }

    pub async fn send_script(&mut self, position: Position) -> Result<(), Box<dyn Error>> {
        let _start = position.dice_position;
        let _end = position.roll_position;

        let script = "def dice_roll():\n".to_string() + "move";
        self.stream.write_all(script.as_bytes()).await?;

        Ok(())
    }
}
