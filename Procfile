# Railway / Fly-style process definition.
# Railway injects a `PORT` env var which the signal server already honors
# (`--port`, then `PORT`, then 5876 default). HEALTH_PORT must be set so the
# plain-HTTP health endpoint is reachable by the orchestrator.
web: emudesk-signal --bind 0.0.0.0