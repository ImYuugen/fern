# Fern 🌿

A native discord client aimed to be fast, responsive and extensible. *(knocking on wood)*
Very WIP, you can risk getting banned if you try to use it.

### Why ?
I'm mostly making this out of curiosity...
Also because I think electron-based webapps are spawns of the devil and should be replaced

I don't believe there exists a discord client written in Rust, not on my watch ! \
Also I like the idea of using IPC to enable the creation of custom overlays,
OBS plugins, and activities without too much hassle. \
Currently, you can only *send* data to discord using their IPC solution, I want it to go both ways.

As for the name, the rendering library is called Xilem, so I just chose a vascular plant's name
Also I like plants

### How ?
Before anything, we need to understand how Discord's APIs and Gateways work,
since it is not officially documented, some digging is needed.
Once this is done, create wrappers around said API to make development easier.
Next, write an IPC system to send/receive events, MAKE IT SECURE PUH-LEASE.
Then write the UI, badabim badaboom Fern is born.

**Features & TODOs**
✨ = UI Done
- [ ] Login
  - [x] Basic login
  - [ ] MFA
  - [ ] QR Code
- [ ] Sending and receiving messages
  - [ ] Simple text
  - [ ] Mentions
  - [ ] Emoji render
  - [ ] Stickers
  - [ ] Embeds
  - [ ] Images/Videos/Files
  - [ ] Reactions
  - [ ] Replies
- [ ] VoIP (Voice channels)
  - [ ] Joining voice channels
  - [ ] Voice transmit/receive
  - [ ] Camera transmit/receive
  - [ ] Streams
  - [ ] User volume
  - [ ] Stages
- [ ] Guilds
  - [ ] Join
  - [ ] Create
  - [ ] Leave
  - [ ] Settings
    - [ ] All the admin stuff
  - [ ] Folders
- [ ] Users
  - [ ] View profile card
  - [ ] Add/Remove friend
  - [ ] Send DM
  - [ ] Call
  - [ ] Block/Unblock
- [ ] Notifications

**Under the hood TODOs**
- [x] Maintain stable gateway connection
- [ ] Handle Dispatch
  - [x] READY
  - [x] READY_SUPPLEMENTAL
  - [ ] RESUMED
  - [ ] REMOTE_COMMAND
  - [ ] PRESENCE_UPDATE
  - [x] MESSAGE_{CREATE,UPDATE,DELETE}
  - [ ] CALL_{CREATE,UPDATE,DELETE}

## Contributing
- Clone the repo
- Install the dependencies or nix to use the flake (much preferred)
- Want to submit your work ? Make a PR
