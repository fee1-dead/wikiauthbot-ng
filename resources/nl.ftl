-privacy_info = Bekijk de [privacyverklaring](<https://wikiauthbot-ng.toolforge.org/ps>) van WikiAuthBot2 voor meer informatie over hoe we je gegevens gebruiken. (Engels)

auth = Gebruik de volgende link om je te authenticeren bij je Wikimedia-account: [Authenticeren]({$url})

    {-privacy_info}

auth_exists_in_server = Je bent al geauthenticeerd op deze server. Je hoeft dit niet opnieuw te doen.

auth_to_server = Je bent al geïdentificeerd als [{$name}](<{$url}>). Wil je dit authenticeren op deze server?

    {-privacy_info}

yes = Ja
no = Nee

auth_footer = Deze link is 5 minuten geldig.

authreq_canceled = Authenticatie geannuleerd.
authreq_expired = Authenticatie verlopen.

authlog = {$mention} is geauthenticeerd als [Gebruiker:{$username}](<{$user_link}>) (id {$wmf_id})

# The entry in the audit log recording why the authenticated role was granted.
auditlog_successful_auth = geauthenticeerd als wikimedia-gebruiker {$wmf_id}

authreq_successful = Authenticatie geslaagd.

bot = WikiAuthBot

whois_no_user_found = Geen gebruiker gevonden. De gebruiker bevindt zich niet op deze server of is niet geauthenticeerd.

revwhois_fail = Kon geen informatie ophalen voor de opgegeven gebruiker. Controleer of je de juiste gebruikersnaam hebt ingevoerd.

revwhois_no_auth = [{$name}](<{$user_link}>) is niet geauthenticeerd op deze server.

revwhois_one = [{$name}](<{$user_link}>) is geauthenticeerd als {$mention}

# note: no space between colon and variable.
revwhois_multiple = [{$name}](<{$user_link}>) is geauthenticeerd op de volgende accounts:{$mentions}

user_link = https://nl.wikipedia.org/w/index.php?title=Speciaal%3ACentralAuth/{$normalized_name}

welcome_has_auth = Welkom {$mention}! Je bent al geauthenticeerd als [{$name}](<{$user_link}>), dus je hoeft dit niet opnieuw te doen.

welcome_has_auth_failed = Welkom {$mention}! Je bent al geauthenticeerd (fout bij het ophalen van info!), dus je hoeft dit niet opnieuw te doen.

welcome = Welkom {$mention}! Als je je Wikimedia-account wilt authenticeren (openbaar koppelen), typ of klik dan op </auth:1241068923730919464>

whois_global_groups = Globale groepen: {$groupslist}

whois_blocked = **GEBLOKKEERD**
whois_locked = **GELOCKED**
whois_pblocked = gedeeltelijk geblokkeerd
whois_edits = Bewerkingen: {$edits}
whois_groups = Groepen: {$groupslist}
whois_overflow = Er worden er maximaal 10 getoond. Klik bovenaan op hun naam om alle info te zien.
whois_no_block_reason = <!-- Geen reden opgegeven -->

# If you need a different date format other than YYYY-MM-DD, let me know.
whois = Discord: {$mention}
    Geregistreerd: {$registration}
    Thuisproject: {$home}
    {$global_groups}Totaal aantal bewerkingen: {$edits}

cancel = annuleren

deauth = Weet je zeker dat je je authenticatie van deze server wilt verwijderen?
deauth_canceled = Deauthenticatie geannuleerd.
deauth_expired = Deauthenticatie verlopen.
deauth_not_found = Je bent momenteel niet geauthenticeerd op deze server. Voer dit commando uit op een server waar je wel geauthenticeerd bent.
deauth_done = Authenticatiegegevens succesvol verwijderd.
deauth_more = Je bent momenteel geauthenticeerd op {$num_servers_authed} servers. Wil je de gegevens van alleen deze server verwijderen, of van alle {$num_servers_authed} servers?
deauth_more_single = Alleen van deze server verwijderen
deauth_more_single_done = Authenticatiegegevens succesvol verwijderd van deze server.
deauth_more_multi = Verwijderen van alle servers waar ik lid van ben
deauth_more_multi_done = Authenticatiegegevens succesvol verwijderd van {$num_servers_authed} servers.
deauth_log = {$mention} heeft de authenticatie van deze server verwijderd.
deauth_audit_log = Deauthenticatie uitgevoerd

auth_failed_blocked = Authenticatie mislukt: Je bent geblokkeerd op een of meer Wikimedia-projecten, wat authenticatie op deze server verhindert. Neem contact op met de serverbeheerders voor hulp.
auth_failed_error = Authenticatie mislukt: Er is een interne fout opgetreden. Neem contact op met beef.w op Discord om bugs te rapporteren.
removed_blocked_user_roles = Rol verwijderd van geblokkeerde gebruiker
adding_managed_role = Door bot beheerde rol toegevoegd (voldoet aan criteria)
removing_managed_role = Door bot beheerde rol verwijderd (voldoet niet langer aan criteria)

server_auth_success = Succes! De autorisatie-informatie is naar de bot verzonden :)

cmd_whois = whois
cmd_whois_desc = Controleer de accountgegevens van een geauthenticeerd lid
cmd_whois_user = gebruiker
cmd_whois_user_desc = De te controleren gebruiker, laat leeg voor jezelf
cmd_whois_menu = Wie is dit?
cmd_auth = auth
cmd_auth_desc = Authenticeren bij je Wikimedia-account
cmd_revwhois = revwhois
cmd_revwhois_desc = Toon de Discord-accounts die gekoppeld zijn aan een Wikimedia-account
cmd_revwhois_user = gebruiker
cmd_revwhois_user_desc = Naam van de Wikimedia-gebruiker
cmd_deauth = deauth
cmd_deauth_desc = Deauthenticeer of verwijder je gegevens uit de bot
