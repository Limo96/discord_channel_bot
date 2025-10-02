use anyhow::Result;
use serenity::all::{
    ChannelType, Context, GuildChannel, Member, PermissionOverwrite,
    PermissionOverwriteType, Permissions, UserId,
};

use crate::{config::Config, embed::send_list_embed};

// Structure to hold channel data read from Discord
#[derive(Debug, Clone)]
pub struct ChannelInfo {
    pub channel_id: u64,
    pub name: String,
    pub creator_id: u64,
    pub description: Option<String>,
}

/// Get all managed channels from Discord category
pub async fn get_managed_channels(ctx: &Context, config: &Config) -> Result<Vec<ChannelInfo>> {
    let category_id = config.category_id();
    let log_channel_id = config.log_channel_id();
    let list_channel_id = config.list_channel_id();

    // We need to get guild_id from one of the channels
    let guild_id = category_id
        .to_channel(&ctx.http)
        .await?
        .guild()
        .map(|c| c.guild_id);

    if guild_id.is_none() {
        return Ok(Vec::new());
    }

    let guild_id = guild_id.unwrap();
    let channels = guild_id.channels(&ctx.http).await?;

    let mut managed_channels = Vec::new();

    for (channel_id, channel) in channels.iter() {
        // Only text channels in our category, excluding log and list channels
        if channel.kind == ChannelType::Text {
            if let Some(parent_id) = channel.parent_id {
                if parent_id == category_id
                    && *channel_id != log_channel_id
                    && *channel_id != list_channel_id
                {
                    // Detect creator from permission overwrites
                    let creator_id = channel
                        .permission_overwrites
                        .iter()
                        .find(|p| {
                            matches!(p.kind, PermissionOverwriteType::Member(_))
                                && p.allow.contains(Permissions::MANAGE_CHANNELS)
                        })
                        .and_then(|p| {
                            if let PermissionOverwriteType::Member(user_id) = p.kind {
                                Some(user_id.get())
                            } else {
                                None
                            }
                        })
                        .unwrap_or(0);

                    let description = channel.topic.clone();

                    managed_channels.push(ChannelInfo {
                        channel_id: channel_id.get(),
                        name: channel.name.clone(),
                        creator_id,
                        description,
                    });
                }
            }
        }
    }

    Ok(managed_channels)
}

/// Check if a channel is managed by the bot
pub async fn is_managed_channel(ctx: &Context, config: &Config, channel_id: u64) -> Result<bool> {
    let channels = get_managed_channels(ctx, config).await?;
    Ok(channels.iter().any(|c| c.channel_id == channel_id))
}

/// Update the channel list in the list channel
pub async fn update_channel_list(ctx: &Context, config: &Config) -> Result<()> {
    let list_channel = config.list_channel_id();

    // Delete all existing messages
    let messages = list_channel.messages(&ctx.http, Default::default()).await?;
    for message in messages {
        message.delete(&ctx.http).await?;
    }

    // Get all managed channels from Discord
    let mut channels = get_managed_channels(ctx, config).await?;

    // Sort by name
    channels.sort_by(|a, b| a.name.cmp(&b.name));

    // Send embed for each channel
    for channel_info in channels {
        send_list_embed(
            ctx,
            list_channel,
            serenity::all::ChannelId::new(channel_info.channel_id),
            channel_info.description.as_ref(),
            channel_info.creator_id,
        )
        .await?;
    }

    Ok(())
}

pub async fn has_special_role(member: &Member, config: &Config) -> bool {
    member.roles.contains(&config.special_role_id())
}

pub async fn has_manager_role(member: &Member, config: &Config) -> bool {
    member.roles.contains(&config.manager_role_id())
}

pub async fn create_channel_with_permissions(
    ctx: &Context,
    config: &Config,
    name: &str,
    creator_id: UserId,
    guild_id: serenity::model::id::GuildId,
) -> Result<GuildChannel> {
    let everyone_role = serenity::model::id::RoleId::new(guild_id.get());
    let category_id = config.category_id();

    let permissions = vec![
        PermissionOverwrite {
            allow: Permissions::VIEW_CHANNEL
                | Permissions::SEND_MESSAGES
                | Permissions::MANAGE_CHANNELS,
            deny: Permissions::empty(),
            kind: PermissionOverwriteType::Member(creator_id),
        },
        PermissionOverwrite {
            allow: Permissions::VIEW_CHANNEL | Permissions::CREATE_PUBLIC_THREADS,
            deny: Permissions::SEND_MESSAGES,
            kind: PermissionOverwriteType::Role(everyone_role),
        },
    ];

    let channel = guild_id
        .create_channel(
            &ctx.http,
            serenity::all::CreateChannel::new(name)
                .kind(serenity::all::ChannelType::Text)
                .category(category_id)
                .permissions(permissions),
        )
        .await?;

    Ok(channel)
}