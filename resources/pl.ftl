-privacy_info = Szczegółowe informacje na temat sposobu wykorzystywania Twoich danych znajdziesz w [oświadczeniu o ochronie prywatności](<https://wikiauthbot-ng.toolforge.org/ps>) WikiAuthBot2.

auth = Użyj następującego linku, aby zweryfikować swoje konto Wikimedia: [Przejdź do weryfikacji]({$url})

    {-privacy_info}

auth_exists_in_server = Udało Ci się już uwierzytelnić na tym serwerze. Nie musisz powtarzać tej czynności.

auth_to_server = Udało Ci się już uwierzytelnić jako [{$name}](<{$url}>). Czy chcesz korzystać z tego konta również na niniejszym serwerze?

    {-privacy_info}

yes = Tak
no = Nie

auth_footer = Ten link będzie ważny przez 5 minut.

authreq_canceled = Anulowano uwierzytelnianie.
authreq_expired = Prośba o uwierzytelnienie wygasła.

authlog = {$mention} uwierzytelniony(-a) jako [{$username}](<{$user_link}>) (id {$wmf_id})

# The entry in the audit log recording why the authenticated role was granted.
auditlog_successful_auth = uwierzytelniony(-a) jako użytkownik Wikimedia {$wmf_id}

authreq_successful = Uwierzytelnianie powiodło się.

bot = WikiAuthBot

whois_no_user_found = Nie znaleziono użytkownika. Nie ma go na tym serwerze lub nie jest uwierzytelniony.

revwhois_fail = Nie udało się pobrać informacji dla tego użytkownika. Upewnij się, że podano poprawną nazwę użytkownika.

revwhois_no_auth = [{$name}](<{$user_link}>) nie uwierzytelnił(a) się na tym serwerze.

revwhois_one = Konto Wikimedia [{$name}](<{$user_link}>) jest powiązane z {$mention}

# note: no space between colon and variable.
revwhois_multiple = Konto Wikimedia [{$name}](<{$user_link}>) jest powiązane z następującymi użytkownikami:{$mentions}

user_link = https://pl.wikipedia.org/w/index.php?title=Special%3ACentralAuth/{$normalized_name}

welcome_has_auth = Witamy, {$mention}! Udało Ci się już uwierzytelnić jako [{$name}](<{$user_link}>) i nie musisz potwierdzać swojego konta ponownie.

welcome_has_auth_failed = Witamy, {$mention}! Udało Ci się już uwierzytelnić i nie musisz potwierdzać swojego konta ponownie (wystąpił błąd przy pobieraniu nazwy Twojego konta Wikimedia).

welcome = Witamy, {$mention}! Jeśli chcesz uwierzytelnić (publicznie powiązać) swoje konto Wikimedia, wpisz lub kliknij </auth:1241068923730919464>

whois_global_groups = Grupy globalne: {$groupslist}

whois_blocked = **ZABLOKOWANO**
whois_locked = **ZABLOKOWANO GLOBALNIE**
whois_pblocked = zablokowano częściowo
whois_edits = Liczba edycji: {$edits}
whois_groups = Grupy: {$groupslist}
whois_overflow = Wyświetlono tylko 10 najważniejszych. Kliknij nazwę użytkownika u góry, aby zobaczyć wszystkie.
whois_no_block_reason = <!-- Nie podano powodu -->

# If you need a different date format other than YYYY-MM-DD, let me know.
whois = Discord: {$mention}
    Data rejestracji: {$registration}
    Domowa wiki: {$home}
    {$global_groups}Liczba wszystkich edycji: {$edits}

cancel = anuluj

deauth = Czy na pewno chcesz skasować powiązanie z kontem Wikimedia na tym serwerze?
deauth_canceled = Przerwano rozłączanie kont.
deauth_expired = Rozłączanie kont trwało za długo.
deauth_not_found = Nie jesteś uwierzytelniony(-a) na tym serwerze. Wywołaj tę komendę na serwerze, gdzie masz już uwierzytelnione konto.
deauth_done = Pomyślnie rozłączono konta.
deauth_more = Udało Ci się uwierzytelnić na {$num_servers_authed} serwerach. Czy chcesz rozłączyć konto jedynie na tym serwerze czy na wszystkich {$num_servers_authed} serwerach?
deauth_more_single = Rozłącz tylko na tym
deauth_more_single_done = Pomyślnie rozłączono konta na tym serwerze.
deauth_more_multi = Rozłącz na wszystkich serwerach, do których należę
deauth_more_multi_done = Pomyślnie rozłączono konta na {$num_servers_authed} serwerach.
deauth_log = {$mention} skasował(a) powiązanie z kontem Wikimedia na tym serwerze.
deauth_audit_log = Rozłączono konta

auth_failed_blocked = Uwierzytelnianie nie powiodło się. Twoje konto jest zablokowane na jednym z projektów Wikimedia. Z tego powodu nie możesz uwierzytelnić się na tym serwerze. Aby uzyskać pomoc, skontaktuj się z administratorami tego serwera.
auth_failed_error = Uwierzytelnianie nie powiodło się. Wystąpił nieznany błąd. Skontaktuj się z beef.w na Discordzie, aby zgłosić błąd.
removed_blocked_user_roles = Usunięto rolę zablokowanemu użytkownikowi
adding_managed_role = Dodano rolę obsługiwaną przez bota, bo użytkownik spełnia jej kryteria
removing_managed_role = Usunięto rolę obsługiwaną przez bota, bo użytkownik przestał spełniać jej kryteria

server_auth_success = Udało się! Informacje uwierzytelniające zostały przekazane do bota :)

cmd_whois = whois
cmd_whois_desc = Zobacz szczegóły powiązanego konta Wikimedia
cmd_whois_user = użytkownik
cmd_whois_user_desc = Użytkownik do sprawdzenia, pozostaw puste, aby sprawdzić siebie
cmd_whois_menu = Uzyskaj informacje
cmd_auth = auth
cmd_auth_desc = Połącz konto z kontem Wikimedia
cmd_revwhois = revwhois
cmd_revwhois_desc = Wyświetl listę użytkowników na Discordzie powiązanych z danym kontem Wikimedia
cmd_revwhois_user = użytkownik
cmd_revwhois_user_desc = Nazwa konta Wikimedia
cmd_deauth = deauth
cmd_deauth_desc = Rozłącz swoje konto Wikimedia lub usuń swoje dane z bota.
