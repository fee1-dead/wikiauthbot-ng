-privacy_info = See WikiAuthBot2's [privacy statement](<https://wikiauthbot-ng.toolforge.org/ps>) to learn more about how we use your information.

auth = Usa il seguente link per autenticare il tuo account Wikimedia: [Authenticate]({$url})

    {-privacy_info}

auth_exists_in_server = La tua utenza è già autenticata in questo server. Non c'è bisogno di autenticarsi di nuovo.

auth_to_server = La tua utenza è già autenticata come [{$name}](<{$url}>). Desideri autenticarla nel server?

    {-privacy_info}

yes = Sì
no = No

auth_footer = Questo link sarà valido per i prossimi 5 minuti.

authreq_canceled = Autenticazione annullata.
authreq_expired = Autenticazione scaduta.

authlog = {$mention} autenticato come [Utente:{$username}](<{$user_link}>) (id {$wmf_id})

# The entry in the audit log recording why the authenticated role was granted.
auditlog_successful_auth = autenticato come utente wikimedia {$wmf_id}

authreq_successful = Autenticazione avvenuta con successo.

bot = WikiAuthBot

whois_no_user_found = Utente non trovato. L'utente non è in questo server oppure non si è autenticato.

revwhois_fail = Impossibile recuperare le informazioni relative all'utente selezionato. Assicurati di aver fornito il nome utente corretto.

revwhois_no_auth = L'utente [{$name}](<{$user_link}>) non è autenticato in questo server.

revwhois_one = [{$name}](<{$user_link}>) è autenticato come {$mention}

# note: no space between colon and variable.
revwhois_multiple = [{$name}](<{$user_link}>) è stato autenticato ai seguenti account:{$mentions}

user_link = https://it.wikipedia.org/w/index.php?title=Speciale%3AUtenzaGlobale/{$normalized_name}

welcome_has_auth = Benvenuto {$mention}! La tua utenza è stata già autenticata come [{$name}](<{$user_link}>), quindi non c'è bisogno che ti autentichi di nuovo.

welcome_has_auth_failed = Benvenuto {$mention}! La tua utenza è già autenticata (errore durante il recupero delle info!), quindi non c'è bisogno che ti reautentichi.

welcome = Benvenuto/a {$mention}! Se desideri autenticare (collegare pubblicamente) il tuo account Wikimedia, per favore premi o clicca </auth:1241068923730919464>

whois_global_groups = Gruppi globali: {$groupslist}

whois_blocked = **BLOCCATO**
whois_locked = **GLOBALMENTE BLOCCATO**
whois_pblocked = parzialmente bloccato
whois_edits = Modifiche: {$edits}
whois_groups = Gruppi: {$groupslist}
whois_overflow = Sono elencati al massimo 10 utenti. Clicca il loro nome in cima per vedere tutte le info.
whois_no_block_reason = <!-- Nessuna motivazione fornita -->

# If you need a different date format other than YYYY-MM-DD, let me know.
whois = Discord: {$mention}
    Registrato: {$registration}
    Home: {$home}
    {$global_groups}Modifiche totali: {$edits}

cancel = annulla

deauth = Sei sicuro/a di voler rimuovere la tua autenticazione da questo server?
deauth_canceled = Deautenticazione annullata.
deauth_expired = Deautenticazione scaduta.
deauth_not_found = La tua utenza non è attualmente autenticata in questo server. Esegui questo comando nel server dove è stata già autenticata.

deauth_done = Dati di autenticazione rimossi con successo.
deauth_more = La tua utenza è attualmente autenticata in {$num_servers_authed} server. Desideri cancellare i dati solo da questo server, oppure da tutti quanti ({$num_servers_authed})?
deauth_more_single = Cancella solo in questo server
deauth_more_single_done = Rimosso con successo i dati di autenticazione da questo server.
deauth_more_multi = Cancella da tutti i server in cui ne faccio parte
deauth_more_multi_done = Rimosso con successo i dati di autenticazione da {$num_servers_authed} server.
deauth_log = {$mention} ha revocato la sua autenticazione in questo server.
deauth_audit_log = Deautenticato

auth_failed_blocked = Autenticazione fallita: La tua utenza è stata bloccata in uno o più progetti Wikimedia, ciò impedisce l'autenticazione in questo server. Contattare gli amministratori del server per richiedere aiuto.
auth_failed_error = Autenticazione fallita: si è verificato un errore interno. Si prega di contattare dbeef su Discord per segnalare eventuali bug.
removed_blocked_user_roles = Rimosso ruolo da utente bloccato
adding_managed_role = Aggiunto ruolo gestito da bot visto che l'utente rispetta i requisiti
removing_managed_role = Rimosso ruolo gestito da bot visto che l'utente non rispetta più i requisiti

server_auth_success = Fantastico! Le informazioni di autenticazione sono state inviate al bot :)

cmd_whois = whois
cmd_whois_desc = Controlla i dettagli dell'account per un membro autenticato
cmd_whois_user = utente
cmd_whois_user_desc = Utente da controllare, lascia vuoto per autocontrollarti
cmd_whois_menu = Ricevi whois
cmd_auth = auth
cmd_auth_desc = Autentica il tuo account Wikimedia
cmd_revwhois = revwhois
cmd_revwhois_desc = Elenca gli account Discord associati a un specifico account Wikimedia
cmd_revwhois_user = utente
cmd_revwhois_user_desc = Nome dell'utente Wikimedia
cmd_deauth = deauth
cmd_deauth_desc = Deautentica o rimuovi i tuoi dati dal bot.
