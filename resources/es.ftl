auth = Por favor usa el siguiente enlace para autenticarte en tu cuenta Wikimedia: [Authenticate]({$url})

auth_exists_in_server = Actualmente estás autenticado en este servidor. No es necesario autenticarse de nuevo.

auth_to_server = Estás identificado como [{$name}](<{$url}>). ¿Quieres autenticarte de nuevo en este servidor?
yes = sí
no = No

auth_footer = Este enlace tendrá validez por 5 minutos.

authreq_canceled = Autenticación cancelada.
authreq_expired = Autenticación expirada.

authlog = {$mention} autenticado como [User:{$username}](<{$user_link}>) (id {$wmf_id}).

# The entry in the audit log recording why the authenticated role was granted.
auditlog_successful_auth = autenticado como el usuario {$wmf_id}.

authreq_successful = Autenticación exitosa.

bot = WikiAuthBot

whois_no_user_found = Usuario no encontrado. Puede deberse a que el usuario no esté en el servidor o esté sin autenticar.

revwhois_fail = Imposible obtener información del usuario indicado. Por favor asegúrate de haber indicado el usuario correcto

revwhois_no_auth = [{$name}](<{$user_link}>) no está autenticado en este servidor

revwhois_one = [{$name}](<{$user_link}>) está autenticado como {$mention}

# note: no space between colon and variable.
revwhois_multiple = [{$name}](<{$user_link}>) está autenticado para las siguientes cuentas:{$mentions}

user_link = https://es.wikipedia.org/w/index.php?title=Special%3ACentralAuth/{$normalized_name}

welcome_has_auth = ¡Bienvenido, {$mention}! Estás autenticado como [{$name}](<{$user_link}>), no necesitas autenticarte de nuevo.

welcome_has_auth_failed = ¡Bienvenido, {$mention}! Estás autenticado (oopsie, no tengo información al respecto), no necesitas autenticarte de nuevo.

welcome = ¡Bienvenido, {$mention}! Si deseas autenticar (validar) tu identidad como integrante del movimiento Wikimedia por favor usa: </auth:1221128504410898571>

whois_global_groups = Grupos globales: {$groupslist}

whois_blocked = **BLOQUEADO**
whois_locked = **DESHABILITADO**
whois_edits = Ediciones: {$edits}
whois_groups = Grupos: {$groupslist}
whois_overflow = Limitado a 10 proyectos. Da clic en su nombre al principio para observar toda la información.
whois_no_block_reason = <!-- No se ha provisto razón del bloqueo -->

# If you need a different date format other than YYYY-MM-DD, let me know.
whois = Discord: {$mention}
    Fecha de registro: {$registration}
    Proyecto principal: {$home}
    {$global_groups}Ediciones totales: {$edits}

# These are currently unused for now.
server_auth_success = Success! Authorization information sent to the bot :)
server_auth_expired = Auth request was expired or invalid.
