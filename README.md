# 🗡️ RuneTalk

A fantasy-themed real-time chat platform — think Discord, but in a world of guilds, adventurers, and ancient runes.

## 🖼️ UI Preview

![RuneTalk UI](./docs/screenshots/runetalk-ui.png)

## ✨ Features

- **Authentication** — Register & login with JWT, OTP verification via email (SMTP)
- **Guilds** — Create and manage servers with roles (owner, admin, member)
- **Rifts** — Text channels inside guilds with topic and ordering
- **Echoes** — Real-time messages in rifts with reply support
- **Scrolls & Whispers** — Private DM conversations between adventurers
- **Presence** — Online/offline/idle/dnd status with custom status
- **Real-time** — WebSocket for live messaging, SSE for event streaming
- **Frontend UI** — React + Tailwind mock-data interface for auth, guild chat, DMs, allies, modals, replies, pinned echoes, and presence states

## 🛠️ Tech Stack

| Layer | Technology |
|---|---|
| Language | Rust |
| Web Framework | Axum 0.8 |
| Frontend | React 18 + Vite + Tailwind CSS |
| Database | PostgreSQL 16 |
| Cache / Session | Redis |
| ORM / Migrations | SQLx 0.9 |
| Auth | JWT + Argon2 + OTP |
| Email | SMTP |
| Real-time | WebSocket + SSE |
| API | REST + GraphQL |
| Validation | validator |
| Logging | tracing + tracing-subscriber |

## 🗃️ Database Schema

| Table | Description |
|---|---|
| `adventurers` | User accounts |
| `guilds` | Servers / communities |
| `guild_members` | Server membership & roles |
| `rifts` | Text channels inside guilds |
| `echoes` | Messages inside rifts |
| `scrolls` | DM conversations between two adventurers |
| `whispers` | Messages inside a scroll (DM) |
| `presence` | Online status per adventurer |

## 🚀 Getting Started

### Prerequisites

- Rust (latest stable)
- PostgreSQL 16
- Redis
- SQLx CLI

```bash
cargo install sqlx-cli --no-default-features --features rustls,postgres
```

### Setup

```bash
# Clone the repo
git clone https://github.com/yourusername/RuneTalk.git
cd RuneTalk

# Set environment variables
export DATABASE_URL=postgres://user:password@localhost:5432/runetalk
export REDIS_URL=redis://localhost:6379
export JWT_SECRET=your_secret_here
export SMTP_HOST=smtp.example.com
export SMTP_USER=your@email.com
export SMTP_PASSWORD=your_password

# Run migrations
sqlx database create
sqlx migrate run

# Run the server
cargo run
```

### Frontend UI

```bash
cd frontend
npm install
npm run dev
```

The frontend currently uses schema-shaped mock data in `frontend/src/app/data/mock.ts`, so it can run before the backend API is fully wired.

### Docker (Database)

```bash
cd docker/postgresql
docker compose up -d
```

## 📖 Documentation

API docs tersedia di [`/docs`](./docs).

## 📜 License

MIT
