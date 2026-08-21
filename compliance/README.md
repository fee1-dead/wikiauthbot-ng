## Application Details
### What does your application do? Please be as detailed as possible, and feel free to include links to image or video examples.

This bot connects to Wikipedia via OAuth, to connect the identity of a Discord user to their username on Wikipedia.

See https://github.com/fee1-dead/wikiauthbot-ng/tree/main/compliance/s1.png for an example screenshot.

Information about the Wikipedia account associated with a Discord user could be obtained using the /whois command.
Information about the Discord account associated with a Wikipedia username could be obtained using the /revwhois command.

### Do you have a public Privacy Policy telling your users about their data usage?

Yes

### Where is your Privacy Policy available?

Whenever the user is prompted to authenticate while using the /auth command, they are given a link to the privacy statement.

### Please share a link to your Privacy Policy.

https://wikiauthbot-ng.toolforge.org/ps

### Privileged Gateway Intents

- [x] Server Members Intent
- [ ] Presence Intent
- [ ] Message Content Intent

## Server Members Intent

### Why do you need the Guild Members intent?

The bot sends a welcome message whenever a member joins. Note that the only data being stored by the bot is the discord user ID and the Wikimedia ID (which they gave by authenticating into Wikimedia OAuth).

### Please provide links to screenshots and/or videos that demonstrate your use case

See https://github.com/fee1-dead/wikiauthbot-ng/tree/main/compliance/s1.png.

### Are you storing any API Data off-platform (outside of Discord)?

Yes.

### Are you storing API Data for 30 days or less?

No.

### How do users contact you to request deletion of their activity data?

The /deauth command clears the account association data.

### Are you encrypting the data that you store at rest, as is required by our developer policy?

No.