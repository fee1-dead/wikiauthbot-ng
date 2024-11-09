auth = 请使用以下的链接来验证您的维基媒体账户: [验证]({$url})

auth_exists_in_server = 您在本服务器已经验证过了，无需重复验证。

auth_to_server = 您被识别为[{$name}](<{$url}>)。您希望绑定这一账户至本服务器吗？
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
whois_edits = 编辑数: {$edits}
whois_groups = 用户组: {$groupslist}
whois_overflow = 最多显示10条最大记录。欲获取全部信息，请点击顶端的用户名。
whois_no_block_reason = <!-- 未给出理由 -->

# If you need a different date format other than YYYY-MM-DD, let me know.
whois = Discord用户名: {$mention}
    注册于: {$registration}
    家维基: {$home}
    {$global_groups}全域编辑数: {$edits}

# These are currently unused for now. Please still translate this if possible!
server_auth_success = 成功！验证信息已发送给机器人。
server_auth_expired = 验证请求已过期或无效。