# Brisk

<p align="center">
  <img src="https://github.com/user-attachments/assets/cb182654-9a53-402c-8dec-d4eb1c5b430e" width="300">
</p>

Generate remote custom desktop notifications for users using MQTT.

The brisk client subscribes to a topic in the MQTT broker and waits for messages. When a message is received in that topic it is parsed and converted to a desktop notification.

The notification is created using notify-send and a D-BUS monitor is used to retrieve the user's action.

## Requirements

- MQTT broker.

## Usage

```
Brisk command line interface

Usage: brisk [OPTIONS] --broker <BROKER>

Options:
  -b, --broker <BROKER>          Hostname of the broker [env: BRISK_BROKER=]
  -p, --port <PORT>              Port of the broker [env: BRISK_BROKER_PORT=] [default: 1883]
  -t, --topics <TOPICS>          Topic of the broker [env: BRISK_TOPICS=]
  -k, --keep-alive <KEEP_ALIVE>  Topic of the broker [env: BRISK_KEEP_ALIVE=] [default: 5]
  -h, --help                     Print help
  -V, --version                  Print version
```

## Notifications

**MQTT message**

```
$ mosquitto_pub -h <BROKER_HOSTNAME> -p 1883 -t "brisk" -m '{"id": 23, "summary": "Alert issues", "body": "There is an issue in the infrastructure, refer to this FAQ", "icon": "firefox", "actions": [{"name": "action1", "display": "Go to FAQ"}, {"name": "action2", "display": "Dismiss"}]}'
```

**Desktop notification**

![image](https://github.com/user-attachments/assets/bf3e32bb-51f2-4e95-9abe-e73cc1c33af4)

## Roadmap

- [x] Support MQTT 5.1
- [ ] Support TLS
- [ ] Support client user:password
