-privacy_info = See WikiAuthBot2's [privacy statement](<https://wikiauthbot-ng.toolforge.org/ps>) to learn more about how we use your information.

auth = Каб праверыць сапраўднасць вашага ўліковага запіса Wikimedia, выкарыстайце наступную спасылку: [Authenticate]({$url})

    {-privacy_info}

auth_exists_in_server = Вы ўжо прайшлі праверку сапраўднасці. Няма патрэбы рабіць яе зноў.

auth_to_server = Вы ўжо ідэнтыфікаваныя як [{$name}](<{$url}>). Ці хочаце вы прайсці праверку сапраўднасці для сервера?

    {-privacy_info}

yes = Так
no = Не

auth_footer = Гэтая спасылка будзе дзейнічаць 5 мінут.

authreq_canceled = Праверка сапраўднасці скасавана.
authreq_expired = Праверка сапраўднасці пратэрмінавана.

authlog = {$mention} прайшоў(-ла) праверку як [User:{$username}](<{$user_link}>) (id {$wmf_id})

# The entry in the audit log recording why the authenticated role was granted.
auditlog_successful_auth = аўтэнтыфікаваны(-ая) як удзельнікі wikimedia {$wmf_id}

authreq_successful = Паспяховая праверка сапраўднасці.

bot = WikiAuthBot

whois_no_user_found = Такога ўдзельніка не знойдзена. Гэтага ўдзельніка або няма на серверы, або ён не прайшоў праверку сапраўднасці.

revwhois_fail = Не ўдалося атрымаць звесткі пра дадзенага ўдзельніка. Пераканайцеся, што вы падалі правільнае імя.

revwhois_no_auth = [{$name}](<{$user_link}>) не прайшоў(-ла) праверку сапраўднасці на серверы.

revwhois_one = Для [{$name}](<{$user_link}>) праверку прайшоў(-ла) {$mention}

# note: no space between colon and variable.
revwhois_multiple = Для [{$name}](<{$user_link}>) праверку сапраўднасці прайшлі наступныя ўліковыя запісы:{$mentions}

user_link = https://be.wikipedia.org/w/index.php?title=Special%3ACentralAuth/{$normalized_name}

welcome_has_auth = Вітаем, {$mention}! Вы ўжо прайшлі праверку сапраўднасці для [{$name}](<{$user_link}>), таму вам не трэба праходзіць яе зноў.

welcome_has_auth_failed = Вітаем, {$mention}! Вы ўжо прайшлі праверку сапраўднасці (памылка падчас атрымання звестак!), таму вам не трэба праходзіць яе зноў.

welcome = Вітаем, {$mention}! Калі вы хочаце прайсці праверку сапраўднасці (публічна прывязаць) свой уліковы запіс Wikimedia, увядзіце або насніце на </auth:1241068923730919464>

whois_global_groups = Глабальныя групы: {$groupslist}

whois_blocked = **ЗАБЛАКАВАНА**
whois_locked = **ЗАМКНУТА**
whois_pblocked = часткова заблакавана
whois_edits = Праўкі: {$edits}
whois_groups = Групы: {$groupslist}
whois_overflow = Паказваецца толькі да 10 удзельнікаў. Націсніце на іх імёны зверху каб пабачыць поўныя звесткі.
whois_no_block_reason = <!-- Прычына не пададзена -->

# If you need a different date format other than YYYY-MM-DD, let me know.
whois = Discord: {$mention}
    Рэгістрацыя: {$registration}
    Дамашняя вікі: {$home}
    {$global_groups}Усяго правак: {$edits}

cancel = скасаваць

deauth = Вы ўпэўненыя, што хочаце выдаліць свае даныя праверкі сапраўднасці з гэтага сервера?
deauth_canceled = Выдаленне даных скасавана.
deauth_expired = Выдаленне даных пратэрмінавана.
deauth_not_found = Цяпер вы не маеце праверанага ўліковага запіса Wikimedia на гэтым серверы. Запусціце гэтую каманду на тым серверы, дзе вы прайшлі праверку сапраўднасці.
deauth_done = Паспяховае выдаленне даных праверкі.
deauth_more = Цяпер вы правераныя на такой колькасці сервераў: {$num_servers_authed}. Вы хочаце выдаліць даныя толькі з гэтага сервера ці з усіх сервераў ({$num_servers_authed} шт.)?
deauth_more_single = Выдаліць толькі на гэтым серверы
deauth_more_single_done = Даныя праверкі сапраўднасці на гэтым серверы выдалены.
deauth_more_multi = Выдаліць з усіх сервераў, дзе я знаходжуся
deauth_more_multi_done = Даныя праверкі сапраўднасці выдалены на ўсех серверах (${num_servers_authed} шт.).
deauth_log = {$mention} выдаліў(-ла) даныя праверкі на гэтым серверы.
deauth_audit_log = Даныя праверкі сапраўднасці выдалены

auth_failed_blocked = Не ўдалося праверыць сапраўднасць: Вы заблакаваныя на адным або некалькіх праектах Wikimedia, што не дазваляе прайсці праверку на гэтым серверы. Звяжыцеся да адміністратараў сервера па дапамогу.
auth_failed_error = Не ўдалося праверыць сапраўднасць: Адбылася ўнутраная памылка. Звярніцеся да beef.w у Discord каб паведаміць пра for reporting памылкі.
removed_blocked_user_roles = Выдаліць ролю з заблакаваных удзельнікаў
adding_managed_role = Adding bot-managed role for meeting criteria
removing_managed_role = Removing bot-managed role as user no longer meets criteria

server_auth_success = Звесткі пра праверку сапраўднасці паспяхова адпраўленыя боту! :)

cmd_whois = whois
cmd_whois_desc = Check account details for an authenticated member
cmd_whois_user = user
cmd_whois_user_desc = User to check, leave blank for yourself
cmd_whois_menu = Get whois
cmd_auth = auth
cmd_auth_desc = Authenticate to your Wikimedia account
cmd_revwhois = revwhois
cmd_revwhois_desc = List Discord accounts associated to a Wikimedia account
cmd_revwhois_user = user
cmd_revwhois_user_desc = Name of the Wikimedia user
cmd_deauth = deauth
cmd_deauth_desc = Deauthenticate or remove your data from the bot.