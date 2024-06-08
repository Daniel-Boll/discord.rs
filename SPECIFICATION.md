## Technical Test: Real-Time Chat Application Development with Rust

### Introduction

In this challenge, you will be responsible for developing a real-time chat application using Rust. The application should offer basic functionalities similar to those found in chat platforms like Discord. The main focus is on creating a robust and scalable backend, using abstractions and best practices.

### Project Requirements

#### Core Features

1. **User Authentication**
   - User registration and login with authentication.
   - **Entities**: User
2. **Server Creation**
   - Users can create chat servers.
   - **Entities**: Guild
3. **Text Channels**
   - Each server can have multiple text channels.
   - Messages can be sent and received in real-time.
   - **Entities**: Channel
4. **Real-Time Messaging**
   - Implementation of real-time messaging using websockets.
   - **Entities**: Message
5. **User Profiles**
   - Each user has a profile.
   - Display basic profile information such as:
     - username
     - number of servers they own
     - number of messages sent
     - account creation date

### Application Routes

| Method | Route                                                       | Description                                     |
| ------ | ----------------------------------------------------------- | ----------------------------------------------- |
| GET    | /login                                                      | Login page                                      |
| GET    | /register                                                   | Registration page                               |
| POST   | /auth/register                                              | Create a user                                   |
| POST   | /auth/login                                                 | User login                                      |
| GET    | /user                                                       | Return the authenticated user                   |
| GET    | /guilds                                                     | Return all available servers                    |
| POST   | /guilds                                                     | Create a new server                             |
| DELETE | /guilds/{guildId}                                           | Delete a server                                 |
| GET    | /guilds/{guildId}                                           | List rooms of a server                          |
| POST   | /guilds/{guildId}/channels                                  | Create a new channel within a server            |
| GET    | /guilds/{guildId}/channels/{channelId}                      | Enter a channel and start listening to messages |
| POST   | /guilds/{guildId}/channels/{channelId}/messages             | Send a message to a specific channel            |
| DELETE | /guilds/{guildId}/channels/{channelId}/messages/{messageId} | Delete a message in a specific channel          |

### Technical Requirements

1. **Backend**

   - Rust
   - Axum
   - Database: ScyllaDB

2. **Frontend**

   - Leptos + TailwindCSS

3. **Documentation and Testing**
   - API documentation using Swagger or another documentation framework.
   - **BONUS:** Integration tests

### Evaluation Criteria

1. **Code Quality**
   - Code structure and organization.
   - Use of abstractions:
     - Service
     - Repositories
     - Jobs/Broadcast
2. **Functionality**
   - Complete implementation of the described functionalities.
3. **Documentation**
   - Clarity and completeness of the documentation.
   - Clear instructions for configuring and running the project.
4. **Innovation**
   - Additional features or improvements that were not specified but add value to the project.

### Submission

Once you start the project, commit the project base (framework) and create a new branch for your challenge. Open a Pull Request with the challenge and send the link to the email below.

- **Git Repository**: The code should be available in a public Git repository (GitHub, GitLab, etc.).
- **Instructions**: A README.md file with detailed instructions on how to configure and run the project.
- **Submission**: Send the code to [danielhe4rt@gmail.com](mailto:danielhe4rt@gmail.com) with the subject "Technical Test: Discord by (your name)"
- **Deadline**: Undefined!

Good luck and happy coding!! :D

### Support Materials

- [Article: How Discord Stores Trillions of Messages](https://discord.com/blog/how-discord-stores-trillions-of-messages)
- [Course: Laravel4Noobs](https://udemy.com/laravel4noobs)
- [Doc: SOLID4Noobs](https://github.com/danielhe4rt/solid4noobs)
- [Article: Creating Exceptions to Impress in the Technical Test](https://dev.to/he4rt/criando-exceptions-para-impressionar-no-teste-tecnico-2nie)
- [Article: SOLID Was What I Was Missing!](https://dev.to/clintonrocha98/era-solid-o-que-me-faltava-bhp)

## Roadmap for Version 2: Voice Channels Support

### Core Features for Version 2

1. **Voice Channels**

   - Implementation of voice channels within servers.
   - Users can join and leave voice channels.
   - Real-time voice communication between users in the same voice channel.
   - **Entities**: VoiceChannel

2. **Voice Communication Management**

   - Handling of voice data streaming using WebRTC.
   - Efficient management of voice channel bandwidth and quality.

3. **Voice Channel Moderation**

   - Permissions for muting, deafening, and kicking users from voice channels.
   - Admin controls for managing voice channel settings.

4. **User Presence**
   - Indication of user status (online, offline, in a voice channel).
   - Display current users in a voice channel.

### Additional Routes for Voice Channels

| Method | Route                                                           | Description                                |
| ------ | --------------------------------------------------------------- | ------------------------------------------ |
| POST   | /guilds/{guildId}/voice-channels                                | Create a new voice channel within a server |
| GET    | /guilds/{guildId}/voice-channels/{voiceChannelId}               | Enter a voice channel                      |
| POST   | /guilds/{guildId}/voice-channels/{voiceChannelId}/join          | Join a voice channel                       |
| POST   | /guilds/{guildId}/voice-channels/{voiceChannelId}/leave         | Leave a voice channel                      |
| POST   | /guilds/{guildId}/voice-channels/{voiceChannelId}/mute          | Mute a user in the voice channel           |
| POST   | /guilds/{guildId}/voice-channels/{voiceChannelId}/deafen        | Deafen a user in the voice channel         |
| DELETE | /guilds/{guildId}/voice-channels/{voiceChannelId}/kick/{userId} | Kick a user from the voice channel         |

### Technical Requirements for Version 2

1. **Backend**

   - Integration of WebRTC for voice data transmission.
   - Enhanced server infrastructure to support voice communication.

2. **Frontend**

   - User interface updates to support voice channel functionalities.
   - Integration with WebRTC for real-time voice streaming.

3. **Documentation and Testing**
   - API documentation updates to include voice channel functionalities.
   - Integration tests for voice communication features.

### Evaluation Criteria for Version 2

1. **Voice Channel Functionality**

   - Effective implementation of real-time voice communication.
   - Seamless integration with existing text channel functionalities.

2. **Performance and Scalability**

   - Efficient handling of voice data.
   - Scalability of voice channels to support multiple users.

3. **User Experience**
   - Intuitive user interface for voice channel management.
   - Smooth user experience in joining, leaving, and managing voice channels.
