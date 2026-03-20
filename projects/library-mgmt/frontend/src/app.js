// Main Application Controller

const App = {
    currentView: 'dashboard',

    init() {
        this.setupNavigation();
        this.setupSearch();
        this.setupActions();
        this.checkServerHealth();
        this.loadView('dashboard');
    },

    setupNavigation() {
        document.querySelectorAll('.nav-item[data-view]').forEach(item => {
            item.addEventListener('click', (e) => {
                e.preventDefault();
                const view = item.dataset.view;
                this.navigateTo(view);
            });
        });
    },

    navigateTo(view) {
        // Update nav items
        document.querySelectorAll('.nav-item').forEach(el => el.classList.remove('active'));
        document.querySelector(`.nav-item[data-view="${view}"]`)?.classList.add('active');

        // Update views
        document.querySelectorAll('.view').forEach(el => el.classList.remove('active'));
        document.getElementById(`${view}View`)?.classList.add('active');

        this.currentView = view;
        this.loadView(view);
    },

    async loadView(view) {
        switch (view) {
            case 'dashboard':
                await Dashboard.load();
                break;
            case 'books':
                await Books.loadFilters();
                Books.setupListeners();
                await Books.load(1);
                break;
            case 'predictions':
                await Predictions.load();
                break;
            case 'analytics':
                await Analytics.load();
                break;
        }
    },

    setupSearch() {
        const searchBtn = document.getElementById('searchBtn');
        const searchInput = document.getElementById('globalSearch');

        searchBtn?.addEventListener('click', () => {
            if (this.currentView === 'books') {
                Books.load(1);
            } else {
                this.navigateTo('books');
            }
        });

        searchInput?.addEventListener('keydown', (e) => {
            if (e.key === 'Enter') {
                if (this.currentView === 'books') {
                    Books.load(1);
                } else {
                    this.navigateTo('books');
                }
            }
        });
    },

    setupActions() {
        document.getElementById('refreshBtn')?.addEventListener('click', () => {
            this.loadView(this.currentView);
        });

        document.getElementById('trainModelBtn')?.addEventListener('click', async () => {
            if (!confirm('Train the ML prediction model? This may take a moment.')) return;
            this.showLoading('Training ML model...');
            try {
                const result = await api.trainModel();
                alert(`Model trained successfully!\nAccuracy: ${(result.accuracy * 100).toFixed(1)}%\nBooks tagged: ${result.booksTagged}`);
                this.loadView(this.currentView);
            } catch (e) {
                alert('Training failed: ' + e.message);
            } finally {
                this.hideLoading();
            }
        });
    },

    async checkServerHealth() {
        const statusEl = document.getElementById('serverStatus');
        try {
            const health = await api.health();
            statusEl.innerHTML = `
                <span class="status-dot online"></span>
                <span>Server online — ${health.books || 0} books</span>`;
        } catch {
            statusEl.innerHTML = `
                <span class="status-dot offline"></span>
                <span>Server offline</span>`;
        }
    },

    showLoading(text = 'Loading...') {
        document.getElementById('loadingText').textContent = text;
        document.getElementById('loadingOverlay').classList.add('active');
    },

    hideLoading() {
        document.getElementById('loadingOverlay').classList.remove('active');
    }
};

// Boot
document.addEventListener('DOMContentLoaded', () => App.init());
