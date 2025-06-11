# Brisk

<p align="center">
  <img src="https://github.com/user-attachments/assets/b9cf2959-e55c-4abc-b5a9-0f9c7efe5e7a" width="300">
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
  -b, --broker <BROKER>            Hostname of the broker [env: BRISK_BROKER=]
  -p, --port <PORT>                Port of the broker [env: BRISK_BROKER_PORT=] [default: 1883]
  -t, --topics <TOPICS>            Topics of the broker [env: BRISK_TOPICS=] [default: brisk]
  -k, --keep-alive <KEEP_ALIVE>    Maximum time in seconds allowed to elapse between MQTT packets sent by the client [env: BRISK_KEEP_ALIVE=] [default: 20]
      --root-ca <ROOT_CA>          Root CA certificate for TLS connection [env: BRISK_ROOT_CA=]
      --client-cert <CLIENT_CERT>  Client certificate for mTLS connection [env: BRISK_CLIENT_CERT=]
      --client-key <CLIENT_KEY>    Client key for mTLS connection [env: BRISK_CLIENT_KEY=]
  -h, --help                       Print help
  -V, --version                    Print version
```

## Notifications

**MQTT message**

```
$ mosquitto_pub -h <BROKER_HOSTNAME> -p 1883 -t "brisk" -m '{"id": 23, "summary": "Alert issues", "body": "There is an issue in the infrastructure, refer to this FAQ", "icon": "firefox", "actions": [{"name": "action1", "display": "Go to FAQ"}, {"name": "action2", "display": "Dismiss"}]}'
```

**Desktop notification Linux**

![image](https://github.com/user-attachments/assets/42521023-fc18-49f7-acf4-6c9bc70bcff9)

**Desktop notification Windows**

*For windows actions are not supported*

![image](https://github.com/user-attachments/assets/6cdda858-2802-4d41-861e-d9ccc4007294)

## Roadmap

- [x] Support MQTT 5.1
- [x] Support TLS
- [x] Support Windows
- [ ] Support client user:password
