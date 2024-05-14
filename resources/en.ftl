authlog = {$mention} authenticated as [User:{$username}](<{$user_link}>) (id {$wmf_id})

# The entry in the audit log recording why the authenticated role was granted.
auditlog_successful_auth = authenticated as wikimedia user {$wmf_id}

authreq_successful = Authentication successful.

# TODO: this key currently doesn't do anything.
authreq_expired = Authentication request expired.

whois_no_user_found = No user found. Either the user is not in this server or is unauthenticated.

revwhois_fail = Could not fetch info for given user. Please make sure you have supplied the correct username.

user_link = https://en.wikipedia.org/w/index.php?title=Special%3ACentralAuth/{$normalized_name}
