use serenity::model::permissions::Permissions;
use tagbot_macros::TBBitflag;

#[derive(TBBitflag)]
pub struct TBPermissions(pub Permissions);
