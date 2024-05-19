auth = Please use the following link to authenticate to your Wikimedia account: [Authenticate]({$url})

auth_exists_in_server = You are already authenticated to this server. No need to authenticate again.

auth_to_server = You are already identified as [{$name}](<{$url}>). Would you like to authenticate this to the server?
yes = Yes
no = No

auth_footer = This link will be valid for 5 minutes.

authreq_canceled = Authentication canceled.
authreq_expired = Authentication expired.

authlog = {$mention} authenticated as [User:{$username}](<{$user_link}>) (id {$wmf_id})

# The entry in the audit log recording why the authenticated role was granted.
auditlog_successful_auth = authenticated as wikimedia user {$wmf_id}

authreq_successful = Authentication successful.

bot = WikiAuthBot

whois_no_user_found = No user found. Either the user is not in this server or is unauthenticated.

revwhois_fail = Could not fetch info for given user. Please make sure you have supplied the correct username.

revwhois_no_auth = [{$name}](<{$user_link}>) has not authenticated to this server.

revwhois_one = [{$name}](<{$user_link}>) is authenticated to {$mention}

# note: no space between colon and variable.
revwhois_multiple = [{$name}](<{$user_link}>) is authenticated to the following accounts:{$mentions}

user_link = https://en.wikipedia.org/w/index.php?title=Special%3ACentralAuth/{$normalized_name}

welcome_has_auth = Welcome {$mention}! You've already authenticated as [{$name}](<{$user_link}>), so you don't need to authenticate again.

welcome_has_auth_failed = Welcome {$mention}! You've already authenticated (error while trying to fetch info!), so you don't need to authenticate again.

welcome = Welcome {$mention}! If you would like to authenticate (validate) your Wikimedia account, please type </auth:1241068923730919464>

whois_global_groups = Global groups: {$groupslist}

whois_blocked = **BLOCKED**
whois_locked = **LOCKED**
whois_edits = Edits: {$edits}
whois_groups = Groups: {$groupslist}
whois_overflow = Only up to 10 max listed. Click their name at the top to see all info.
whois_no_block_reason = <!-- No reason given -->

# If you need a different date format other than YYYY-MM-DD, let me know.
whois = Discord: {$mention}
    Registered: {$registration}
    Home: {$home}
    {$global_groups}Total edits: {$edits}

# These are currently unused for now.
server_auth_success = Success! Authorization information sent to the bot :)
server_auth_expired = Auth request was expired or invalid.