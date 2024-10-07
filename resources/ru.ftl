auth = Чтобы аутентифицировать вашу учётную запись Викимедиа, пожалуйста, перейдите по следующей ссылке: [аутентифицироваться]({$url})

auth_exists_in_server = Вы уже аутентифицированы на этом сервере. Повторная аутентификация не требуется.

auth_to_server = Вы идентифицированы как [{$name}](<{$url}>). Хотите аутентифицироваться на сервере так?
yes = Да
no = Нет

auth_footer = Ссылка будет действительна 5 минут.

authreq_canceled = Аутентификация прервана.
authreq_expired = Аутентификация истекла.

authlog = {$mention} аутентифицирован(а) как [User:{$username}](<{$user_link}>) (id {$wmf_id})

# The entry in the audit log recording why the authenticated role was granted.
auditlog_successful_auth = аутентифицирован(а) как пользователь Викимедиа {$wmf_id}

authreq_successful = Аутентификация прошла успешно.

bot = WikiAuthBot

whois_no_user_found = Участник не найден. Либо участника нет на сервере, либо он не аутентифицирован.

revwhois_fail = Не удалось получить информацию о данном участнике. Пожалуйста, убедитесь, что вы указали корректный ник.

revwhois_no_auth = [{$name}](<{$user_link}>) не аутентифицирован(а) на сервере.

revwhois_one = [{$name}](<{$user_link}>) аутентифицирован(а) как {$mention}

# note: no space between colon and variable.
revwhois_multiple = [{$name}](<{$user_link}>) аутентифицирован(а) следующими учётными записями:{$mentions}

user_link = https://ru.wikipedia.org/w/index.php?title=Служебная%3ACentralAuth/{$normalized_name}

welcome_has_auth = Добро пожаловать, {$mention}! Вы уже аутентифицированы как [{$name}](<{$user_link}>), повторная аутентификация не требуется.

welcome_has_auth_failed = Добро пожаловать, {$mention}! Вы уже аутентифицированы как (ошибка при получении информации!), повторная аутентификация не требуется.

welcome = Добро пожаловать, {$mention}! Если вы хотите аутентифицировать (подтвердить) вашу учётную запись Викимедиа, пожалуйста, введите </auth:1241068923730919464>

whois_global_groups = Глобальные группы: {$groupslist}

whois_blocked = **ЗАБЛОКИРОВАН(А)**
whois_locked = **ЗАМОРОЖЕН(А)**
whois_edits = Правок: {$edits}
whois_groups = Группы: {$groupslist}
whois_overflow = Перечислено до 10 проектов. Для получения полной информации нажмите на ник вверху.
whois_no_block_reason = <!-- Причина не указана -->

# If you need a different date format other than YYYY-MM-DD, let me know.
whois = Discord: {$mention}
    Регистрация: {$registration}
    Домашний проект: {$home}
    {$global_groups}Всего правок: {$edits}

# These are currently unused for now. Please still translate this if possible!
server_auth_success = Успешно! Информация об авторизации отправлена боту. :)
server_auth_expired = Запрос на авторизацию истёк или недействителен.
