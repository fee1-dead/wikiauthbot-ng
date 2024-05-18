auth = Użyj następującego linku, aby zweryfikować swoje konto Wikimedia: [Przejdź do weryfikacji]({$url})

auth_exists_in_server = Już jesteś uwierzytelniony(-a) na tym serwerze. Nie musisz powtarzać tej czynności.

auth_to_server = Uwierzytelniłeś(-aś) się już jako [{$name}](<{$url}>). Czy chcesz korzystać z tego konta również na niniejszym serwerze?
yes = Tak
no = Nie

auth_footer = Ten link będzie ważny przez 5 minut.

authreq_canceled = Anulowano uwierzytelnianie.
authreq_expired = Prośba o uwierzytelnienie wygasła.

authlog = {$mention} uwierzytelniony(-a) jako [User:{$username}](<{$user_link}>) (id {$wmf_id})

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

welcome = Witamy {$mention}! Jeśli chcesz uwierzytelnić (powiązać) swoje konto Wikimedia, wpisz </auth:1221128504410898571>

whois_global_groups = Grupy globalne: {$groupslist}

whois_blocked = **ZABLOKOWANO**
whois_locked = **ZABLOKOWANO GLOBALNIE**
whois_edits = Edycji: {$edits}
whois_groups = Grupy: {$groupslist}
whois_overflow = Tylko 10 najważniejszych jest wyświetlane. Kliknij nazwę użytkownika u góry, aby zobaczyć wszystkie.
whois_no_block_reason = <!-- Nie podano powodu -->

# If you need a different date format other than YYYY-MM-DD, let me know.
whois = Discord: {$mention}
    Data rejestracji: {$registration}
    Domowa wiki: {$home}
    {$global_groups}Wszystkich edycji: {$edits}

# These are currently unused for now.
server_auth_success = Sukces! Informacje uwierzytelniające zostały przekazane do bota :)
server_auth_expired = Prośba o uwierzytelnienie wygasła lub była niepoprawna.
