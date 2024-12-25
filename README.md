# Fern 🌿

A native discord client aimed to be fast, responsive and extensible. *(knocking on wood)*
Very WIP, you can risk getting banned if you try to use it.

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

### Why ?
I'm mostly making this out of curiosity...
Also because I think electron-based webapps are spawns of the devil and should be replaced

As for the name, the rendering library is called Xilem, so I just chose a vascular plant's name

### How ?
First and foremost, wrappers need to be created around Discord's API, a lot of it is not officially documented, some digging is needed.
After those are done, create UI elements and link them to the wrapper.

In the future, add wrappers around the app so that scripts and whatnot can expand its behaviour.

## Contributing
- Clone the repo
- Install the dependencies or nix to use the flake (much preferred)
- Want to submit your work ? Make a PR
