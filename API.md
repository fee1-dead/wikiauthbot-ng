## API interface

The bot has an API at https://wikiauthbot-ng.toolforge.org/whois/. The Discord ID should be a part of the URL and the
API key should be given as the "Authorization" header value.

Use the following command to call it. 

```
curl "https://wikiauthbot-ng.toolforge.org/whois/{discord_ID}" --verbose -H "authorization: {API_KEY}"
```

API keys are bound to a specific Discord server. You must be or obtain the authorization of a server's owner to create
an API key. Contact dbeef, send the invite link and the API key will be created for the server.

If the API call is successful it will return 200 OK. The content of the response determines the WHOIS result. If a user is found,
the content will be an unsigned 32bit integer representing the user's CentralAuth (globaluser) ID. If not, the content will be
"not found".
