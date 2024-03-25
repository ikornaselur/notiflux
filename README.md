# notiflux

notiflux is a pub/sub server where clients subscribe over a WebSocket and
messages are broadcast over a POST request

## How does it work?

Client connect via WebSocket and subscribe to topics. A publishing source
will make POST requests to the broadcast endpoint, with a message and a topic,
which will then publish the message to all connected clients.

### Usage

#### Subscribing to topics

Clients connect over WebSocket and subscribe to the topics, which can be any
string. The idea is that they could subscribe to something like
`campaign:<campaign_id>` for notifications about a specific campaign, or even
just `campaigns` for all campaign updates.

This is by calling the subscribe command with the topic and a token (see next
section for token info)

```
/subscribe <topic> <token>
```

clients can then unsubscribe with

```
/unsubscribe <topic>  # From one topic
/unsubscribe-all      # From all topics
```

#### Broadcasting messages

To make a broadcast to a topic, a POST request must be made to `/broadcast`,
which includes the topic, message and token

```bash
curl \
    -XPOST \
    -H "Content-Type: application/json" \
    -d '{"topic": "<topic>", "message": "<message>", "token": "<token>"}' \
    localhost:8080/broadcast
```

### Auth token

Notiflux uses an EC256 public/private key pair JWT for authentication. Notiflux
is deployed with the public key, while the broadcaster has the private key as a
secret.

When clients connect, they need to provide a JWT with the topic and scope,
signed by the private key. This token should be provided be the broadcasting
source.

To make a broadcast, the broadcaster makes a POST request with a JWT as well,
using the broadcast scope.

### JWT structure

The JWT token needs to have the following payload:

```js
{
    "sub": "notiflux",               // Can be any value
    "exp": 123,                      // Expiry just needs to be valid for the broadcast
                                     // or subscribe event, so just few seconds is enough
    "topic": "topic",                // The topic to validate against
    "scope": "subscribe|broadcast",  // Needs to be either 'subscribe' for clients
                                     // or 'broadcast' for broadcaster
}
```

See ./scripts folder for examples in Python
