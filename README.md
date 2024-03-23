# notiflux

Broadcast messages to connected WebSocket clients by sending a POST request

## TODO

- [x] Ability to subscribe to channels
- [x] Auth
- [ ] Tests
- [ ] CI/CD

And probably a whole lot more 


## How does it work?

Notiflux is deployed with a EC256 public key, to validate tokens from the
broadcast source. Each client connects to the service and issues a `/join
<channel> <token>` command. The client should get the token from the
broadaster, with the right scope. The scope is validated against the channel,
and if valid, lets the client join that broadcast channel

The broadcaster then makes POST requests to `/broadcast` with a channel,
message and token, which is validated as well to come from the broadcaster,
which has the private key. If the token is valid, the broadcast is sent to
all connected clients.

## JWT Token
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
