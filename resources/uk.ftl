auth = Щоб автентифікувати свій обліковий запис Вікімедіа, перейдіть, будь ласка, за цим посиланням: [Authenticate]({$url})

auth_exists_in_server = Ви вже автентифіковані на цьому сервері. Немає потреби автентифікуватися ще раз.

auth_to_server = Вас уже ідентифіковано як [{$name}](<{$url}>). Бажаєте автентифікуватися так на сервері?
yes = Так
no = Ні

auth_footer = Це посилання буде дійсним упродовж 5 хвилин.

authreq_canceled = Автентифікацію скасовано.
authreq_expired = Термін дії автентифікації закінчився.

authlog = {$mention} автентифікувався/лася як [Користувач:{$username}](<{$user_link}>) (id {$wmf_id})

# The entry in the audit log recording why the authenticated role was granted.
auditlog_successful_auth = автентифікований/а як користувач Вікімедіа {$wmf_id}

authreq_successful = Успішно автентифіковано.

bot = WikiAuthBot

whois_no_user_found = Користувача не знайдено. Або користувача немає на цьому сервері, або його не автентифіковано.

revwhois_fail = Не вдалося отримати інформацію про вказаного користувача. Будь ласка, переконайтеся, що ви ввели правильне ім'я.

revwhois_no_auth = [{$name}](<{$user_link}>) не автентифіковано на цьому сервері.

revwhois_one = [{$name}](<{$user_link}>) автентифіковано як {$mention}

# note: no space between colon and variable.
revwhois_multiple = [{$name}](<{$user_link}>) автентифікований/а у таких обліковках:{$mentions}

user_link = https://uk.wikipedia.org/w/index.php?title=Special%3ACentralAuth/{$normalized_name}

welcome_has_auth = Привіт, {$mention}! Ви уже автентифіковані як [{$name}](<{$user_link}>), тому вам не потрібно автентифіковуватися ще раз.

welcome_has_auth_failed = Привіт, {$mention}! Ви уже автентифіковані як (помилка при отриманні інформації!), тому вам не потрібно автентифіковуватися ще раз.

welcome = Привіт, {$mention}! Якщо ви бажаєте автентифікувати (публічно пов'язати) свій обліковий запис Вікімедіа, введіть, будь ласка, </auth:1241068923730919464>

whois_global_groups = Глобальні групи: {$groupslist}

whois_blocked = **ЗАБЛОКОВАНО**
whois_locked = **ЗАКРИТО**
whois_pblocked = частково заблоковано
whois_edits = Редагування: {$edits}
whois_groups = Групи: {$groupslist}
whois_overflow = Наведено лише до 10 проєктів. Натисніть на ім'я угорі, щоб переглянути повну інформацію.
whois_no_block_reason = <!-- Причини не вказано -->

# If you need a different date format other than YYYY-MM-DD, let me know.
whois = Discord: {$mention}
    Зареєстровано: {$registration}
    Домашня вікі: {$home}
    {$global_groups}Разом редагувань: {$edits}

cancel = скасувати

deauth = Ви впевнені, що бажаєте прибрати автентифікацію себе з цього сервера?
deauth_canceled = Деавтентифікацію скасовано.
deauth_expired = Термін деавтентифікації минув.
deauth_not_found = Ви наразі не автентифіковані на цьому сервері. Виконайте цю команду на сервері, де ви автентифіковані.
deauth_done = Дані автентифікації успішно вилучено.
deauth_more = Ви наразі автентифіковані на {$num_servers_authed} сервері(ах). Ви бажаєте вилучити дані лише з цього сервера чи з усіх {$num_servers_authed} серверів?
deauth_more_single = Вилучити лише з цього сервера
deauth_more_single_done = Дані автентифікації успішно вилучено з цього сервера.
deauth_more_multi = Вилучити з усіх серверів, де я є
deauth_more_multi_done = Дані автентифікації успішно вилучено з {$num_servers_authed} серверів.
deauth_log = {$mention} деавтентифікувався/лася на цьому сервері.
deauth_audit_log = Деавтентифіковано

auth_failed_blocked = Автентифікація не вдалася: Вас заблоковано в одному чи більше проєктів Вікімедіа, що унеможливлює автентифікацію на цьому сервері. Зверніться по допомогу до адмінів сервера.
auth_failed_error = Автентифікація не вдалася: сталася внутрішня помилка. Будь ласка, зв'яжіться з beef.w у Discord і повідомте про баг.
removed_blocked_user_roles = Вилучено роль із заблокованого користувача
adding_managed_role = Додавання ботовиданої ролі на позначення відповідності критеріям
removing_managed_role = Вилучення ботовиданої ролі, оскільки користувач більше не відповідає критеріям

server_auth_success = Авторизаційну інформацію успішно надіслано ботові :)

cmd_whois = whois
cmd_whois_desc = Перевірити дані про обліковий запис автентифікованого учасника
cmd_whois_user = user
cmd_whois_user_desc = Користувач, чиї дані перевірити, залиште порожнім, якщо перевіряєте себе
cmd_whois_menu = Отримати інформацію
cmd_auth = auth
cmd_auth_desc = Автентифікувати свій обліковий запис Вікімедіа
cmd_revwhois = revwhois
cmd_revwhois_desc = Список обліковок Discord, пов'язаних з обліковкою Вікімедіа
cmd_revwhois_user = user
cmd_revwhois_user_desc = Ім'я користувача Вікімедіа
cmd_deauth = deauth
cmd_deauth_desc = Деавтентифікуватися або вилучити свої дані з бота.
