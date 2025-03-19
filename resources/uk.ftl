auth = Щоб автентифікувати свій обліковий запис Вікімедіа, перейдіть, будь ласка, за цим посиланням: [Authenticate]({$url})

auth_exists_in_server = Ви вже автентифіковані на цьому сервері. Немає потреби автентифікуватися ще раз.

auth_to_server = Вас уже ідентифіковано як [{$name}](<{$url}>). Бажаєте автентифікуватися так на сервері?
yes = Так
no = Ні

auth_footer = Це посилання буде дійсним упродовж 5 хвилин.

authreq_canceled = Автентифікацію скасовано.
authreq_expired = Термін дії автентифікації закінчився.

authlog = {$mention} автентифікувався/лася як [Користувач:{$username}](<{$user_link}>) (id {$wmf_id})

# The entry in the audit log recording why the authenticated role was granted.
auditlog_successful_auth = автентифікований/а як користувач Вікімедіа {$wmf_id}

authreq_successful = Успішно автентифіковано.

bot = WikiAuthBot

whois_no_user_found = Користувача не знайдено. Або користувача немає на цьому сервері, або його не автентифіковано.

revwhois_fail = Не вдалося отримати інформацію про вказаного користувача. Будь ласка, переконайтеся, що ви ввели правильне ім'я.

revwhois_no_auth = [{$name}](<{$user_link}>) не автентифіковано на цьому сервері.

revwhois_one = [{$name}](<{$user_link}>) автентифіковано як {$mention}

# note: no space between colon and variable.
revwhois_multiple = [{$name}](<{$user_link}>) автентифікований/а у таких обліковках:{$mentions}

user_link = https://uk.wikipedia.org/w/index.php?title=Special%3ACentralAuth/{$normalized_name}

welcome_has_auth = Привіт, {$mention}! Ви уже автентифіковані як [{$name}](<{$user_link}>), тому вам не потрібно автентифіковуватися ще раз.

welcome_has_auth_failed = Привіт, {$mention}! Ви уже автентифіковані як (помилка при отриманні інформації!), тому вам не потрібно автентифіковуватися ще раз.

welcome = Привіт, {$mention}! Якщо ви бажаєте автентифікувати (валідувати) свій обліковий запис Вікімедіа, введіть, будь ласка, </auth:1241068923730919464>

whois_global_groups = Глобальні групи: {$groupslist}

whois_blocked = **ЗАБЛОКОВАНО**
whois_locked = **ЗАКРИТО**
whois_pblocked = partially blocked
whois_edits = Редагування: {$edits}
whois_groups = Групи: {$groupslist}
whois_overflow = Наведено лише до 10 проєктів. Натисніть на ім'я угорі, щоб переглянути повну інформацію.
whois_no_block_reason = <!-- Причини не вказано -->

# If you need a different date format other than YYYY-MM-DD, let me know.
whois = Discord: {$mention}
    Зареєстровано: {$registration}
    Домашня вікі: {$home}
    {$global_groups}Разом редагувань: {$edits}

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

server_auth_success = Авторизаційну інформацію успішно надіслано ботові :)

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
