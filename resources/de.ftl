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

welcome = Wil{$mention}, willkommen auf dem inoffiziellen Discord-Server der deutschsprachigen Wikipedia-Community! Wenn Sie Ihr Wikimedia-Konto verifizieren möchten (empfohlen), geben Sie bitte `</auth:1241068923730919464>` ein oder klicken Sie darauf.

whois_global_groups = Globale Gruppen: {$groupslist}

whois_blocked = **GESPERRT**
whois_locked = **GELOCKT (GLOBAL)**
whois_edits = Bearbeitungen: {$edits}
whois_groups = Globale Gruppen: {$groupslist}
whois_overflow = Nur zehn sind gelistet. Klicke oben auf den Benutzernamen, um alle Informationen zu sehen.
whois_no_block_reason = <!-- No reason given -->

# If you need a different date format other than YYYY-MM-DD, let me know.
whois = Discord: {$mention}
    Registiert: {$registration}
    Heimatwiki: {$home}
    {$global_groups}Gesamte Bearbeitungen: {$edits}

# These are currently unused for now. Please still translate this if possible!
server_auth_success = Die Bestätigungsdaten wurden erfolgreich an den Bot gesendet! :)
server_auth_expired = Der Bestätigungslink ist abgelaufen oder ungültig.
