# Discord Channel Management Bot

A Discord bot written in Rust that allows creating and managing text channels with custom permissions.

## Features

- **Channel Creation**: Users with special role can create channels in a dedicated category
- **Channel Management**: Rename, add descriptions, and delete created channels
- **Channel List**: Maintains an updated list of all managed channels
- **Operation Logging**: Records all operations in a dedicated log channel
- **Persistent State**: Reads channel data directly from Discord for reliability

## Prerequisites

- Rust (version 1.70 or higher)
- A Discord bot with valid token
- A Discord server with the following configured:
  - A category for channels
  - A channel for logs
  - A channel for the channel list
  - A special role for creating channels
  - A manager role for administration

## Installation

### 1. Create the Bot on Discord

1. Go to [Discord Developer Portal](https://discord.com/developers/applications)
2. Click "New Application"
3. Navigate to the "Bot" section and click "Add Bot"
4. Copy the bot token
5. Enable the following **Privileged Gateway Intents**:
   - Server Members Intent
   - Message Content Intent

### 2. Invite the Bot to Your Server

Use this URL (replace `CLIENT_ID` with your application ID):
```
https://discord.com/api/oauth2/authorize?client_id=YOUR_CLIENT_ID&permissions=8&scope=bot%20applications.commands
```

### 3. Configure the Project

1. Clone or create the project structure:
```bash
mkdir discord-channel-bot
cd discord-channel-bot
```

2. Create the directory structure:
```bash
mkdir -p src/commands
```

3. Copy all provided files into the correct structure

4. Edit the `config.toml` file with your IDs:
```toml
token = "YOUR_BOT_TOKEN"
category_id = 123456789012345678
log_channel_id = 123456789012345678
list_channel_id = 123456789012345678
special_role_id = 123456789012345678
manager_role_id = 123456789012345678
```

**How to get IDs:**
- Enable Developer Mode on Discord (Settings → Advanced)
- Right-click on channels/roles/categories and select "Copy ID"

## Building and Running

```bash
# Build the project
cargo build --release

# Run the bot
cargo run --release
```

## Available Commands

### `/create <name>`
Creates a new channel in the dedicated category.
- **Required Permission**: Special role
- **Example**: `/create project-alpha`

### `/delete`
Deletes the current channel.
- **Required Permission**: Special role
- **Usage**: Execute in the channel to be deleted

### `/rename <new_name>`
Renames the current channel.
- **Required Permission**: Special role
- **Example**: `/rename project-beta`

### `/description <text>`
Sets or removes the channel description.
- **Required Permission**: Special role
- **Example**: `/description Channel for discussing the project`
- **Remove**: `/description` (without parameters)

### `/updatelist`
Completely rebuilds the channel list from the category.
- **Required Permission**: Manager role
- **Usage**: Run this command if the list is out of sync

## Permissions for Created Channels

| Role/User | Permissions |
|-----------|-------------|
| Creator | View ✅, Send Messages ✅, Manage Channel ✅ |
| @everyone | View ✅, Create Threads ✅, Send Messages ❌ |
| Administrators | All permissions ✅ |
| Bot | All permissions ✅ |

## Project Structure

```
discord-channel-bot/
├── Cargo.toml
├── config.toml
└── src/
    ├── main.rs
    ├── config.rs
    ├── embed.rs
    ├── utils.rs
    └── commands/
        ├── mod.rs
        ├── create.rs
        ├── delete.rs
        ├── rename.rs
        ├── description.rs
        └── updatelist.rs
```

## Configuration File

### `config.toml`
Contains bot configuration (token, channel and role IDs).

Example:
```toml
token = "YOUR_BOT_TOKEN_HERE"
category_id = 123456789012345678
log_channel_id = 123456789012345678
list_channel_id = 123456789012345678
special_role_id = 123456789012345678
manager_role_id = 123456789012345678
```

## Troubleshooting

### Bot Won't Connect
- Verify the token is correct in `config.toml`
- Check that intents are enabled in the Developer Portal

### Commands Don't Appear
- Wait a few minutes after startup (Discord may take time)
- Restart the bot
- Verify bot permissions in the server

### Permission Errors
- Ensure the bot has "Administrator" permission in the server
- Verify role IDs are correct

### Channel Not Created
- Verify the category exists
- Check that the bot has permissions in the category

## Main Dependencies

- `serenity`: Discord bot framework
- `tokio`: Async runtime
- `serde`: Serialization/deserialization
- `anyhow`: Error handling

## License

This project is provided as an educational example. Feel free to use it for your purposes.

## Contributing

Feel free to modify and improve the bot according to your needs!