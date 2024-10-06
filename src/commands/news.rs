use crate::{Context, Error};
use anyhow::{Context as AnyhowContext, Result};
use log::info;
use poise::serenity_prelude::{Member, RoleId};

#[derive(poise::ChoiceParameter)]
pub enum Roles {
    ServerUpdates,
    TechnicalUpdates,
}

#[poise::command(prefix_command, slash_command, subcommands("subscribe", "unsubscribe"))]
pub async fn news(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Checks if a member has a specific role
pub fn has_role(member: &Member, role_id: u64) -> bool {
    member.roles.contains(&RoleId::new(role_id))
}

/// Retrieves the member object for the command author
async fn get_member(ctx: &Context<'_>) -> Result<Member, Error> {
    Ok(ctx
        .author_member()
        .await
        .context("Failed to retrieve member")
        .map(|member| member.into_owned())?)
}

/// Parses the role enum into a role ID
fn parse_role(roles: Roles) -> Result<u64, Error> {
    let role_str = match roles {
        Roles::ServerUpdates => "805078371725869066",
        Roles::TechnicalUpdates => "944371601560969326",
    };
    Ok(role_str.parse::<u64>().context("Failed to parse role ID")?)
}

/// Lets you subscribe to updates
#[poise::command(prefix_command, slash_command)]
pub async fn subscribe(ctx: Context<'_>, roles: Roles) -> Result<(), Error> {
    let role = parse_role(roles)?;
    let member = get_member(&ctx).await?;

    if has_role(&member, role) {
        ctx.say(format!("You already have the role <@&{}>!", role))
            .await?;
    } else {
        member.add_role(&ctx, RoleId::new(role)).await?;
        ctx.say(format!("You now have the role <@&{}>!", role))
            .await?;
        info!("Role <@&{}> added to user {}", role, ctx.author().name);
    }

    Ok(())
}

/// Lets you unsubscribe from updates
#[poise::command(prefix_command, slash_command)]
pub async fn unsubscribe(ctx: Context<'_>, roles: Roles) -> Result<(), Error> {
    let role = parse_role(roles)?;
    let member = get_member(&ctx).await?;

    if has_role(&member, role) {
        member.remove_role(&ctx, RoleId::new(role)).await?;
        ctx.say(format!("You no longer have the role <@&{}>!", role))
            .await?;
        info!("Role <@&{}> removed from user {}", role, ctx.author().name);
    } else {
        ctx.say(format!("You don't have the role <@&{}>!", role))
            .await?;
    }

    Ok(())
}
