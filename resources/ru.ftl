auth = Чтобы аутентифицировать вашу учётную запись Викимедиа, пожалуйста, перейдите по следующей ссылке: [аутентифицироваться]({$url})

auth_exists_in_server = Вы уже аутентифицированы на этом сервере. Повторная аутентификация не требуется.

auth_to_server = Вы идентифицированы как [{$name}](<{$url}>). Хотите аутентифицироваться на сервере так?
yes = Да
no = Нет

auth_footer = Ссылка будет действительна 5 минут.

authreq_canceled = Аутентификация прервана.
authreq_expired = Аутентификация истекла.

authlog = {$mention} аутентифицирован(а) как [User:{$username}](<{$user_link}>) (id {$wmf_id})

# The entry in the audit log recording why the authenticated role was granted.
auditlog_successful_auth = аутентифицирован(а) как пользователь Викимедиа {$wmf_id}

authreq_successful = Аутентификация прошла успешно.

bot = WikiAuthBot

whois_no_user_found = Участник не найден. Либо участника нет на сервере, либо он не аутентифицирован.

revwhois_fail = Не удалось получить информацию о данном участнике. Пожалуйста, убедитесь, что вы указали корректный ник.

revwhois_no_auth = [{$name}](<{$user_link}>) не аутентифицирован(а) на сервере.

revwhois_one = [{$name}](<{$user_link}>) аутентифицирован(а) как {$mention}

# note: no space between colon and variable.
revwhois_multiple = [{$name}](<{$user_link}>) аутентифицирован(а) следующими учётными записями:{$mentions}

user_link = https://ru.wikipedia.org/w/index.php?title=Служебная%3ACentralAuth/{$normalized_name}

welcome_has_auth = Добро пожаловать, {$mention}! Вы уже аутентифицированы как [{$name}](<{$user_link}>), повторная аутентификация не требуется.

welcome_has_auth_failed = Добро пожаловать, {$mention}! Вы уже аутентифицированы как (ошибка при получении информации!), повторная аутентификация не требуется.

welcome = Добро пожаловать, {$mention}! Если вы хотите аутентифицировать (подтвердить) вашу учётную запись Викимедиа, пожалуйста, введите </auth:1241068923730919464>

whois_global_groups = Глобальные группы: {$groupslist}

whois_blocked = **ЗАБЛОКИРОВАН(А)**
whois_locked = **ЗАМОРОЖЕН(А)**
whois_pblocked = partially blocked
whois_edits = Правок: {$edits}
whois_groups = Группы: {$groupslist}
whois_overflow = Перечислено до 10 проектов. Для получения полной информации нажмите на ник вверху.
whois_no_block_reason = <!-- Причина не указана -->

# If you need a different date format other than YYYY-MM-DD, let me know.
whois = Discord: {$mention}
    Регистрация: {$registration}
    Домашний проект: {$home}
    {$global_groups}Всего правок: {$edits}

cancel = cancel

deauth = Are you sure you want to remove your authentication from this server?
deauth_canceled = Deauthentication canceled.
deauth_expired = Deauthentication expired.
deauth_not_found = You are currently not authenticated to this server. Run this command in a server where you are authenticated.
deauth_done = Successfully removed authentication data.
deauth_more = You are currently authenticated to {$num_servers_authed} servers. Would you like to remove data from only this server, or from all {$num_servers_authed} servers?
deauth_more_single = Delete from only this server
deauth_more_single_done = Successfully removed authentication data from this server.
deauth_more_multi = Delete from all servers I am in
deauth_more_multi_done = Successfully removed authentication data from {$num_servers_authed} servers.
deauth_log = {$mention} has deauthenticated from this server.
deauth_audit_log = Deauthenticated

auth_failed_blocked = Authentication failed: You have been blocked from one or more Wikimedia projects, which prevents authentication to this server. Contact the server admins for help.
auth_failed_error = Authentication failed: An internal error has occured. Please contact beef.w on Discord for reporting bugs.
removed_blocked_user_roles = Removed role from blocked user
adding_managed_role = Adding bot-managed role for meeting criteria
removing_managed_role = Removing bot-managed role as user no longer meets criteria

server_auth_success = Успешно! Информация об авторизации отправлена боту. :)

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
