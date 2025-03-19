auth = Użyj następującego linku, aby zweryfikować swoje konto Wikimedia: [Przejdź do weryfikacji]({$url})

auth_exists_in_server = Już jesteś uwierzytelniony(-a) na tym serwerze. Nie musisz powtarzać tej czynności.

auth_to_server = Uwierzytelniłeś(-aś) się już jako [{$name}](<{$url}>). Czy chcesz korzystać z tego konta również na niniejszym serwerze?
yes = Tak
no = Nie

auth_footer = Ten link będzie ważny przez 5 minut.

authreq_canceled = Anulowano uwierzytelnianie.
authreq_expired = Prośba o uwierzytelnienie wygasła.

authlog = {$mention} uwierzytelniony(-a) jako [{$username}](<{$user_link}>) (id {$wmf_id})

# The entry in the audit log recording why the authenticated role was granted.
auditlog_successful_auth = uwierzytelniony(-a) jako użytkownik Wikimedia {$wmf_id}

authreq_successful = Uwierzytelnianie się powiodło.

bot = WikiAuthBot

whois_no_user_found = Nie znaleziono użytkownika. Nie ma go na tym serwerze lub nie jest uwierzytelniony.

revwhois_fail = Nie udało się pobrać informacji dla tego użytkownika. Upewnij się, że podałeś(-aś) poprawną nazwę użytkownika.

revwhois_no_auth = [{$name}](<{$user_link}>) nie uwierzytelnił(-a) się na tym serwerze.

revwhois_one = Konto Wikimedia [{$name}](<{$user_link}>) jest powiązane z {$mention}

# note: no space between colon and variable.
revwhois_multiple = Konto Wikimedia [{$name}](<{$user_link}>) jest powiązane z następującymi użytkownikami:{$mentions}

user_link = https://pl.wikipedia.org/w/index.php?title=Special%3ACentralAuth/{$normalized_name}

welcome_has_auth = Witamy {$mention}! Jesteś już uwierzytelniony(-a) jako [{$name}](<{$user_link}>) i nie musisz potwierdzać swojego konta ponownie.

welcome_has_auth_failed = Witamy {$mention}! Jesteś już uwierzytelniony(-a) i nie musisz potwierdzać swojego konta ponownie (wystąpił błąd przy pobieraniu nazwy Twojego konta Wikimedia).

welcome = Witamy {$mention}! Jeśli chcesz uwierzytelnić (publicznie powiązać) swoje konto Wikimedia, wpisz lub kliknij </auth:1241068923730919464>

whois_global_groups = Grupy globalne: {$groupslist}

whois_blocked = **ZABLOKOWANO**
whois_locked = **ZABLOKOWANO GLOBALNIE**
whois_pblocked = partially blocked
whois_edits = Edycji: {$edits}
whois_groups = Grupy: {$groupslist}
whois_overflow = Tylko 10 najważniejszych jest wyświetlane. Kliknij nazwę użytkownika u góry, aby zobaczyć wszystkie.
whois_no_block_reason = <!-- Nie podano powodu -->

# If you need a different date format other than YYYY-MM-DD, let me know.
whois = Discord: {$mention}
    Data rejestracji: {$registration}
    Domowa wiki: {$home}
    {$global_groups}Wszystkich edycji: {$edits}

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

server_auth_success = Sukces! Informacje uwierzytelniające zostały przekazane do bota :)

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
