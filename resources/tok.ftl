-privacy_info = ilo WikiAuthBot #2 li kepeken seme e sona sina? o lukin e [lipu pi ken len](<https://wikiauthbot-ng.toolforge.org/ps>).

auth = kepeken linja ni la o pona e sona pi sijelo sina lon kulupu Wikimesija: [o pona]({$url})

    {-privacy_info}

auth_exists_in_server = sina pona e sona sina lon ma ni. sina wile ala pona sin.

auth_to_server = sijelo [{$name}](<{$url}>) la mi sona e sina. ma ni la sina wile ala wile pana e sona ni?

    {-privacy_info}

yes = wile
no = ala

auth_footer = tenpo 00:05:00 la ni li pona.

authreq_canceled = mi ala e wile pi pona sona.
authreq_expired = wile pi pona sona li majuna ike.

authlog = sona pona la jan {$mention} li jan [User:{$username}](<{$user_link}>) (id {$wmf_id})

# The entry in the audit log recording why the authenticated role was granted.
auditlog_successful_auth = kulupu Wikimesija la ona li pona e sona pi sijelo {$wmf_id}

authreq_successful = mi sona pona.

bot = ilo WikiAuthBot

whois_no_user_found = mi lukin ala e jan. ken la ona li lon ala ma ni. ken la ona li pona ala e sona sijelo.

revwhois_fail = mi ken ala lukin e sona jan. ni li nimi pona anu seme?

revwhois_no_auth = jan [{$name}](<{$user_link}>) li pona ala e sona sijelo lon ma ni.

revwhois_one = sona pona la jan [{$name}](<{$user_link}>) li jan {$mention}

# note: no space between colon and variable.
revwhois_multiple = sona pona la jan "[{$name}](<{$user_link}>)" li kepeken sijelo ni:{$mentions}

# Anticipated to become tok.wikipedia.org in the future.
user_link = https://meta.wikimedia.org/w/index.php?title=Special%3ACentralAuth/{$normalized_name}

welcome_has_auth = jan {$mention} o kama pona! mi sona e nimi sina [{$name}](<{$user_link}>), la sina wile ala pona sin e sona.

welcome_has_auth_failed = jan {$mention} o kama pona! sina alasa pana e nimi sina. mi sona e ni. taso pakala la ilo Wikimesija li pana ala e sijelo. taso sina wile ala pana sin e sona.
welcome = jan {$mention} o kama pona! kulupu Wikimesija la sina wile pona e sona pi sijelo sina, la o sitelen anu luka e ni: </auth:1241068923730919464>.

whois_global_groups = ken pi ma ali: {$groupslist}

whois_blocked = **ona li ken ala pali**
whois_locked = **ona li ken ala kepeken sijelo**
whois_pblocked = **kipisi la ona li ken ala pali**
whois_edits = pali: {$edits}
whois_groups = ken: {$groupslist}
whois_overflow = mi pana e sijelo 10 taso. sina wile lukin e ali, la o nena e nimi ona lon sewi.
whois_no_block_reason = <!-- tan li weka -->

# If you need a different date format other than YYYY-MM-DD, let me know.
whois = ilo Siko: {$mention}
    tenpo pi kama lon: {$registration}
    tomo: {$home}
    {$global_groups}mute pali: {$edits}

cancel = ala

deauth = ma ni la sina wile ala wile weka e pona sona pi sijelo sina?
deauth_canceled = mi ala e weka pi pona sona.
deauth_expired = weka pi pona sona la wile li majuna ike.
deauth_not_found = sina pona ala e sona pi sijelo sina lon ma ni. sina pona e sona lon ma ante, la sina ken kepeken pali ni lon ona.
deauth_done = mi weka e sona pi sijelo sina lon kulupu Wikimesija.
deauth_more = mi sona e sina lon ma {$num_servers_authed}. sina wile weka e sona tan ma ni taso anu ma {$num_servers_authed} ali anu seme?
deauth_more_single = ma ni taso
deauth_more_single_done = ma ni la mi sona ala e sijelo sina lon kulupu Wikimesija.
deauth_more_multi = ma ali
deauth_more_multi_done = mi weka e sona sijelo lon ma {$num_servers_authed}.
deauth_log = jan {$mention} li weka e sona sijelo lon ma ni.
deauth_audit_log = ona li weka e sona sijelo

auth_failed_blocked = pakala: sina ken ala pali lon lipu pi kulupu Wikimesija, la sina ken ala pona e sona pi sijelo sina. sina ken toki tawa jan lawa pi ma ni.
auth_failed_error = pakala: pakala insa li kama. o toki tawa jan "beef.w" lon ma Siko.
removed_blocked_user_roles = mi weka e poki tan jan pi ken pali ala.
adding_managed_role = jan li pona tawa wile mi, la mi poki e ona.
removing_managed_role = jan li kama ike tawa wile mi, la mi weka e ona tan poki.

server_auth_success = pona a! mi pana e sona ken tawa ilo :)

cmd_whois = seme
cmd_whois_desc = jan pi ma ni la o sona e sijelo pi kulupu Wikimesija
cmd_whois_user = jan
cmd_whois_user_desc = nimi jan pi ilo Siko. ni li sina, la o sitelen ala.
cmd_whois_menu = o kama sona e sijelo pi kulupu Wikimesija
cmd_auth = pona-sona
cmd_auth_desc = sijelo sina pi kulupu Wikimesija la o pona e sona
cmd_revwhois = seme-jasima
cmd_revwhois_desc = jan pi kulupu Wikimesija la o sona e sijelo pi ilo Siko
cmd_revwhois_user = jan
cmd_revwhois_user_desc = nimi jan pi kulupu Wikimesija
cmd_deauth = weka-sona
cmd_deauth_desc = o weka e sona tan ilo
