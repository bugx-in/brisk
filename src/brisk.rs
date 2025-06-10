// src/brisk.rs

#![allow(dead_code)]
use whoami;
use std::thread;
use notify_rust::Notification;
use rumqttc::v5::mqttbytes::{v5::Publish, v5::Packet, QoS};
use rumqttc::v5::{AsyncClient, ClientError, ConnectionError, MqttOptions};
use rumqttc::{TlsConfiguration, Transport};
use serde_json::error;
use std::time::Duration;
use tokio::{task, time};
use crate::message::*;
use gethostname::gethostname;
use log::{debug, error, info, warn};

/// Brisk struct
#[derive(Clone, Debug)]
pub struct Brisk {
    pub broker: String,
    pub port: u16,
    pub topics: Vec<String>,
    pub keep_alive: u64,
    pub default_ca: bool,
    pub root_ca: Option<String>,
    pub client_cert: Option<String>,
    pub client_key: Option<String>,
}

impl Brisk {
    /// New Brisk object.
    pub fn new() -> Brisk {
        Brisk::default()
    }

    /// Broker hostname.
    pub fn broker(&mut self, broker: &str) -> &mut Brisk {
        broker.clone_into(&mut self.broker);
        self
    }

    /// Broker port.
    pub fn port(&mut self, port: &u16) -> &mut Brisk {
        port.clone_into(&mut self.port);
        self
    }

    /// Add topic to broker.
    pub fn topic(&mut self, topic: &str) -> &mut Brisk {
        self.topics.push(topic.to_string());
        self
    }

    /// Broker topics.
    pub fn topics(&mut self, topics: &Vec<String>) -> &mut Brisk {
        topics.clone_into(&mut self.topics);
        self
    }

    /// Keep alive.
    pub fn keep_alive(&mut self, keep_alive: &u64) -> &mut Brisk {
        keep_alive.clone_into(&mut self.keep_alive);
        self
    }

    /// Root CA.
    pub fn root_ca(&mut self, root_ca: &Option<String>) -> &mut Brisk {
        self.root_ca.clone_from(&root_ca);
        self
    }

    /// Client certificate
    pub fn client_cert(&mut self, client_cert: &Option<String>) -> &mut Brisk {
        self.client_cert.clone_from(&client_cert);
        self
    }

    /// Client key
    pub fn client_key(&mut self, client_key: &Option<String>) -> &mut Brisk {
        self.client_key.clone_from(&client_key);
        self
    }

    /// ID for the MQTT connection.
    fn id() -> String {
        gethostname().to_str().unwrap().to_string()
    }

    fn id_user() -> String {
        whoami::username()
    }

    /// Finalizes the builder.
    pub fn finalize(&self) -> Brisk {
        self.clone()
    }

    #[cfg(target_os = "windows")]
    /// Run the notification.
    fn notify(message: &Message) {
        // Initialize the notifier.
        let mut notifier = Notification::new()
            .summary(&message.summary)
            .body(&message.body)
            .icon(&message.icon)
            .timeout(0)
            .finalize();

         let _ = thread::spawn(move || {
            // Get the user's response.
            notifier.show().unwrap();
        });
    }

    #[cfg(target_os = "linux")]
    /// Run the notification.
    fn notify(message: &Message) {
        // Initialize the notifier.
        let mut notifier = Notification::new()
            .summary(&message.summary)
            .body(&message.body)
            .icon(&message.icon)
            .timeout(0)
            .finalize();

        // Add actions if any.
        for action in message.actions.iter().flatten() {
            notifier.action(&action.name, &action.display);
        }

        // Run the notifier and wait for action inside a thread.
        let mut action_str: String = String::new();

        let _ = thread::spawn(move || {
            // Get the user's response.
            notifier.show().unwrap().wait_for_action(|action | {
                debug!("Action from user: {action:?}");
                action_str.push_str(action)
            });

            // Execute action with non-blocking.
            if action_str == "action1" {
                let _ = open::with_detached("https://google.com", "firefox");
            }

        });
    }

    /// Parses the message received by MQTT.
    fn parse_mqtt_message(msg: Publish) -> Result<Message, error::Error> {
        // Convert payload bytes to string.
        let payload_str: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&msg.payload);
        debug!("Received message on topic {:?}: {:?}", msg.topic, payload_str);

        // Load the message.
        // Return error if message could not be parsed.
        let mqtt_message: Message = match Message::from_json(&payload_str.to_string()) {
            Ok(message) => message,
            Err(error) => return Err(error)
        };

        debug!("Message parsed: {:?}", mqtt_message);
        Ok(mqtt_message)
    }

    #[tokio::main(flavor = "current_thread")]
    /// Run brisker.
    pub async fn run(&self) -> Result<(), ClientError> {
        // Initialize MQTT client options.
        let mut mqttoptions = MqttOptions::new(Brisk::id_user(), self.broker.clone(), self.port.clone())
            .set_keep_alive(Duration::from_secs(self.keep_alive))
            .clone();

        // Use client authentication.
        if let (Some(root_ca), Some(client_cert), Some(client_key)) = (self.root_ca.as_ref(), self.client_cert.as_ref(), self.client_key.as_ref()) {
            let transport = Transport::Tls(TlsConfiguration::Simple {
                ca: root_ca.as_bytes().to_vec(),
                alpn: None,
                client_auth: Some((
                    client_cert.as_bytes().to_vec(),
                    client_key.as_bytes().to_vec(),
                )) 
            });
            mqttoptions.set_transport(transport);
        }

        debug!("Using MQTT options: {mqttoptions:?}");

        // Create a new MQTT client and subscribe to topics.
        let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
        for topic in &self.topics {
            debug!("Subscribing to topic: {:?}", topic);
            client.subscribe(topic, QoS::AtMostOnce).await?;
        }

        // Spawn a task to handle incoming messages.
        info!("Listening for MQTT packets...");
        task::spawn(async move {
            loop {
                match eventloop.poll().await {
                    Ok(notification) => {
                        if let rumqttc::v5::Event::Incoming(Packet::Publish(msg)) = notification {

                            // Load the message.
                            // If error reading message skip it.
                            let mqtt_message = match Brisk::parse_mqtt_message(msg) {
                                Err(e) => {
                                    warn!("Message could not be parsed: {e:?}");
                                    continue;
                                }
                                Ok(message) => message
                            };

                            // Run notifier
                            Brisk::notify(&mqtt_message);
                        }
                    }
                    Err(ConnectionError::Io(e)) => {
                        error!("Error receiving message from MQTT broker: {e:?}");
                        if let std::io::ErrorKind::ConnectionRefused = e.kind() {
                            warn!("Connection lost. Attempting to reconnect...");
                        } else {
                            warn!("Could not connect to the message broker, will try again in 1 minute.");
                            time::sleep(Duration::from_secs(60)).await;
                            
                        }
                    }
                    _ => {}
                }
            }
        });

        // Keep the main thread alive
        loop {
            time::sleep(Duration::from_secs(1)).await;
        }
    }

}

impl Default for Brisk {
    fn default() -> Brisk {
        Brisk { broker: String::new(),
                port: 1883,
                topics: Vec::new(),
                keep_alive: 5,
                default_ca: false,
                root_ca: None,
                client_cert: None,
                client_key: None }
    }
    
}
