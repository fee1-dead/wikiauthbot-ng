auth = Zəhmət olmasa, Vikimedia hesabınızı təsdiqləmək üçün keçidə daxil olun: [Təsdiqləyin]({$url})

auth_exists_in_server = Siz bu serverdə, onsuz da, təsdiqlənmisiniz. Yenidən təsdiqləməyə ehtiyac yoxdur.

auth_to_server = Siz artıq [{$name}](<{$url}>) kimi qeyd edilmisiniz. Bu serverdə də eyni kimliklə təsdiqlənmək istəyirsiniz?
yes = Bəli
no = Xeyr

auth_footer = Bu keçidin etibarlılıq müddəti 5 dəqiqədir.

authreq_canceled = Təsdiqlənmə prosesi ləğv edildi.
authreq_expired = Təsdiqlənmə prosesinin müddəti bitdi.

authlog = {$mention} adlı istifadəçi [User:{$username}](<{$user_link}>) (id {$wmf_id}) kimi təsdiqlənib.

# The entry in the audit log recording why the authenticated role was granted.
auditlog_successful_auth = {$wmf_id} Vikimedia istifadəçisi kimi təsdiqlənib

authreq_successful = Təsdiqlənmə prosesi uğurludur.

bot = WikiAuthBot

whois_no_user_found = İstifadəçi tapılmadı. O, ya bu serverdə yoxdur, ya da öz hesabını təsdiqləməyib.

revwhois_fail = Verilmiş istifadəçi haqqında məlumat əldə edilə bilmədi. Zəhmət olmasa, düzgün istifadəçi adını daxil edib-etmədiyinizi yoxlayın.

revwhois_no_auth = [{$name}](<{$user_link}>) bu serverdə öz hesabını təsdiqləməyib.

revwhois_one = [{$name}](<{$user_link}>) adlı istifadəçi {$mention} kimi təsdiqlənib.

# note: no space between colon and variable.
revwhois_multiple = [{$name}](<{$user_link}>) adlı istifadəçi bu hesabları təsdiqləyib:{$mentions}

user_link = https://en.wikipedia.org/w/index.php?title=Special%3ACentralAuth/{$normalized_name}

welcome_has_auth = Xoş gəlmisiniz, {$mention}! Siz artıq [{$name}](<{$user_link}>) kimi təsdiqlənmisiniz, buna görə də hesabınızı yenidən təsdiqləməyə ehtiyac yoxdur.

welcome_has_auth_failed = Xoş gəlmisiniz, {$mention}! Siz artıq təsdiqlənmisiniz (məlumat əldə edilən zaman xəta baş verdi), buna görə də hesabınızı yenidən təsdiqləməyə ehtiyac yoxdur.

welcome = Xoş gəlmisiniz, {$mention}! Öz Vikimedia hesabınızı təsdiqləmək (ictimai şəkildə əlaqələndirmək) istəyirsinizsə, </auth:1241068923730919464> komandasını işə salın.

whois_global_groups = Qlobal qruplar: {$groupslist}

whois_blocked = **BLOKLANIB**
whois_locked = **QIFILLANIB**
whois_pblocked = qismən bloklanıb
whois_edits = {$edits} redaktə
whois_groups = Qruplar: {$groupslist}
whois_overflow = Yalnızca 10-a qədər nəticə göstərilir. İstifadəçi haqqındakı bütün məlumatları görmək üçün yuxarıdakı istifadəçi adına klikləyin.
whois_no_block_reason = <!-- Səbəb yoxdur -->

# If you need a different date format other than YYYY-MM-DD, let me know.
whois = Discord: {$mention}
    Qeydiyyatdan keçib: {$registration}
    Əsas viki: {$home}
    {$global_groups}Cəmi redaktə sayı: {$edits}

cancel = imtina

deauth = Bu serverdəki təsdiqinizdən, dəqiq, imtina etmək istəyirsiniz?
deauth_canceled = Təsdiq imtinası prosesi ləğv edildi.
deauth_expired = Təsdiq imtinası prosesinin müddəti bitdi.
deauth_not_found = Bu serverdə hal-hazırda təsdiqlənməmisiniz. Komandanı təsdiqləndiyiniz bir serverdə işə salın.
deauth_done = Təsdiq datası uğurla silindi.
deauth_more = Hal-hazırda {$num_servers_authed} serverdə təsdiqlənmisiniz. Təsdiq datanızı təkcə bu serverdən, yoxsa {$num_servers_authed} serverin hamısından silmək istəyirsiniz?
deauth_more_single = Təkcə bu server
deauth_more_single_done = Bu serverdəki təsdiq datası uğurla silindi.
deauth_more_multi = Üzv olduğum bütün serverlər
deauth_more_multi_done = {$num_servers_authed} serverdəki təsdiq datası uğurla silindi.
deauth_log = {$mention} serverdəki təsdiqindən imtina etdi.
deauth_audit_log = Təsdiqdən imtina

auth_failed_blocked = Təsdiqlənmə prosesi uğursuz nəticələndi: Bir, yaxud bir neçə Vikimedia proyektində bloklandığınız üçün server sizi hesabınızı təsdiqləməyə qoymur.
auth_failed_error = Təsdiqlənmə prosesi uğursuz nəticələndi: Daxili bir xəta baş verdi. Zəhmət olmasa, baqlar barədə məlumat vermək üçün Discord vasitəsilə beef.w ilə əlaqə saxlayın.
removed_blocked_user_roles = Bloklanmış istifadəçidən rol götürüldü
adding_managed_role = Şərtlər ödəndiyi üçün bot tərəfindən idarə olunan rol əlavə edildi
removing_managed_role = Şərtlər daha ödənmədiyi üçün bot tərəfindən idarə olunan rol istifadəçidən götürüldü

server_auth_success = Uğurlu! Təsdiqlənmə barədə məlumat bota göndərildi :)
