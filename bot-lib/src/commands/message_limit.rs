use crate::{
    data::PoiseContext,
    utils::{GetRelativeTimestamp, SendReplyEphemeral},
};
use color_eyre::eyre::{ContextCompat, Result};
use human_repr::HumanDuration;
use humantime::parse_duration;
use poise::{
    CreateReply,
    serenity_prelude::{EditMember, Mentionable, User},
};
use rand::prelude::*;
use std::time::Duration;
#[poise::command(slash_command)]
pub async fn message_limit(ctx: PoiseContext<'_>,
    #[description = "Number of daily messages"] limit: u32,
) -> Result<()>{

}