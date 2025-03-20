auth = Bitte nutze den folgenden Link, um dein Wikimedia-Benutzerkonto verifizieren: [Authenticate]({$url})

auth_exists_in_server = Du bist auf diesem Server bereits bestätigt. Du musst dich daher nicht erneut bestätigen.

auth_to_server = Du bist bereits als bestätigt [{$name}](<{$url}>). Möchtest du dich auf diesem Server ebenfalls bestätigen?
yes = Ja
no = Nein

auth_footer = Dieser Link ist fünf Minuten lang gültig.

authreq_canceled = Bestätigungsprozess abgebrochen.
authreq_expired = Bestätigungslink abgelaufen.

authlog = {$mention} authenticated as [User:{$username}](<{$user_link}>) (id {$wmf_id})

# The entry in the audit log recording why the authenticated role was granted.
auditlog_successful_auth = bestätigt als {$wmf_id}

authreq_successful = Die Bestätigung war erfolgreich.

bot = WikiAuthBot

whois_no_user_found = Es tut mir leid, ich habe keinen Benutzer gefunden. Entweder ist der Benutzer nicht auf diesem Server oder unbestätigt.

revwhois_fail = Ich habe die Informationen über den Benutzer nicht verarbeiten können. Bitte stelle sicher, dass du den korrekten Benutzernamen eingegeben hast.

revwhois_no_auth = [{$name}](<{$user_link}>) hat sich auf diesem Server nicht bestätigt.

revwhois_one = [{$name}](<{$user_link}>) ist auf diesem Discord als {$mention} aktiv.

# note: no space between colon and variable.
revwhois_multiple = [{$name}](<{$user_link}>) ist auf diesem Discord mit folgenden Accounts: {$mentions}

user_link = https://de.wikipedia.org/w/index.php?title=Special%3ACentralAuth/{$normalized_name}

welcome_has_auth = Willkommen {$mention}! Du bist bereits als [{$name}](<{$user_link}>) bestätigt, du musst dich also nicht erneut bestätigen.

welcome_has_auth_failed = Willkommen {$mention}! Du bist bereits als (Fehler beim Laden der Information!) bestätigt, du musst dich also nicht erneut bestätigen.

welcome = Hallo {$mention}, willkommen auf dem inoffiziellen Discord-Server der deutschsprachigen Wikipedia-Community! Wenn du dein Wikimedia-Konto verifizieren möchtest (empfohlen), gib bitte `</auth:1241068923730919464>` ein oder klicke darauf.

whois_global_groups = Globale Gruppen: {$groupslist}

whois_blocked = **GESPERRT**
whois_locked = **GELOCKT (GLOBAL)**
whois_pblocked = partially blocked
whois_edits = Bearbeitungen: {$edits}
whois_groups = Globale Gruppen: {$groupslist}
whois_overflow = Nur zehn sind gelistet. Klicke oben auf den Benutzernamen, um alle Informationen zu sehen.
whois_no_block_reason = <!-- No reason given -->

# If you need a different date format other than YYYY-MM-DD, let me know.
whois = Discord: {$mention}
    Registiert: {$registration}
    Heimatwiki: {$home}
    {$global_groups}Gesamte Bearbeitungen: {$edits}

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

server_auth_success = Die Bestätigungsdaten wurden erfolgreich an den Bot gesendet! :)

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
