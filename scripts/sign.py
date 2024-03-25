import jwt
import datetime as dt
import typer
import enum


class Scopes(enum.Enum):
    BROADCAST = "broadcast"
    SUBSCRIBE = "subscribe"


def main(
    private_key_path: str = typer.Option("private_key.pem"),
    verbose: bool = typer.Option(False),
    scope: Scopes = typer.Argument(),
    topic: str = typer.Argument(),
):
    private_key = open(private_key_path, "r").read()

    payload = {
        "sub": "notiflux",
        "exp": dt.datetime.now(dt.timezone.utc) + dt.timedelta(days=365*100),
        "topic": topic,
        "scope": scope.value,
    }
    if verbose:
        print(f"{payload=}")

    token = jwt.encode(payload, private_key, algorithm="ES256")
    print(token)


typer.run(main)
