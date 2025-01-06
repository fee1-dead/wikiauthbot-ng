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
whois_edits = সম্পাদনা: {$edits}
whois_groups = দল: {$groupslist}
whois_overflow = কেবল ১০টি তালিকাভুক্ত করা হয়েছে। সমস্ত তথ্য দেখতে উপরে নামের উপর ক্লিক করুন।
whois_no_block_reason = <!-- কোনো কারণ দেওয়া হয়নি -->

# If you need a different date format other than YYYY-MM-DD, let me know.
whois = ডিসকর্ড: {$mention}
    নিবন্ধন: {$registration}
    নীড়: {$home}
    {$global_groups}মোট সম্পাদনা: {$edits}

# These are currently unused for now. Please still translate this if possible!
server_auth_success = সফল! অনুমোদনের তথ্য বটের কাছে পাঠানো হয়েছে :)
server_auth_expired = প্রমাণীকরণের অনুরোধ মেয়াদোত্তীর্ণ বা অবৈধ।
