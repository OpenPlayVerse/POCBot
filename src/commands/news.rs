use poise::serenity_prelude::{Member, RoleId};

use crate::{Context,Error};

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
    member.roles.contains(&RoleId(role_id))
}

/// lets you subscribe to updates
#[poise::command(prefix_command, slash_command)]
pub async fn subscribe(ctx: Context<'_>, roles: Roles) -> Result<(), Error> {
    let role = match roles {
        Roles::ServerUpdates => "805078371725869066",
        Roles::TechnicalUpdates => "944371601560969326",
    };

    let mut member = match ctx.author_member().await {
        Some(m) => m,
        None => return Ok(()), // member not found, do nothing
    };

    let role = role.parse::<u64>().unwrap();
    if has_role(&ctx, &member, role) {
        ctx.say(format!("You already have the role <@&{}>!", role)).await?;
    } else {
        let member_mut = member.to_mut();
        member_mut.add_role(&ctx, &RoleId(role)).await?;
        ctx.say(format!("You now have the role <@&{}>!", role)).await?;
    }

    Ok(())
}


/// Lets you unsubscribe from updates
#[poise::command(prefix_command, slash_command)]
pub async fn unsubscribe(ctx: Context<'_>, roles: Roles) -> Result<(), Error> {
    let role = match roles {
        Roles::ServerUpdates => "805078371725869066",
        Roles::TechnicalUpdates => "944371601560969326",
    };

    let mut member = match ctx.author_member().await {
        Some(m) => m,
        None => return Ok(()), // member not found, do nothing
    };

    let role = role.parse::<u64>().unwrap();
    if has_role(&ctx, &member, role) {
		let member_mut = member.to_mut();
        member_mut.remove_role(&ctx, &RoleId(role)).await?;
        ctx.say(format!("You no longer have the role <@&{}>!", role)).await?;
    } else {
		ctx.say(format!("You don't have the role <@&{}>!", role)).await?;
    }

    Ok(())
}
