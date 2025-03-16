auth = Koristite sljedeći link za potvrdu svog računa na Wikimediji: [Authenticate]({$url})

auth_exists_in_server = Već ste se potvrdili na ovom serveru. Nije potrebna ponovna potvrda.

auth_to_server = Već ste potvrdili da je Vaš račun [{$name}](<{$url}>). Želite li to potvrditi na ovom serveru?
yes = Da
no = Ne

auth_footer = Link će biti važeći 5 minuta.

authreq_canceled = Potvrda otkazana.
authreq_expired = Isteklo vrijeme za potvrdu.

authlog = {$mention} je [User:{$username}](<{$user_link}>) (id {$wmf_id})

# The entry in the audit log recording why the authenticated role was granted.
auditlog_successful_auth = je {$wmf_id} na Wikimediji

authreq_successful = Potvrda uspješna.

bot = WikiAuthBot

whois_no_user_found = Ne mogu pronaći korisnika. Moguće je da nije na serveru ili nije potvrđen.

revwhois_fail = Ne mogu učitati podatke o korisniku. Provjerite jeste li korisničko ime ispravno.

revwhois_no_auth = [{$name}](<{$user_link}>) nije potvrdio/-la račun na ovom serveru.

revwhois_one = [{$name}](<{$user_link}>) je {$mention} na Discordu

# note: no space between colon and variable.
revwhois_multiple = [{$name}](<{$user_link}>) je povezan s više računa:{$mentions}

user_link = https://en.wikipedia.org/w/index.php?title=Special%3ACentralAuth/{$normalized_name}

welcome_has_auth = Dobro došli, {$mention}! Već ste potvrdili da je Vaš račun [{$name}](<{$user_link}>). Ponovna potvrda nije potrebna.

welcome_has_auth_failed = Dobro došli, {$mention}! Već ste potvrdili da je Vaš račun (error while trying to fetch info!). Ponovna potvrda nije potrebna.

welcome = Dobro došli, {$mention}! Da biste potvrdili (javno povezali) svoj račun na Wikimediji, upišite ili kliknite na </auth:1241068923730919464>

whois_global_groups = Globalne grupe: {$groupslist}

whois_blocked = **BLOKIRAN**
whois_locked = **ZAKLJUČAN**
whois_pblocked = djelimično blokiran
whois_edits = Ukupno izmjena: {$edits}
whois_groups = Grupe: {$groupslist}
whois_overflow = Prikazano je najviše 10 projekata. Za više informacija kliknite na ime na vrhu.
whois_no_block_reason = <!-- Nije naveden razlog -->

# If you need a different date format other than YYYY-MM-DD, let me know.
whois = Discord: {$mention}
    Datum registracije: {$registration}
    Projekt registracije: {$home}
    {$global_groups}Ukupno izmjena: {$edits}

cancel = otkaži

deauth = Jeste li sigurno da želite ukloniti potvrdu s ovog severa?
deauth_canceled = Uklanjanje potvrde otkazano.
deauth_expired = Isteklo vrijeme za uklanjanje potvrde.
deauth_not_found = Trenutno niste potvrđeni na ovom serveru. Pokrenite ovu naredbu u serveru u kojem ste potvrđeni.
deauth_done = Uspješno uklonjeni podaci.
deauth_more = Trenutno ste potvrđeni na sljedećem broju servera: {$num_servers_authed}. Želite li ukloniti podatke samo s ovog servera ili sa svih?
deauth_more_single = Ukloni samo s ovog
deauth_more_single_done = Uspješno uklonjeni podaci s ovog servera.
deauth_more_multi = Ukloni sa svih servera u kojima se nalazim
deauth_more_multi_done = Uspješno uklonjeni podaci sa sljedećeg broja servera: {$num_servers_authed}.
deauth_log = {$mention} uklonio/-la je potvrdu s ovog servera.
deauth_audit_log = Uklonjena potvrda

auth_failed_blocked = Potvrda nije uspjela: Blokirani ste na jednom ili na više Wikimedijinih projekata, što Vam onemogućava potvrdu na ovom serveru. Obratite se administratorima servera za pomoć.
auth_failed_error = Potvrda nije uspjela: Došlo je do interne greška. Za prijavu grešaka, obratite se korisniku beef.w na Discordu.
removed_blocked_user_roles = Uklonjena uloga blokiranom korisniku
adding_managed_role = Dodajem ulogu kojom upravlja bot jer odgovara kriterijima
removing_managed_role = Uklanjam ulogu kojom upravlja bot jer više ne odgovara kriterijima

server_auth_success = Uspješno! Podaci poslani botu :)
