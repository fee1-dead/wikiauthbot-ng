-privacy_info = 请参阅WikiAuthBot2的[隐私声明](<https://wikiauthbot-ng.toolforge.org/ps>)，以进一步了解我们如何使用您的信息。

auth = 请使用以下的链接来验证您的维基媒体账户: [验证]({$url})

    {-privacy_info}

auth_exists_in_server = 您在本服务器已经验证过了，无需重复验证。

auth_to_server = 您被识别为[{$name}](<{$url}>)。您希望绑定这一账户至本服务器吗？

    {-privacy_info}

yes = 是
no= 否

auth_footer = 此链接有效期为5分钟。

authreq_canceled = 验证取消。
authreq_expired = 验证已过期。

authlog = {$mention} 已验证为[User:{$username}](<{$user_link}>) (ID {$wmf_id})

# The entry in the audit log recording why the authenticated role was granted.
auditlog_successful_auth = 验证为维基媒体用户{$wmf_id}

authreq_successful = 验证成功。

bot = WikiAuthBot

whois_no_user_found = 未找到用户。用户可能并未加入本服务器，或未进行验证。

revwhois_fail = 无法获取给定用户的相关信息，请确保您提供了正确的用户名。

revwhois_no_auth = [{$name}](<{$user_link}>)没有在本服务器上验证。

revwhois_one = [{$name}](<{$user_link}>)已验证为 {$mention}

# note: no space between colon and variable.
revwhois_multiple = [{$name}](<{$user_link}>)已绑定了以下账户:{$mentions}

user_link = https://zh.wikipedia.org/w/index.php?title=Special%3ACentralAuth/{$normalized_name}

welcome_has_auth = 欢迎， {$mention}！您已成功验证为[{$name}](<{$user_link}>)，因此您不需要重复验证。

welcome_has_auth_failed = 欢迎， {$mention}！您已成功验证为（无法获取用户相关信息！），因此您不需要重复验证。

welcome = 欢迎， {$mention}！如果您希望验证（公开关联）您的维基媒体账户，请输入或点击 </auth:1241068923730919464>

whois_global_groups = 全域用户组: {$groupslist}

whois_blocked = **已封禁**
whois_locked = **已锁定**
whois_pblocked = 已部分封禁
whois_edits = 编辑数: {$edits}
whois_groups = 用户组: {$groupslist}
whois_overflow = 最多显示10条最大记录。欲获取全部信息，请点击顶端的用户名。
whois_no_block_reason = <!-- 未给出理由 -->

# If you need a different date format other than YYYY-MM-DD, let me know.
whois = Discord用户名: {$mention}
    注册于: {$registration}
    家维基: {$home}
    {$global_groups}全域编辑数: {$edits}

cancel = 取消

deauth = 您确定要在本服务器解除验证吗？
deauth_canceled = 解除验证取消。
deauth_expired = 解除验证已过期。
deauth_not_found = 您还没有在本服务器上验证。请在一个您已经验证过的服务器执行本指令。
deauth_done = 已成功解除验证信息。
deauth_more = 您当前已在{$num_servers_authed}个服务器上验证。您希望仅在本服务器上解除验证，还是在所有{$num_servers_authed}个服务器上均解除验证？
deauth_more_single = 仅在本服务器上解除验证
deauth_more_single_done = 已成功在本服务器上解除验证。
deauth_more_multi = 在所有我在的服务器上解除验证
deauth_more_multi_done = 已成功在{$num_servers_authed}个服务器上解除验证。
deauth_log = {$mention} 已在本服务器解除验证。
deauth_audit_log = 已解除验证

auth_failed_blocked = 验证失败：您已在一个或多个维基媒体项目被封禁，因此无法在本服务器完成验证。请联系服务器管理员以获取支持。
auth_failed_error = 验证失败：内部错误。请使用Discord联系 beef.w 提交错误报告。
removed_blocked_user_roles = 已移除被封禁用户的身份组
adding_managed_role = 正在为符合要求的用户添加由机器人管理的身份组
removing_managed_role = 正在为不再符合要求的用户移除由机器人管理的身份组

server_auth_success = 成功！验证信息已发送给机器人。

cmd_whois = whois
cmd_whois_desc = 获取成员的验证信息
cmd_whois_user = user
cmd_whois_user_desc = 需查询的用户，为空时查询自己
cmd_whois_menu = 获取验证信息
cmd_auth = auth
cmd_auth_desc = 验证你的维基媒体账户
cmd_revwhois = revwhois
cmd_revwhois_desc = 列出与特定维基媒体账户关联的 Discord 帐号
cmd_revwhois_user = user
cmd_revwhois_user_desc = 维基媒体账户名称
cmd_deauth = deauth
cmd_deauth_desc = 删除你的验证信息
