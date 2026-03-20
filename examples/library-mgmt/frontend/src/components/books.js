// Books catalog component

const Books = {
    currentPage: 1,
    pageSize: 50,

    async load(page = 1) {
        this.currentPage = page;
        const params = {
            page,
            pageSize: this.pageSize,
            q: document.getElementById('globalSearch')?.value || null,
            category: document.getElementById('categoryFilter')?.value || null,
            status: document.getElementById('statusFilter')?.value || null,
            predictionTag: document.getElementById('tagFilter')?.value || null,
            language: document.getElementById('languageFilter')?.value || null,
            sortBy: document.getElementById('sortBy')?.value || 'title',
            sortOrder: document.getElementById('sortOrder')?.value || 'asc',
        };

        try {
            const data = await api.getBooks(params);
            this.renderTable(data.books);
            this.renderPagination(data);
        } catch (e) {
            console.error('Books load error:', e);
        }
    },

    async loadFilters() {
        try {
            const [categories, languages] = await Promise.all([
                api.getCategories(),
                api.getLanguages(),
            ]);
            const catSelect = document.getElementById('categoryFilter');
            categories.forEach(c => {
                const opt = document.createElement('option');
                opt.value = c;
                opt.textContent = c;
                catSelect.appendChild(opt);
            });
            const langSelect = document.getElementById('languageFilter');
            languages.forEach(l => {
                const opt = document.createElement('option');
                opt.value = l;
                opt.textContent = l;
                langSelect.appendChild(opt);
            });
        } catch (e) {
            console.error('Filter load error:', e);
        }
    },

    renderTable(books) {
        const tbody = document.getElementById('booksTableBody');
        if (!books.length) {
            tbody.innerHTML = '<tr><td colspan="8" style="text-align:center;color:var(--text-muted);padding:40px">No books found</td></tr>';
            return;
        }
        tbody.innerHTML = books.map(b => `<tr>
            <td>
                <strong>${escapeHtml(b.title)}</strong>
                <br><small style="color:var(--text-muted)">${b.isbn || ''}</small>
            </td>
            <td>${escapeHtml(b.author)}</td>
            <td><span class="tag">${b.category}</span></td>
            <td>
                ${b.averageRating > 0 ? `<span class="stars">${'★'.repeat(Math.round(b.averageRating))}</span> ${b.averageRating}` : '<span style="color:var(--text-muted)">—</span>'}
                ${b.totalRatings > 0 ? `<br><small style="color:var(--text-muted)">${b.totalRatings} reviews</small>` : ''}
            </td>
            <td>${b.totalCheckouts}</td>
            <td><span class="tag tag-${b.status}">${formatStatus(b.status)}</span></td>
            <td>${b.predictionTag ? `<span class="tag tag-${b.predictionTag}">${formatTag(b.predictionTag)}</span>` : '<span style="color:var(--text-muted)">—</span>'}</td>
            <td>${b.lateReturnRate !== undefined ? `<span style="color:${b.lateReturnRate > 0.25 ? 'var(--accent-red)' : 'var(--text-secondary)'}">${(b.lateReturnRate * 100).toFixed(1)}%</span>` : '—'}</td>
        </tr>`).join('');
    },

    renderPagination(data) {
        const container = document.getElementById('pagination');
        const { page, totalPages, total } = data;
        if (totalPages <= 1) {
            container.innerHTML = `<span style="color:var(--text-muted);font-size:13px">${total} books</span>`;
            return;
        }

        let html = `<span style="color:var(--text-muted);font-size:13px;margin-right:12px">${total.toLocaleString()} books</span>`;
        html += `<button class="page-btn" onclick="Books.load(1)" ${page === 1 ? 'disabled' : ''}>«</button>`;
        html += `<button class="page-btn" onclick="Books.load(${page - 1})" ${page === 1 ? 'disabled' : ''}>‹</button>`;

        const start = Math.max(1, page - 2);
        const end = Math.min(totalPages, page + 2);
        for (let i = start; i <= end; i++) {
            html += `<button class="page-btn ${i === page ? 'active' : ''}" onclick="Books.load(${i})">${i}</button>`;
        }

        html += `<button class="page-btn" onclick="Books.load(${page + 1})" ${page === totalPages ? 'disabled' : ''}>›</button>`;
        html += `<button class="page-btn" onclick="Books.load(${totalPages})" ${page === totalPages ? 'disabled' : ''}>»</button>`;
        container.innerHTML = html;
    },

    setupListeners() {
        ['categoryFilter', 'statusFilter', 'tagFilter', 'languageFilter', 'sortBy', 'sortOrder'].forEach(id => {
            document.getElementById(id)?.addEventListener('change', () => this.load(1));
        });
    }
};

function formatStatus(s) {
    return s.replace(/_/g, ' ').replace(/\b\w/g, c => c.toUpperCase());
}
