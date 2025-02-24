# wikiauthbot-ng

The next generation (ng) authentication bot for Wikimedia Discord servers.

Current usage is limited to official and semi-official Wikimedia Discord servers only.

Please contact beef.w on Discord for an invite link for the bot.

## Setting up the bot in your server

After inviting the bot to your server, the bot must be fully configured to work properly.

DM the bot with the following command, note that you need administrator permissions on the server to run:

```
wab!setup_server [guild_id] [welcome_channel_id] [auth_log_channel_id] [deauth_log_channel_id] [authenticated_role_id] [server_language] [allow_banned_users] [whois_is_ephemeral] [allow_partially_blocked_users]
```

Here's some explanation for the options:

* To get the Guild ID of your server, open developer mode and right click your server to Copy ID. IDs for the later options should also use the Copy ID feature.
* Welcome channel: the channel the bot will send the message "Welcome @Foo! If you would like to authenticate (validate) your Wikimedia account, please type /auth" in.
* Auth log channel: Channel to send "@Foo authenticated as User:Foo (id xxxx)"
* Deauth log channel: currently unused but you must fill in a channel. The channel to send "@Foo has deauthenticated to the server, they were previously User:Foo (id xxxx)"
* Authenticated role ID: ID of the role for authenticated accounts
* Server language: supported options are found in [this directory](/resources/). If you would like to help translating to another language, please let me know on discord.
* Allow Banned Users: Setting this to `false` would prevent anyone who is indefinitely blocked on *any* project (sitewide blocks only) from authenticating and receiving the authenticated roles.
* Whois is Ephemeral: Use "true" if you want the `/whois` command and the "Get whois" right click action to only display to the user who ran the command. Use "false" if you want all whois output to be public.
* Allow partially blocked users: This option should always be true unless allow_banned_users is `false`. When allow_banned_users is `false`, set this to `false` if you want to disallow indefinitely blocked users who are only *partially* blocked to also be prevented from authenticating.


## Migrating from the old bot

If you have the old WikiAuthBot in your server already, you need to kick the old bot to prevent it from
conflicting with the new bot's command. Furthermore, introducing the new bot may cause people's data to be
lost (e.g. due to renames). To get a list of people that need to be reauthenticated, DM the bot `wab!unauthed_list guild_id`.

After notifying people who need to reauthenticate, use `/cleanup_roles` so that the bot removes the
authentiacted role from them.
