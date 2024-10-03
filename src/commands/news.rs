use poise::serenity_prelude::{Member, RoleId};
use crate::{Context, Error};
use log::{info, error};

#[derive(poise::ChoiceParameter)]
pub enum Roles {
    ServerUpdates,
    TechnicalUpdates,
}

#[poise::command(prefix_command, slash_command, subcommands("subscribe", "unsubscribe"))]
pub async fn news(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

pub fn has_role(_ctx: &Context<'_>, member: &Member, role_id: u64) -> bool {
    member.roles.contains(&RoleId::new(role_id))
}

async fn get_member(ctx: &Context<'_>) -> Result<Member, Error> {
    match ctx.author_member().await {
        Some(member) => Ok(member.into_owned()),
        None => {
            error!("Member not found");
            Err("Member not found".into())
        }
    }
}

fn parse_role(roles: Roles) -> Result<u64, Error> {
    let role_str = match roles {
        Roles::ServerUpdates => "805078371725869066",
        Roles::TechnicalUpdates => "944371601560969326",
    };
    role_str.parse::<u64>().map_err(|e| {
        error!("Failed to parse role ID: {}", e);
        "Failed to parse role ID".into()
    })
}

/// Lets you subscribe to updates
#[poise::command(prefix_command, slash_command)]
pub async fn subscribe(ctx: Context<'_>, roles: Roles) -> Result<(), Error> {
    let role = parse_role(roles)?;
    let member = get_member(&ctx).await?;

    if has_role(&ctx, &member, role) {
        ctx.say(format!("You already have the role <@&{}>!", role)).await?;
    } else {
        member.add_role(&ctx, RoleId::new(role)).await?;
        ctx.say(format!("You now have the role <@&{}>!", role)).await?;
        info!("Role <@&{}> added to user {}", role, ctx.author().name);
    }

    Ok(())
}

/// Lets you unsubscribe from updates
#[poise::command(prefix_command, slash_command)]
pub async fn unsubscribe(ctx: Context<'_>, roles: Roles) -> Result<(), Error> {
    let role = parse_role(roles)?;
    let member = get_member(&ctx).await?;

    if has_role(&ctx, &member, role) {
        member.remove_role(&ctx, RoleId::new(role)).await?;
        ctx.say(format!("You no longer have the role <@&{}>!", role)).await?;
        info!("Role <@&{}> removed from user {}", role, ctx.author().name);
    } else {
        ctx.say(format!("You don't have the role <@&{}>!", role)).await?;
    }

    Ok(())
}