# Library Management System — UniLang Edition

A full-stack library management application built entirely in **UniLang** — no Python, no Java, no Node.js. The HTTP server, REST API, business logic, and static file serving all run from a single UniLang script.

## Overview

This example demonstrates that a real, production-style web application can be built and run purely with UniLang. It includes:

- A **REST API** with 11 endpoints covering books, categories, languages, dashboard analytics, and ML-style predictions
- A **Single Page Application** frontend (vanilla JS) served directly by the UniLang server
- **50 books** in an in-memory store with full metadata: checkouts, ratings, late returns, prediction tags, and more
- A **dashboard** with top checked-out books, top-rated books, category stats, late-return hotspots, and prediction tag distribution

## Project Structure

```
library-mgmt/
├── backend/
│   └── api/
│       └── server.uniL          # Full HTTP server — the only file you need to run
├── frontend/
│   ├── public/
│   │   └── index.html           # SPA entry point
│   └── src/
│       ├── app.js               # App router
│       ├── styles/
│       │   └── main.css
│       ├── utils/
│       │   └── api.js           # API client (calls localhost:8080)
│       └── components/
│           ├── dashboard.js
│           ├── books.js
│           ├── predictions.js
│           └── analytics.js
└── unilang.toml                 # Project configuration
```

## Running the Server

From the **repository root**:

```bash
unilang run examples/library-mgmt/backend/api/server.uniL
```

Then open your browser at:

```
http://localhost:8080
```

The frontend SPA loads automatically. The API is available at `http://localhost:8080/api`.

## API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/api/health` | Server health and book counts |
| `GET` | `/api/dashboard` | Full dashboard analytics |
| `GET` | `/api/books` | List books (paginated, filterable) |
| `GET` | `/api/books/:id` | Get a single book |
| `POST` | `/api/books` | Add a new book |
| `PUT` | `/api/books/:id/checkout` | Check out a book |
| `PUT` | `/api/books/:id/return` | Return a book |
| `GET` | `/api/categories` | List all categories |
| `GET` | `/api/languages` | List all languages |
| `GET` | `/api/predictions/stats` | Prediction model stats |
| `GET` | `/api/predictions/books` | Books with prediction tags |

### Query Parameters for `GET /api/books`

| Parameter | Description | Example |
|-----------|-------------|---------|
| `page` | Page number (default: 1) | `?page=2` |
| `pageSize` | Items per page (default: 20) | `?pageSize=10` |
| `category` | Filter by category | `?category=Fiction` |
| `language` | Filter by language | `?language=English` |
| `status` | Filter by status | `?status=available` |
| `search` | Search title or author | `?search=tolkien` |

### Query Parameters for `GET /api/predictions/books`

| Parameter | Description | Example |
|-----------|-------------|---------|
| `tag` | Filter by prediction tag | `?tag=high_demand` |

Prediction tags: `popular`, `high_demand`, `normal`

## Dashboard Response Shape

The `/api/dashboard` endpoint returns:

```json
{
  "overview": {
    "totalBooks": 50,
    "availableBooks": 36,
    "checkedOutBooks": 14,
    "totalReviews": 2468,
    "averageRating": 4.51,
    "totalCheckouts": 5407
  },
  "topCheckedOut": [{ "title": "...", "author": "...", "checkouts": 240 }],
  "topRated":      [{ "title": "...", "author": "...", "rating": 4.9 }],
  "predictionTagDistribution": { "popular": 22, "high_demand": 10, "normal": 18 },
  "lateReturnHotspots": [{ "title": "...", "lateReturns": 22, "totalCheckouts": 240 }],
  "categoryStats": [{ "category": "Fiction", "available": 10, "checkedOut": 3, "total": 13 }]
}
```

## Book Data Model

Each book in the store has the following fields:

```
id, title, author, isbn, category, language, status,
totalCopies, availableCopies, pageCount, publishYear, publisher,
totalCheckouts, lateReturns, onTimeReturns,
averageRating, totalRatings, predictionTag
```

## How It Works

The server is a single UniLang script (`server.uniL`) that:

1. Defines 50 books as an in-memory list of dicts (globals persist across requests)
2. Implements helper functions for JSON responses and query string parsing
3. Defines one handler function per endpoint
4. Serves the frontend SPA's static files via `read_file()` and `file_exists()` builtins
5. Routes all requests through a `router(req)` function passed to `serve(8080, router)`

The built-in `serve()` function starts a TCP-based HTTP/1.1 server. Each incoming request is passed to the router as a dict:

```
{ "method": "GET", "path": "/api/books", "query": "page=1&pageSize=20", "headers": {...}, "body": "" }
```

The router returns a response dict:

```
{ "status": 200, "body": "...", "content_type": "application/json" }
```

CORS headers (`Access-Control-Allow-Origin: *`) are automatically added by the runtime.
