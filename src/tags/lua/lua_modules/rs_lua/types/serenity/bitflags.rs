use serenity::model::permissions::Permissions;
use tagbot_macros::{tb_names, TBBitflag};

#[tb_names("get_permission_names")]
#[derive(TBBitflag)]
pub struct TBPermissions(pub Permissions);
