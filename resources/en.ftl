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

welcome = Welcome {$mention}! If you would like to authenticate (publicly link) your Wikimedia account, please type or click </auth:1241068923730919464>

whois_global_groups = Global groups: {$groupslist}

whois_blocked = **BLOCKED**
whois_locked = **LOCKED**
whois_pblocked = partially blocked
whois_edits = Edits: {$edits}
whois_groups = Groups: {$groupslist}
whois_overflow = Only up to 10 max listed. Click their name at the top to see all info.
whois_no_block_reason = <!-- No reason given -->

# If you need a different date format other than YYYY-MM-DD, let me know.
whois = Discord: {$mention}
    Registered: {$registration}
    Home: {$home}
    {$global_groups}Total edits: {$edits}

cancel = cancel

deauth = Are you sure you want to remove your authentication from this server?
deauth_canceled = Deauthentication canceled.
deauth_expired = Deauthentication expired.
deauth_not_found = You are currently not authenticated to this server. Run this command in a server where you are authenticated.
deauth_done = Successfully removed authentication data.
deauth_more = You are currently authenticated to {$num_servers_authed} servers. Would you like to remove data from only this server, or from all {$num_servers_authed} servers?
deauth_more_single = Delete from only this server
deauth_more_single_done = Successfully removed authentication data from this server.
deauth_more_multi = Delete from all servers I am in
deauth_more_multi_done = Successfully removed authentication data from {$num_servers_authed} servers.
deauth_log = {$mention} has deauthenticated from this server.
deauth_audit_log = Deauthenticated

auth_failed_blocked = Authentication failed: You have been blocked from one or more Wikimedia projects, which prevents authentication to this server. Contact the server admins for help.
auth_failed_error = Authentication failed: An internal error has occured. Please contact beef.w on Discord for reporting bugs.
removed_blocked_user_roles = Removed role from blocked user

server_auth_success = Success! Authorization information sent to the bot :)
