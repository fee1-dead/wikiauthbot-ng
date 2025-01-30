auth = Veuillez utiliser le lien suivant pour vous authentifier sur votre compte Wikimedia : [Authentifier]({$url})

auth_exists_in_server = Vous êtes déjà authentifié·ée sur ce serveur. Pas besoin de s'authentifier à nouveau.

auth_to_server = Vous êtes déjà identifié·ée comme [{$name}](<{$url}>). Souhaitez-vous l'authentifier auprès du serveur ?
yes = Oui
no = Non

auth_footer = Ce lien sera valide pendant 5 minutes.

authreq_canceled = Authentification annulée.

authreq_expired = L'authentification a expiré.

authlog = {$mention} authentifié·ée comme [User:{$username}](<{$user_link}>) (id {$wmf_id})

# The entry in the audit log recording why the authenticated role was granted.
auditlog_successful_auth = authentifié·ée en tant qu'utilisateur·trice Wikimedia {$wmf_id}

authreq_successful = Authentification réussie.

bot = WikiAuthBot

whois_no_user_found = Aucun·e utilisateur·trice trouvé·ée. Soit l'utilisateur·trice n'est pas sur ce serveur, soit iel n'est pas authentifié·ée.

revwhois_fail = Impossible de récupérer les informations pour un·e utilisateur·trice donné·ée. Veuillez vous assurer que vous avez fourni le bon nom d'utilisateur·trice.

revwhois_no_auth = [{$name}](<{$user_link}>) ne s'est pas authentifié·ée sur ce serveur.

revwhois_one = [{$name}](<{$user_link}>) est authentifié·ée sous {$mention}

# note: no space between colon and variable.
revwhois_multiple = [{$name}](<{$user_link}>) est authentifié·ée auprès des comptes suivants :{$mentions}

user_link = https://fr.wikipedia.org/w/index.php?title=Sp%C3%A9cial%3ACentralAuth/{$normalized_name}

welcome_has_auth = Bienvenue {$mention} ! Vous êtes déjà authentifié·ée en tant que [{$name}](<{$user_link}>), vous n'avez donc pas besoin de vous authentifier à nouveau.

welcome_has_auth_failed = Bienvenue {$mention} ! Vous êtes déjà authentifié·ée (erreur lors de la tentative de récupération des informations !), vous n'avez donc pas besoin de vous authentifier à nouveau.

# note: The command ID is changed to the new one, according to https://discord.com/channels/221049808784326656/1221136365639434382/1241074873355468923
welcome = Bienvenue {$mention} ! Si vous souhaitez authentifier (valider) votre compte Wikimedia, veuillez taper </auth:1241068923730919464>

whois_global_groups = Groupes globaux : {$groupslist}

whois_blocked = **BLOQUÉ**
whois_locked = **VERROUILLÉ**
whois_pblocked = partially blocked
whois_edits = Contributions : {$edits}
whois_groups = Statuts : {$groupslist}
whois_overflow = Jusqu'à 10 max répertoriés seulement. Cliquez sur leur nom en haut pour voir toutes les informations.
whois_no_block_reason = <!-- Aucune raison donnée -->

# If you need a different date format other than YYYY-MM-DD, let me know.
whois = Discord : {$mention}
    Date d’inscription : {$registration}
    Wiki d’origine : {$home}
    {$global_groups}Total des contributions : {$edits}

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

# These are currently unused for now. Please still translate this if possible!
server_auth_success = Succès ! Informations d'autorisation envoyées au bot :)
server_auth_expired = La demande d'authentification a expiré ou n'est pas valide.
