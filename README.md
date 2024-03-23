# notiflux

Broadcast messages to connected WebSocket clients by sending a POST request

## How does it work?

Client connect via WebSocket and 'join' channels. A broadcasting source will
make POST requests to the broadcast endpoint, with a message and a channel,
which will then broadcast the message to all connected clients.

### Auth

Notiflux uses an EC256 public/private key pair JWT for authentication. Notiflux
is deployed with the public key, while the broadcaster has the private key as a
secret.

When clients connect, they need to provide a JWT with the channel and scope,
signed by the private key. This token should be provided be the broadcasting
source.

To make a broadcast, the broadcaster makes a POST request with a JWT as well,
using the broadcast scope.

### JWT structure

The JWT token needs to have the following payload:

```js
{
    "sub": "notiflux",          // Can be any value
    "exp": 123,                 // Expiry just needs to be valid for the broadcast
                                // or join event, so just few seconds is enough
    "channel": "channel",       // The channel to validate against
    "scope": "join|broadcast",  // Needs to be either 'join' for clients 
                                // or 'broadcast' for broadcaster
}
```

See ./scripts folder for examples in Python
