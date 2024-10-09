auth = Brug venligst følgende link til at autentificere til din Wikimedia-konto: [Authenticate]({$url})

auth_exists_in_server = Du er allerede autentificeret til denne server. Ingen grund til at godkende igen.

auth_to_server = Du er allerede identificeret som [{$name}](<{$url}>). Vil du godkende dette til serveren?
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
whois_edits = Redigeringer: {$edits}
whois_groups = Grupper: {$groupslist}
whois_overflow = Anført kun op til 10. Klik på deres navn øverst for at se alle oplysninger.
whois_no_block_reason = <!-- No reason given -->

# If you need a different date format other than YYYY-MM-DD, let me know.
whois = Discord: {$mention}
    Registreret: {$registration}
    Hjem: {$home}
    {$global_groups}Samlet antal redigeringer: {$edits}

# These are currently unused for now. Please still translate this if possible!
server_auth_success = Succes! Autorisationsoplysninger sendt til botten :)
server_auth_expired = Autorisationsanmodningen var udløbet eller ugyldig.
