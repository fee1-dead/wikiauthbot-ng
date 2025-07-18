-privacy_info = See WikiAuthBot2's [privacy statement](<https://wikiauthbot-ng.toolforge.org/ps>) to learn more about how we use your information.

auth = Brug venligst følgende link til at autentificere til din Wikimedia-konto: [Authenticate]({$url})

    {-privacy_info}

auth_exists_in_server = Du er allerede autentificeret til denne server. Ingen grund til at godkende igen.

auth_to_server = Du er allerede identificeret som [{$name}](<{$url}>). Vil du godkende dette til serveren?

    {-privacy_info}

yes = Ja
no = Nej

auth_footer = Dette link vil være gyldigt i 5 minutter

authreq_canceled = Autentificering annulleret.
authreq_expired = Autentificering er udløbet.

authlog = {$mention} autentificeret som [Bruger:{$username}](<{$user_link}>) (id {$wmf_id})

# The entry in the audit log recording why the authenticated role was granted.
auditlog_successful_auth = autentificeret som wikimedia bruger {$wmf_id}

authreq_successful = Autentificering gennemført..

bot = WikiAuthBot

whois_no_user_found = Ingen bruger fundet. Enten er brugeren ikke på denne server eller er uautoriseret.

revwhois_fail = Kunne ikke hente oplysninger for den pågældende bruger. Sørg for, at du har angivet det korrekte brugernavn.

revwhois_no_auth = [{$name}](<{$user_link}>) har ikke autentificeret til denne server.

revwhois_one = [{$name}](<{$user_link}>) er autentificeret til {$mention}

# note: no space between colon and variable.
revwhois_multiple = [{$name}](<{$user_link}>) er autentificeret til følgende konti:{$mentions}

user_link = https://da.wikipedia.org/w/index.php?title=Special%3ACentralAuth/{$normalized_name}

welcome_has_auth = Velkommen {$mention}! Du har allerede autentificeret som [{$name}](<{$user_link}>), så du behøver ikke at godkende igen.

welcome_has_auth_failed = Velkommen {$mention}! Du har allerede autentificeret (fejl under forsøg på at hente oplysninger!), så du behøver ikke at godkende igen.

welcome = Velkommen {$mention}! Hvis du gerne vil autentificere (validere) din Wikimedia-konto, skal du indtaste </auth:1241068923730919464>

whois_global_groups = Globale grupper: {$groupslist}

whois_blocked = **BLOCKED**
whois_locked = **LOCKED**
whois_pblocked = partially blocked
whois_edits = Redigeringer: {$edits}
whois_groups = Grupper: {$groupslist}
whois_overflow = Anført kun op til 10. Klik på deres navn øverst for at se alle oplysninger.
whois_no_block_reason = <!-- No reason given -->

# If you need a different date format other than YYYY-MM-DD, let me know.
whois = Discord: {$mention}
    Registreret: {$registration}
    Hjem: {$home}
    {$global_groups}Samlet antal redigeringer: {$edits}

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

server_auth_success = Succes! Autorisationsoplysninger sendt til botten :)

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
