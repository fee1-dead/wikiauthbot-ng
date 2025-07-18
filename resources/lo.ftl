-privacy_info = See WikiAuthBot2's [privacy statement](<https://wikiauthbot-ng.toolforge.org/ps>) to learn more about how we use your information.

auth = ກະລຸນາໃຊ້ລິ້ງຕໍ່ໄປນີ້ເພື່ອພິສູດຢືນຢັນບັນຊີວິກິມີເດຍຂອງທ່ານ: [ຢຶນຢັນ]({$url})

    {-privacy_info}

auth_exists_in_server = ທ່ານໄດ້ຮັບການຢືນຢັນກັບເຊີບເວີນີ້ແລ້ວ ບໍ່ຈໍາເປັນຕ້ອງຢືນຢັນອີກເທື່ອໜຶ່ງ

auth_to_server = ທ່ານໄດ້ຖືກຢຶນຢັນແລ້ວເປັນ [{$name}](<{$url}>) ທ່ານຕ້ອງການຢືນຢັນອັນນີ້ໃຫ້ກັບເຊີບເວີບໍ່

    {-privacy_info}
yes = ແມ່ນ
no = ບໍ່

auth_footer = ລິ້ງນີ້ຈະໃຊ້ໄດ້ເປັນເວລາ 5 ນາທີ

authreq_canceled = ຍົກເລີກການຢືນຢັນແລ້ວ
authreq_expired = ການຢືນຢັນໝົດອາຍຸແລ້ວ

authlog = {$mention} ຢືນຢັນເປັນ [ຜູ້ໃຊ້:{$username}](<{$user_link}>) (ໄອດີ {$wmf_id})

# ການເຂົ້າ ຢູ່ ໃນບັນທຶກການກວດສອບວ່າເປັນຫຍັງຈັ່ງໄດ້ຮັບບົດບາດການຢືນຢັນ
auditlog_successful_auth = ຢຶນຢັນເປັນຜູ້ໃຊ້ວິກິມີເດຍ {$wmf_id}

authreq_successful = ການຢືນຢັນສຳເລັດແລ້ວ

bot = WikiAuthBot

whois_no_user_found = ບໍ່ພົບຜູ້ໃຊ້ ຜູ້ໃຊ້ບໍ່ໄດ້ ຢູ່ ໃນເຊີບເວີນີ້ ຫຼື ບໍ່ໄດ້ຜ່ານການຢືນຢັນ

revwhois_fail = ບໍ່ສາມາດດຶງຂໍ້ມູນສໍາລັບຜູ້ໃຊ້ທີ່ລະບຸ ກະລຸນາກວດສອບໃຫ້ແນ່ໃຈວ່າທ່ານໄດ້ລະບຸຊື່ຜູ້ໃຊ້ທີ່ຖືກຕ້ອງແລ້ວ

revwhois_no_auth = [{$name}](<{$user_link}>) ບໍ່ໄດ້ຢືນຢັນກັບເຊີບເວີນີ້

revwhois_one = [{$name}](<{$user_link}>) ໄດ້ຮັບການຢືນຢັນທີ່ {$mention}

# ຫມາຍເຫດ: ບໍ່ມີຊ່ອງຫວ່າງລະຫວ່າງຈໍ້າສອງເມັດ ແລະ ຕົວແປ
revwhois_multiple = [{$name}](<{$user_link}>) ໄດ້ຮັບການຢືນຢັນໃນບັນຊີຕໍ່ໄປນີ້:{$mentions}

user_link = https://lo.wikipedia.org/w/index.php?title=ພິເສດ%3ACentralAuth/{$normalized_name}

welcome_has_auth = ຍິນດີຕ້ອນຮັບ {$mention}! ທ່ານໄດ້ຢືນຢັນແລ້ວເປັນ [{$name}](<{$user_link}>) ສະນັ້ນ ທ່ານບໍ່ຈຳເປັນຕ້ອງຢືນຢັນອີກເທື່ອໜຶ່ງ

welcome_has_auth_failed = ຍິນດີຕ້ອນຮັບ {$mention}! ທ່ານໄດ້ຢຶນຢັນແລ້ວ (ເກີດຄວາມຜິດພາດໃນຂະນະທີ່ພະຍາຍາມດຶງຂໍ້ມູນ!) ສະນັ້ນ ທ່ານບໍ່ຈຳເປັນຕ້ອງຢືນຢັນອີກເທື່ອໜຶ່ງ

welcome = ຍິນດີຕ້ອນຮັບ {$mention}! ຖ້າທ່ານຕ້ອງການຢຶນຢັນ (ລິ້ງສາທາລະນະ) ບັນຊີວິກິມີເດຍຂອງທ່ານ ກະລຸນາພິມ ຫຼື ຄລິກ  </auth:1241068923730919464>

whois_global_groups = ກຸ່ມຜູ້ໃຊ້ທົ່ວໂລກ: {$groupslist}

whois_blocked = **ບລັອກ**
whois_locked = **ລັອກ**
whois_pblocked = ບລັອກບາງສ່ວນ
whois_edits = ດັດແກ້: {$edits}
whois_groups = ກຸ່ມຜູ້ໃຊ້: {$groupslist}
whois_overflow = ສະແດງໄດ້ສູງສຸດ 10 ລາຍຊື່ ຄລິກຊື່ຂອງເຂົາເຈົ້າ ຢູ່ ເທິງສຸດເພື່ອເບິ່ງຂໍ້ມູນທັງໝົດ
whois_no_block_reason = <!-- ບໍ່ໄດ້ໃຫ້ເຫດຜົນ -->

# ຖ້າທ່ານຕ້ອງການຮູບແບບວັນທີອື່ນນອກຈາກ YYYY-MM-DD ບອກໃຫ້ຂ້ອຍຮູ້
whois = ດິສຄອດ: {$mention}
    ລົງທະບຽນ: {$registration}
    ວິກິບ້ານ: {$home}
    {$global_groups}ດັດແກ້ທັງໝົດ: {$edits}

cancel = ຍົກເລີກ

deauth = ທ່ານແນ່ໃຈບໍ່ວ່າຕ້ອງການລຶບການຢືນຢັນຂອງທ່ານອອກຈາກເຊີບເວີນີ້
deauth_canceled = ຍົກເລີກການຍົກເລີກຢືນຢັນແລ້ວ
deauth_expired = ການຍົກເລີກຢືນຢັນໝົດອາຍຸແລ້ວ
deauth_not_found = ປັດຈຸບັນທ່ານຍັງບໍ່ໄດ້ຢືນຢັນກັບເຊີບເວີນີ້ ດໍາເນີນການຄໍາສັ່ງນີ້ ຢູ່ ໃນເຊີບເວີທີ່ທ່ານໄດ້ຮັບການຢຶນຢັນ
deauth_done = ລຶບຂໍ້ມູນການຢືນຢັນສຳເລັດແລ້ວ
deauth_more = ປັດຈຸບັນທ່ານໄດ້ຮັບການຢືນຢັນເຖິງ {$num_servers_authed} ເຊີບເວີ ທ່ານຕ້ອງການລຶບຂໍ້ມູນອອກຈາກເຊີບເວີນີ້ເທົ່ານັ້ນ ຫຼື ຈາກທັງໝົດ {$num_servers_authed} ເຊີບເວີ
deauth_more_single = ລຶບອອກຈາກເຊີບເວີນີ້ເທົ່ານັ້ນ
deauth_more_single_done = ລຶບຂໍ້ມູນການຢືນຢັນອອກຈາກເຊີບເວີນີ້ສຳເລັດແລ້ວ
deauth_more_multi = ລຶບອອກຈາກເຊີບເວີທັງໝົດ
deauth_more_multi_done = ສຳເລັດ ລຶບຂໍ້ມູນການຢືນຢັນອອກຈາກ {$num_servers_authed} ເຊີບເວີ
deauth_log = {$mention} ໄດ້ຍົກເລີກຢືນຢັນຈາກເຊີບເວີນີ້
deauth_audit_log = ຍົກເລີກຢືນຢັນ

auth_failed_blocked = ການພິສູດຢືນຢັນບໍ່ສຳເລັດ: ທ່ານໄດ້ຖືກບລັອກຈາກໜຶ່ງ ຫຼື ຫຼາຍໂຄງການຂອງວິກິມີເດຍ ເຊິ່ງປ້ອງກັນການຢືນຢັນກັບເຊີບເວີນີ້ ຕິດຕໍ່ຜູ້ດູແລເຊີບເວີເພື່ອຂໍຄວາມຊ່ວຍເຫຼືອ
auth_failed_error = ການພິສູດຢືນຢັນບໍ່ສຳເລັດ: ມີຂໍ້ຜິດພາດພາຍໃນເກີດຂຶ້ນ ກະລຸນາຕິດຕໍ່ beef.w ທີ່ດິສຄອດສໍາລັບການລາຍງານບັຄ
removed_blocked_user_roles = ລຶບບົດບາດຈາກຜູ້ໃຊ້ທີ່ຖືກບລັອກ
adding_managed_role = ເພີ່ມບົດບາດການຈັດການບັອດ ສໍາລັບຜູ້ໃຊ້ທີ່ ຢູ່ ໃນເງື່ອນໄຂ
removing_managed_role = ລຶບບົດບາດການຈັດການບັອດ ຍ້ອນວ່າຜູ້ໃຊ້ບໍ່ກົງກັບເງື່ອນໄຂອີກຕໍ່ໄປ

server_auth_success = ສຳເລັດ! ຂໍ້ມູນການຢືນຢັນໄດ້ຮັບການສົ່ງໄປຍັງບັອດແລ້ວ :)

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
