auth = আপনার উইকিমিডিয়া অ্যাকাউন্ট প্রমাণীকরণের জন্য নিম্নলিখিত লিঙ্কটি ব্যবহার করুন: [প্রমাণীকরণ করুন]({$url})

auth_exists_in_server = আপনি ইতিমধ্যেই এই সার্ভারে প্রমাণীকৃত আছেন। পুনরায় প্রমাণীকরণের প্রয়োজন নেই।

auth_to_server = আপনি ইতিমধ্যে [{$name}](<{$url}>) হিসাবে সনাক্ত হয়েছেন। আপনি কি এটি সার্ভারে প্রমাণীকরণ করতে চান?
yes = হ্যাঁ
no = না

auth_footer = এই লিঙ্কটি ৫ মিনিটের জন্য বৈধ থাকবে।

authreq_canceled = প্রমাণীকরণ বাতিল করা হয়েছে।
authreq_expired = প্রমাণীকরণের সময়সীমা শেষ হয়েছে।

authlog = {$mention} [ব্যবহারকারী:{$username}](<{$user_link}>) (আইডি নং {$wmf_id}) হিসেবে প্রমাণীকৃত হয়েছেন

# The entry in the audit log recording why the authenticated role was granted.
auditlog_successful_auth = উইকিমিডিয়া ব্যবহারকারী {$wmf_id} হিসেবে প্রমাণীকৃত হয়েছেন

authreq_successful = প্রমাণীকরণ সফল হয়েছে।

bot = WikiAuthBot

whois_no_user_found = কোনো ব্যবহারকারী পাওয়া যায়নি। এই ব্যবহারকারী হয়ত এই সার্ভারে নেই অথবা প্রমাণীকরণ করেননি।

revwhois_fail = প্রদত্ত ব্যবহারকারীর তথ্য আনা যায়নি। দয়া করে নিশ্চিত করুন যে আপনি সঠিক ব্যবহারকারী নাম প্রদান করেছেন।

revwhois_no_auth = [{$name}](<{$user_link}>) এই সার্ভারে প্রমাণীকরণ করেননি।

revwhois_one = [{$name}](<{$user_link}>) এই সার্ভারে {$mention} হিসেবে নিজেকে প্রমাণীকরণ করেছেন।

# note: no space between colon and variable.
revwhois_multiple = [{$name}](<{$user_link}>) নিম্নলিখিত অ্যাকাউন্টে প্রমাণীকৃত:{$mentions}

user_link = https://bn.wikipedia.org/w/index.php?title=Special%3ACentralAuth/{$normalized_name}

welcome_has_auth = স্বাগতম {$mention}! আপনি ইতিমধ্যে [{$name}](<{$user_link}>) হিসেবে প্রমাণীকৃত, তাই আপনাকে পুনরায় প্রমাণীকরণ করতে হবে না।

welcome_has_auth_failed = স্বাগতম {$mention}! আপনি ইতিমধ্যে প্রমাণীকৃত (তথ্য আনতে ত্রুটি হয়েছে!), তাই আপনাকে পুনরায় প্রমাণীকরণ করতে হবে না।

welcome = স্বাগতম {$mention}! আপনি যদি আপনার উইকিমিডিয়া অ্যাকাউন্ট প্রমাণীকরণ করতে চান (সর্বজনীনভাবে লিঙ্ক করতে চান), অনুগ্রহ করে টাইপ করুন বা ক্লিক করুন: </auth:1241068923730919464>

whois_global_groups = বৈশ্বিক দল: {$groupslist}

whois_blocked = **বাধাপ্রাপ্ত**
whois_locked = **তালাবদ্ধ**
whois_pblocked = partially blocked
whois_edits = সম্পাদনা: {$edits}
whois_groups = দল: {$groupslist}
whois_overflow = কেবল ১০টি তালিকাভুক্ত করা হয়েছে। সমস্ত তথ্য দেখতে উপরে নামের উপর ক্লিক করুন।
whois_no_block_reason = <!-- কোনো কারণ দেওয়া হয়নি -->

# If you need a different date format other than YYYY-MM-DD, let me know.
whois = ডিসকর্ড: {$mention}
    নিবন্ধন: {$registration}
    নীড়: {$home}
    {$global_groups}মোট সম্পাদনা: {$edits}

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
adding_managed_role = Adding bot-managed role for meeting criteria
removing_managed_role = Removing bot-managed role as user no longer meets criteria

server_auth_success = সফল! অনুমোদনের তথ্য বটের কাছে পাঠানো হয়েছে :)

cmd_whois = whois
cmd_whois_desc = Check account details for an authenticated member
cmd_whois_user = user
cmd_whois_user_desc = User to check, leave blank for yourself
cmd_whois_menu = Get whois
cmd_auth = auth
cmd_auth_desc = Authenticate to your Wikimedia account
cmd_revwhois = revwhois
cmd_revwhois_desc = List Discord accounts associated to a Wikimedia account
cmd_revwhois_user = user
cmd_revwhois_user_desc = Name of the Wikimedia user
cmd_deauth = deauth
cmd_deauth_desc = Deauthenticate or remove your data from the bot.
