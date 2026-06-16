# RuneTalk Backend API Documentation

Welcome to the unified RuneTalk Backend API documentation. This document covers general API guidelines, authentication requirements, REST endpoints, and real-time communications (GraphQL, WebSockets, and Server-Sent Events).

---

## 📖 Table of Contents
1. [Global Requirements & Conventions](#global-requirements--conventions)
2. [Authentication & Session API](#authentication--session-api)
3. [User & Friendship API](#user--friendship-api)
4. [Guilds & Rifts API](#guilds--rifts-api)
5. [Real-time, WebSockets, SSE & GraphQL API](#real-time-websockets-sse--graphql-api)

---

## Global Requirements & Conventions

### Base URL
The base API URL for local development:
```
http://localhost:8080
```

### 1. API Secret Header
All requests sent to the backend must include the `X-API-SECRET` header to verify the client application.
- **Header Key**: `X-API-SECRET`
- **Value**: Must match the `api.secret` value in the backend configurations (e.g., `api-secret`).

### 2. Cookie Authentication
For protected routes, authentication is handled via a secure HTTP-Only cookie.
- **Cookie Name**: `token`
- **Value**: A valid JSON Web Token (JWT) containing user claims.
- **Attributes**: `HttpOnly`, `SameSite=Lax`, `Path=/`.

### 3. Standard Response Formats
The API conforms to a standardized JSON response envelope.

#### Success Response
```json
{
  "success": true,
  "message": "Descriptive success message",
  "data": {}
}
```
*Note: `data` can be an object, an array, or `null` depending on the endpoint.*

#### Error Response
```json
{
  "success": false,
  "message": "Descriptive error message",
  "data": null
}
```
*Note: Validation errors will return `400 Bad Request` with validation failure messages in the `message` field.*

---

## Authentication & Session API

All auth endpoints are public but still require the global `X-API-SECRET` header.

### 1. Register Account
Creates a new pending account. An OTP verification email will be triggered.
- **Endpoint**: `POST /auth/register`
- **Request Body (JSON)**:
  ```json
  {
    "username": "johndoe",
    "email": "john.doe@example.com",
    "password": "supersecurepassword123"
  }
  ```
- **Validation**:
  - `username`: Minimum 4 characters.
  - `email`: Valid email format.
  - `password`: Minimum 8 characters.
- **Response (`201 Created`)**:
  ```json
  {
    "success": true,
    "message": "Berhasil mendaftar",
    "data": {}
  }
  ```

### 2. Verify OTP
Verifies the register/login OTP code sent via email.
- **Endpoint**: `POST /auth/verify`
- **Request Body (JSON)**:
  ```json
  {
    "email": "john.doe@example.com",
    "otp": "123456"
  }
  ```
- **Validation**:
  - `email`: Valid email format.
  - `otp`: Exactly 6 characters/digits.
- **Response (`200 OK`)**:
  ```json
  {
    "success": true,
    "message": "Berhasil memverifikasi OTP",
    "data": {}
  }
  ```

### 3. Resend OTP
Resends the verification OTP code to the specified email.
- **Endpoint**: `POST /auth/resend`
- **Request Body (JSON)**:
  ```json
  {
    "email": "john.doe@example.com"
  }
  ```
- **Response (`200 OK`)**:
  ```json
  {
    "success": true,
    "message": "Berhasil mengirim OTP",
    "data": {}
  }
  ```

### 4. Login
Authenticates a user and sets the session token cookie.
- **Endpoint**: `POST /auth/login`
- **Request Body (JSON)**:
  ```json
  {
    "identifier": "john.doe@example.com",
    "password": "supersecurepassword123"
  }
  ```
- **Response (`200 OK`)**:
  - **Headers**: `Set-Cookie: token=<jwt_token>; HttpOnly; SameSite=Lax; Path=/`
  - **Body**:
    ```json
    {
      "success": true,
      "message": "Berhasil Login",
      "data": {}
    }
    ```

### 5. Logout
Clears the session token by removing the cookie.
- **Endpoint**: `GET /auth/logout`
- **Response (`200 OK`)**:
  - **Headers**: `Set-Cookie: token=; Max-Age=0; Path=/`
  - **Body**:
    ```json
    {
      "success": true,
      "message": "Berhasil Logout",
      "data": {}
    }
    ```

---

## User & Friendship API

Protected endpoints. Require valid `token` cookie.

### 1. Get Current User Profile
- **Endpoint**: `GET /user/me`
- **Response (`200 OK`)**:
  ```json
  {
    "success": true,
    "message": "Berhasil Mengambil Profil",
    "data": {
      "id": "2781b2a9-7c85-48fa-89eb-2bf48beea8b4",
      "username": "johndoe",
      "email": "john.doe@example.com",
      "avatar_url": "/public/user/2781b2a9-7c85-48fa-89eb-2bf48beea8b4/avatar.png",
      "banner_url": null,
      "bio": "Hello, I am John!",
      "created_at": "2026-06-16T08:00:00",
      "updated_at": "2026-06-16T08:30:00"
    }
  }
  ```

### 2. Get User Profile by ID
- **Endpoint**: `GET /user/{id}`
- **Path Parameters**:
  - `id` (UUID): Target user ID.
- **Response (`200 OK`)**:
  ```json
  {
    "success": true,
    "message": "Berhasil Mengambil Profil",
    "data": {
      "id": "e9bd848b-3e5e-4efb-91cc-8ef3e4d9cfa2",
      "username": "janedoe",
      "email": "jane.doe@example.com",
      "avatar_url": null,
      "banner_url": null,
      "bio": "Jane's bio",
      "created_at": "2026-06-15T09:00:00",
      "updated_at": "2026-06-15T09:00:00"
    }
  }
  ```

### 3. Edit User Profile
Updates user profile information and handles avatar/banner image uploads.
- **Endpoint**: `PATCH /user/edit`
- **Content-Type**: `multipart/form-data`
- **Request Parameters (Multipart)**:
  - `display_name` (Text, Optional)
  - `bio` (Text, Optional)
  - `avatar` (File, Optional, Content-Type `image/*`)
  - `banner` (File, Optional, Content-Type `image/*`)
- **Response (`200 OK`)**:
  ```json
  {
    "success": true,
    "message": "Berhasil Mengubah Data",
    "data": {
      "username": "johndoe",
      "display_name": "John Doe",
      "email": "john.doe@example.com",
      "avatar_url": "./public/user/2781b2a9-7c85-48fa-89eb-2bf48beea8b4/avatar.png",
      "banner_url": "./public/user/2781b2a9-7c85-48fa-89eb-2bf48beea8b4/banner.jpg",
      "bio": "Updated bio content"
    }
  }
  ```

### 4. Send Friend Request
- **Endpoint**: `GET /user/add/{username}`
- **Path Parameters**:
  - `username` (String): The username of the recipient.
- **Response (`200 OK`)**:
  ```json
  {
    "success": true,
    "message": "Berhasil Mengirim Permintaan",
    "data": {}
  }
  ```
- **Triggers**: SSE event `friend_request_received` sent to recipient.

### 5. List Incoming Friend Requests
- **Endpoint**: `GET /user/requests`
- **Response (`200 OK`)**:
  ```json
  {
    "success": true,
    "message": "Berhasil Mengambil Permintaan Pertemanan",
    "data": [
      {
        "id": "52857416-83cf-4d9f-a2e6-a0de795328ad",
        "from_id": "e9bd848b-3e5e-4efb-91cc-8ef3e4d9cfa2",
        "username": "janedoe",
        "display_name": "Jane",
        "avatar_url": null,
        "created_at": "2026-06-16T09:12:00"
      }
    ]
  }
  ```

### 6. Accept Friend Request
- **Endpoint**: `GET /user/add/{id}/accept`
- **Path Parameters**:
  - `id` (UUID): Friend request sender's adventurer ID.
- **Response (`200 OK`)**:
  ```json
  {
    "success": true,
    "message": "Berhasil Menerima Permintaan",
    "data": {}
  }
  ```
- **Triggers**: SSE event `friend_request_accepted` sent to sender.

### 7. Reject Friend Request
- **Endpoint**: `GET /user/add/{id}/reject`
- **Path Parameters**:
  - `id` (UUID): Friend request sender's adventurer ID.
- **Response (`200 OK`)**:
  ```json
  {
    "success": true,
    "message": "Berhasil Menolak Permintaan",
    "data": {}
  }
  ```
- **Triggers**: SSE event `friend_request_rejected` sent to sender.

### 8. Block User
- **Endpoint**: `GET /user/add/{id}/block`
- **Path Parameters**:
  - `id` (UUID): Target user ID.
- **Response (`200 OK`)**:
  ```json
  {
    "success": true,
    "message": "Berhasil Memblokir Pengguna",
    "data": {}
  }
  ```

### 9. Check Friend/Ally Status
- **Endpoint**: `GET /user/ally/{id}`
- **Path Parameters**:
  - `id` (UUID): Other user's ID.
- **Response (`200 OK`)**:
  ```json
  {
    "success": true,
    "message": "Status pertemanan berhasil dicek",
    "data": {
      "is_ally": true
    }
  }
  ```

### 10. Remove Friend
- **Endpoint**: `DELETE /user/ally/{id}`
- **Path Parameters**:
  - `id` (UUID): Friend's user ID.
- **Response (`200 OK`)**:
  ```json
  {
    "success": true,
    "message": "Berhasil menghapus teman",
    "data": {}
  }
  ```
- **Triggers**: SSE event `friend_removed` sent to target friend.

---

## Guilds & Rifts API

Protected endpoints. Require valid `token` cookie.

### 1. Create Guild
- **Endpoint**: `POST /guild`
- **Request Body (JSON)**:
  ```json
  {
    "name": "The Fellowship",
    "description": "A place for adventurers.",
    "is_public": true
  }
  ```
- **Response (`200 OK`)**:
  ```json
  {
    "success": true,
    "message": "Berhasil Membuat Guild",
    "data": {
      "id": "a90ebfcd-e50a-429a-ab86-271d530869ef",
      "owner_id": "2781b2a9-7c85-48fa-89eb-2bf48beea8b4",
      "name": "The Fellowship",
      "description": "A place for adventurers.",
      "icon_url": null,
      "banner_url": null,
      "invite_code": "RF4G8A",
      "is_public": true,
      "created_at": "2026-06-16T09:15:00",
      "updated_at": "2026-06-16T09:15:00"
    }
  }
  ```

### 2. Get Guild Detail
- **Endpoint**: `GET /guild/{id}`
- **Path Parameters**:
  - `id` (UUID): Guild ID.
- **Response (`200 OK`)**:
  ```json
  {
    "success": true,
    "message": "Berhasil Mengambil Guild",
    "data": {
      "id": "a90ebfcd-e50a-429a-ab86-271d530869ef",
      "owner_id": "2781b2a9-7c85-48fa-89eb-2bf48beea8b4",
      "name": "The Fellowship",
      "description": "A place for adventurers.",
      "icon_url": null,
      "banner_url": null,
      "invite_code": "RF4G8A",
      "is_public": true,
      "created_at": "2026-06-16T09:15:00",
      "updated_at": "2026-06-16T09:15:00"
    }
  }
  ```

### 3. Edit Guild
Only guild owners/admins can perform this action.
- **Endpoint**: `PATCH /guild/{id}`
- **Path Parameters**:
  - `id` (UUID): Guild ID.
- **Request Body (JSON)**:
  ```json
  {
    "name": "The Fellowship Updated",
    "description": "A new description.",
    "icon_url": "http://example.com/icon.png",
    "banner_url": "http://example.com/banner.png",
    "is_public": false
  }
  ```
- **Response (`200 OK`)**:
  ```json
  {
    "success": true,
    "message": "Berhasil Mengubah Guild",
    "data": {
      "id": "a90ebfcd-e50a-429a-ab86-271d530869ef",
      "owner_id": "2781b2a9-7c85-48fa-89eb-2bf48beea8b4",
      "name": "The Fellowship Updated",
      "description": "A new description.",
      "icon_url": "http://example.com/icon.png",
      "banner_url": "http://example.com/banner.png",
      "invite_code": "RF4G8A",
      "is_public": false,
      "created_at": "2026-06-16T09:15:00",
      "updated_at": "2026-06-16T09:20:00"
    }
  }
  ```

### 4. Delete Guild
Only guild owners can perform this action.
- **Endpoint**: `DELETE /guild/{id}`
- **Path Parameters**:
  - `id` (UUID): Guild ID.
- **Response (`200 OK`)**:
  ```json
  {
    "success": true,
    "message": "Berhasil Menghapus Guild",
    "data": null
  }
  ```

### 5. Join Guild via Invite Code
- **Endpoint**: `POST /guild/join/{invite_code}`
- **Path Parameters**:
  - `invite_code` (String): The 6-character guild invite code.
- **Response (`200 OK`)**:
  ```json
  {
    "success": true,
    "message": "Berhasil Bergabung ke Guild",
    "data": {
      "id": "a90ebfcd-e50a-429a-ab86-271d530869ef",
      "name": "The Fellowship",
      "invite_code": "RF4G8A",
      "is_public": true
    }
  }
  ```

### 6. Get Guild Invite Link
- **Endpoint**: `GET /guild/{id}/invite`
- **Path Parameters**:
  - `id` (UUID): Guild ID.
- **Response (`200 OK`)**:
  ```json
  {
    "success": true,
    "message": "Berhasil Mengambil Invite Link",
    "data": {
      "invite_code": "RF4G8A",
      "invite_link": "http://localhost:5173/guild/join/RF4G8A"
    }
  }
  ```

### 7. Regenerate Guild Invite Link
Generates a new code and invalidates the previous one.
- **Endpoint**: `POST /guild/{id}/invite`
- **Path Parameters**:
  - `id` (UUID): Guild ID.
- **Response (`200 OK`)**:
  ```json
  {
    "success": true,
    "message": "Berhasil Mengambil Invite Link",
    "data": {
      "invite_code": "NEW789",
      "invite_link": "http://localhost:5173/guild/join/NEW789"
    }
  }
  ```

### 8. Create Rift (Channel)
- **Endpoint**: `POST /guild/{guild_id}/rift`
- **Path Parameters**:
  - `guild_id` (UUID): The parent Guild ID.
- **Request Body (JSON)**:
  ```json
  {
    "name": "general",
    "topic": "General chatroom.",
    "rift_type": "text",
    "is_private": false
  }
  ```
- **Validation**:
  - `name`: Minimum 3 characters.
  - `rift_type`: `"text"`, `"voice"`, or `"announcement"`. Defaults to `"text"`.
- **Response (`200 OK`)**:
  ```json
  {
    "success": true,
    "message": "Berhasil Membuat Rift",
    "data": {
      "id": "e8f7ad9c-4c6e-44e2-a083-d95bc2cf460f",
      "guild_id": "a90ebfcd-e50a-429a-ab86-271d530869ef",
      "name": "general",
      "topic": "General chatroom.",
      "type": "Text",
      "position": 0,
      "is_private": false,
      "created_at": "2026-06-16T09:25:00"
    }
  }
  ```

### 9. Edit Rift
- **Endpoint**: `PATCH /guild/{guild_id}/rift/{rift_id}`
- **Path Parameters**:
  - `guild_id` (UUID): Guild ID.
  - `rift_id` (UUID): Rift ID.
- **Request Body (JSON)**:
  ```json
  {
    "name": "general-chat",
    "topic": "New topic description.",
    "rift_type": "text",
    "position": 1,
    "is_private": true
  }
  ```
- **Response (`200 OK`)**:
  ```json
  {
    "success": true,
    "message": "Berhasil Mengubah Rift",
    "data": {
      "id": "e8f7ad9c-4c6e-44e2-a083-d95bc2cf460f",
      "name": "general-chat",
      "topic": "New topic description.",
      "type": "Text",
      "position": 1,
      "is_private": true
    }
  }
  ```

### 10. Delete Rift
- **Endpoint**: `DELETE /guild/{guild_id}/rift/{rift_id}`
- **Path Parameters**:
  - `guild_id` (UUID), `rift_id` (UUID).
- **Response (`200 OK`)**:
  ```json
  {
    "success": true,
    "message": "Berhasil Menghapus Rift",
    "data": null
  }
  ```

---

## Real-time, WebSockets, SSE & GraphQL API

Protected endpoints. Require valid `token` cookie.

### 1. GraphQL Endpoint
GraphQL is used for fetching complex batch relationships at startup.

- **Endpoint**: `GET /graphql` (Supports GET queries via query parameters, e.g. `?query=...`)
- **Query Document**:
  ```graphql
  query {
    myGuilds {
      id
      name
      description
      iconUrl
      bannerUrl
      isPublic
      members {
        id
        guildId
        adventurerId
        username
        displayName
        avatarUrl
        nickname
        role
      }
      rifts {
        id
        guildId
        name
        topic
        riftType
        position
        isPrivate
      }
    }
  }
  ```
- **Response Example**:
  ```json
  {
    "data": {
      "myGuilds": [
        {
          "id": "a90ebfcd-e50a-429a-ab86-271d530869ef",
          "name": "The Fellowship",
          "description": "A place for adventurers.",
          "iconUrl": null,
          "bannerUrl": null,
          "isPublic": true,
          "members": [
            {
              "id": "cb1cdeef-1234-4567-89ab-cdef01234567",
              "guildId": "a90ebfcd-e50a-429a-ab86-271d530869ef",
              "adventurerId": "2781b2a9-7c85-48fa-89eb-2bf48beea8b4",
              "username": "johndoe",
              "role": "owner"
            }
          ],
          "rifts": [
            {
              "id": "e8f7ad9c-4c6e-44e2-a083-d95bc2cf460f",
              "name": "general",
              "riftType": "Text",
              "position": 0,
              "isPrivate": false
            }
          ]
        }
      ]
    }
  }
  ```

---

### 2. Server-Sent Events (SSE)
Used for real-time one-way updates. Respond with `text/event-stream`.

#### A. Friends Stream (`GET /sse/friends`)
- **Initial Connection Event**:
  ```http
  event: connected
  data: {"user_id": "2781b2a9-7c85-48fa-89eb-2bf48beea8b4"}
  ```
- **Payload Events (Sent as JSON)**:
  - `friend_request_received`:
    ```json
    {
      "type": "friend_request_received",
      "data": {
        "id": "relationship_uuid",
        "from_id": "sender_uuid",
        "username": "janedoe",
        "display_name": "Jane",
        "avatar_url": null,
        "created_at": "datetime"
      }
    }
    ```
  - `friend_request_accepted`:
    ```json
    {
      "type": "friend_request_accepted",
      "user_id": "acceptor_uuid",
      "username": "janedoe",
      "display_name": null
    }
    ```
  - `friend_request_rejected`:
    ```json
    {
      "type": "friend_request_rejected",
      "user_id": "rejector_uuid"
    }
    ```
  - `friend_removed`:
    ```json
    {
      "type": "friend_removed",
      "user_id": "remover_uuid"
    }
    ```

#### B. Messages Stream (`GET /sse/messages`)
- **Initial Connection Event**:
  ```http
  event: connected
  data: {"user_id": "2781b2a9-7c85-48fa-89eb-2bf48beea8b4"}
  ```
- **Payload Events**:
  - `whisper_received` (Direct Message):
    ```json
    {
      "type": "whisper_received",
      "data": {
        "id": "msg_uuid",
        "scroll_id": "scroll_uuid",
        "sender_id": "sender_uuid",
        "reply_to_id": null,
        "content": "Secret message content...",
        "message_type": "text",
        "is_read": false,
        "created_at": "datetime"
      }
    }
    ```
  - `echo_received` (Channel Message):
    ```json
    {
      "type": "echo_received",
      "data": {
        "id": "msg_uuid",
        "rift_id": "rift_uuid",
        "adventurer_id": "sender_uuid",
        "reply_to_id": null,
        "content": "Hello world in channel!",
        "message_type": "text",
        "is_pinned": false,
        "created_at": "datetime"
      }
    }
    ```

---

### 3. WebSocket API (`GET /ws`)
Establishes a real-time bidirectional connection. Requires `token` cookie during handshake.

#### Client-to-Server Messages
Must contain a `"type"` field.

##### A. Subscribe to Rift (Channel)
```json
{
  "type": "subscribe_rift",
  "rift_id": "e8f7ad9c-4c6e-44e2-a083-d95bc2cf460f"
}
```
##### B. Unsubscribe from Rift
```json
{
  "type": "unsubscribe_rift",
  "rift_id": "e8f7ad9c-4c6e-44e2-a083-d95bc2cf460f"
}
```
##### C. Subscribe to Scroll (Direct Message Chat)
```json
{
  "type": "subscribe_scroll",
  "scroll_id": "550e8400-e29b-41d4-a716-446655440000"
}
```
##### D. Unsubscribe from Scroll
```json
{
  "type": "unsubscribe_scroll",
  "scroll_id": "550e8400-e29b-41d4-a716-446655440000"
}
```
##### E. Send Echo (Channel Message)
```json
{
  "type": "send_echo",
  "rift_id": "e8f7ad9c-4c6e-44e2-a083-d95bc2cf460f",
  "content": "Hello friends!",
  "reply_to_id": null,
  "message_type": "text"
}
```
##### F. Send Whisper (Direct Message)
```json
{
  "type": "send_whisper",
  "scroll_id": "550e8400-e29b-41d4-a716-446655440000",
  "content": "Hey, this is between us.",
  "reply_to_id": null,
  "message_type": "text"
}
```

#### Server-to-Client Messages

##### A. Subscribed/Unsubscribed Confirmations
```json
{ "type": "subscribed_rift", "rift_id": "e8f7ad9c-4c6e-44e2-a083-d95bc2cf460f" }
{ "type": "subscribed_scroll", "scroll_id": "550e8400-e29b-41d4-a716-446655440000" }
```
##### B. New Echo Message (Channel Message Received)
```json
{
  "type": "echo",
  "data": {
    "id": "message_id_uuid",
    "rift_id": "rift_id_uuid",
    "adventurer_id": "sender_id_uuid",
    "content": "Hello friends!",
    "message_type": "text",
    "is_pinned": false,
    "created_at": "datetime"
  }
}
```
##### C. New Whisper Message (Direct Message Received)
```json
{
  "type": "whisper",
  "data": {
    "id": "message_id_uuid",
    "scroll_id": "scroll_id_uuid",
    "sender_id": "sender_id_uuid",
    "content": "Hey, this is between us.",
    "message_type": "text",
    "is_read": false,
    "created_at": "datetime"
  }
}
```
##### D. Presence Snapshot (Sent initially on connection)
```json
{
  "type": "presence_snapshot",
  "online_users": [
    "e9bd848b-3e5e-4efb-91cc-8ef3e4d9cfa2",
    "2781b2a9-7c85-48fa-89eb-2bf48beea8b4"
  ]
}
```
##### E. Presence Update (Broadcasted status change)
```json
{
  "type": "presence_update",
  "user_id": "e9bd848b-3e5e-4efb-91cc-8ef3e4d9cfa2",
  "status": "online"
}
```
*(Presence status values: `"online"`, `"idle"`, `"dnd"`, `"offline"`)*

##### F. Error Payload
```json
{
  "type": "error",
  "message": "Pesan tidak boleh kosong"
}
```
