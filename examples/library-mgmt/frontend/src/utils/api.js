// API Client for Library Management System

const API_BASE = 'http://localhost:8080/api';

const api = {
    async get(endpoint, params = {}) {
        const url = new URL(`${API_BASE}${endpoint}`);
        Object.entries(params).forEach(([k, v]) => {
            if (v !== null && v !== undefined && v !== '') url.searchParams.set(k, v);
        });
        const res = await fetch(url);
        if (!res.ok) throw new Error(`API error: ${res.status}`);
        return res.json();
    },

    async post(endpoint, body = {}) {
        const res = await fetch(`${API_BASE}${endpoint}`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(body),
        });
        if (!res.ok) throw new Error(`API error: ${res.status}`);
        return res.json();
    },

    // Health check
    health() { return this.get('/health'); },

    // Dashboard
    dashboard() { return this.get('/dashboard'); },

    // Books
    getBooks(params) { return this.get('/books', params); },
    getBook(id) { return this.get(`/books/${id}`); },

    // Metadata
    getCategories() { return this.get('/categories'); },
    getLanguages() { return this.get('/languages'); },

    // Predictions
    getPredictionStats() { return this.get('/predictions/stats'); },
    getPredictionBooks(tag, page = 1) {
        return this.get('/predictions/books', { tag, page });
    },

    // Admin
    generateData() { return this.post('/admin/generate-data'); },
    trainModel() { return this.post('/admin/train-model'); },
};
