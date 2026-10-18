# Coin Rush: rules for Baseplate

Baseplate reads this file at the start of every session. Keep it short and specific.

## Code
- Server code goes in `ServerScriptService`, client code in `StarterPlayerScripts`. No frameworks.
- Shared modules live in `ReplicatedStorage/Shared`. Remotes live in `ReplicatedStorage/Remotes`, one RemoteEvent per action.
- Use `--!strict` and type every public function.
- Never trust the client with coins, damage or purchases.

## Building
- Parts are anchored unless they need physics.
- Lobby colours: off-white walls, light grey trim, one blue accent.
- Group every build in a Model named after what it is (`Lobby`, `Shop`, `Arena`).

## Playtests
- Walk the main path after every turn that touches gameplay.
- A turn isn't done while Output shows errors or warnings from our scripts.

## Never
- Don't publish. I publish by hand.
- Don't touch `ServerStorage/Legacy`.
