auth = Sila gunakan pautan berikut untuk mengesahkan akaun Wikimedia anda: [Sahkan]({$url})

auth_exists_in_server = Anda sudah pun disahkan dalam pelayan ini. Tiada keperluan mengesahkan lagi.

auth_to_server = Anda sudah disahkan sebagai [{$name}](<{$url}>). Inginkah anda mengesahkan ini kepada pelayan ini?
yes = Ya
no = Tidak

auth_footer = Pautan ini sah bagi 5 minit.

authreq_canceled = Pengesahan dibatal.
authreq_expired = Pengesahan luput.

authlog = {$mention} disahkan sebagai [Pengguna:{$username}](<{$user_link}>) (id {$wmf_id})

# Masukan dalam log audit merakan mengapa peranan pengguna disahkan diberi.
auditlog_successful_auth = disahkan sebagai pengguna wikimedia {$wmf_id}

authreq_successful = Pengesahan berjaya.

bot = WikiAuthBot

whois_no_user_found = Tiada pengguna ditemui. Sama ada pengguna tidak dalam pelayan ini atau belum disahkan.

revwhois_fail = Gagal memperoleh maklumat pengguna diberi. Sila pastikan anda telah memberi nama pengguna betul.

revwhois_no_auth = [{$name}](<{$user_link}>) belum disahkan di pelayan ini.

revwhois_one = [{$name}](<{$user_link}>) disahkan sebagai {$mention}

# note: tiada jarak di antara noktah bertindih dan pemboleh ubah.
revwhois_multiple = [{$name}](<{$user_link}>) disahkan kepada akaun berikut:{$mentions}

user_link = https://en.wikipedia.org/w/index.php?title=Special%3ACentralAuth/{$normalized_name}

welcome_has_auth = Selamat datang {$mention}! Anda telah disahkan sebagai [{$name}](<{$user_link}>), jadi anda tak perlu mengesahkan diri sekali lagi.

welcome_has_auth_failed = Selamat datang {$mention}! Anda telah disahkan (ralat ketika mencapai maklumat!), jadi anda tak perlu mengesahkan diri sekali lagi.

welcome = Selamat datang {$mention}! Jika anda ingin mengesahkan (pautkan secara awam) akaun Wikimedia anda, sila taip atau tekan </auth:1241068923730919464>

whois_global_groups = Kumpulan global: {$groupslist}

whois_blocked = **DISEKAT**
whois_locked = **DIKUNCI**
whois_pblocked = disekat separa
whois_edits = Suntingan: {$edits}
whois_groups = Kumpulan: {$groupslist}
whois_overflow = Setakat 10 sahaja disenaraikan. Tekan namanya di atas untuk melihat semua maklumat.
whois_no_block_reason = <!-- Tiada sebab diberi -->

# Jika anda perlukan format tarikh lain berbanding YYYY-MM-DD, sila maklumkan saya.
whois = Discord: {$mention}
    Didaftar: {$registration}
    Utama: {$home}
    {$global_groups}Jumlah suntingan: {$edits}

cancel = batal

deauth = Anda pasti anda ingin membuang pengesahan anda daripada pelayan ini?
deauth_canceled = Penyahsahan dibatal.
deauth_expired = Penyahsahan luput.
deauth_not_found = Anda belum disahkan kepada pelayan ini buat masa kini. Jalankan perintah ini dalam satu pelayan di mana anda telah disahkan.
deauth_done = Berjaya membuang data pengesahan.
deauth_more = Anda kini disahkan dalam {$num_servers_authed} pelayan. Inginkah anda membuang data daripada pelayan ini sahaja, atau kesemua {$num_servers_authed} pelayan?
deauth_more_single = Buang hanya daripada pelayan ini
deauth_more_single_done = Berjaya membuang data pengesahan daripada pelayan ini.
deauth_more_multi = Buang daripada semua pelayan saya sertai
deauth_more_multi_done = Berjaya membuang data pengesahan daripada {$num_servers_authed} pelayan.
deauth_log = {$mention} telah dinyahsahkan daripada pelayan ini.
deauth_audit_log = Dinyahsahkan

auth_failed_blocked = Pengesahan gagal: Anda telah disekat daripada satu atau lebih projek Wikimedia, yang melarang andna daripada mengesahkan diri kepada pelayan ini. Hubungi pentadbir pelayan untuk bantuan
auth_failed_error = Pengesahan gagal: Ralat dalaman telah berlaku. Sila hubungi beef.w di Discord untuk melaporkan pepijat.
removed_blocked_user_roles = Peranan dibuang daripada pengguna tersekat
adding_managed_role = Menambah peranan kendalian bot untuk menepati ukur tara
removing_managed_role = Membuang peranan kendalian bot kerana pengguna tidak lagi menepati ukur tara

server_auth_success = Berjaya! Maklumat pembenaran telah dihantar ke bot :)

cmd_whois = whois
cmd_whois_desc = Periksa perincian akaun untuk ahli yang disahkan
cmd_whois_user = user
cmd_whois_user_desc = Pengguna untuk diperiksa, kosongkan jika diri anda
cmd_whois_menu = Get whois
cmd_auth = auth
cmd_auth_desc = Sahkan akaun Wikimedia anda
cmd_revwhois = revwhois
cmd_revwhois_desc = Senarai akaun Discord terkait pada akaun Wikimedia
cmd_revwhois_user = user
cmd_revwhois_user_desc = Nama pengguna Wikimedia
cmd_deauth = deauth
cmd_deauth_desc = Nyahsahkan atau buang data anda daripada bot.