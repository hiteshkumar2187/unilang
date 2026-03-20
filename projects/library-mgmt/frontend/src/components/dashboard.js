// Dashboard component

const CHART_COLORS = [
    '#4f8cf7', '#4caf7d', '#e8a847', '#e85d5d', '#9b6dd7',
    '#4fc3c5', '#f06292', '#aed581', '#ffb74d', '#90a4ae',
    '#ce93d8', '#80cbc4', '#fff176', '#a1887f', '#b0bec5'
];

const Dashboard = {
    async load() {
        try {
            const data = await api.dashboard();
            this.renderStats(data.overview);
            this.renderTopCheckedOut(data.topCheckedOut);
            this.renderTopRated(data.topRated);
            this.renderPredictionChart(data.predictionTagDistribution);
            this.renderLateReturns(data.lateReturnHotspots);
            this.renderCategoryChart(data.categoryStats);
        } catch (e) {
            console.error('Dashboard load error:', e);
        }
    },

    renderStats(o) {
        document.getElementById('totalBooks').textContent = o.totalBooks.toLocaleString();
        document.getElementById('availableBooks').textContent = o.availableBooks.toLocaleString();
        document.getElementById('checkedOutBooks').textContent = o.checkedOutBooks.toLocaleString();
        document.getElementById('totalReviews').textContent = o.totalReviews.toLocaleString();
        document.getElementById('avgRating').textContent = `${o.averageRating} ★`;
        document.getElementById('totalCheckouts').textContent = o.totalCheckouts.toLocaleString();
    },

    renderTopCheckedOut(books) {
        const html = `<table class="mini-table">
            ${books.map((b, i) => `<tr>
                <td style="width:30px;color:var(--text-muted)">${i + 1}</td>
                <td><strong>${escapeHtml(b.title)}</strong><br><small style="color:var(--text-secondary)">${escapeHtml(b.author)}</small></td>
                <td style="text-align:right"><span style="color:var(--accent-blue);font-weight:600">${b.checkouts}</span></td>
            </tr>`).join('')}
        </table>`;
        document.getElementById('topCheckedOutTable').innerHTML = html;
    },

    renderTopRated(books) {
        const html = `<table class="mini-table">
            ${books.map((b, i) => `<tr>
                <td style="width:30px;color:var(--text-muted)">${i + 1}</td>
                <td><strong>${escapeHtml(b.title)}</strong><br><small style="color:var(--text-secondary)">${escapeHtml(b.author)}</small></td>
                <td style="text-align:right"><span class="stars">${'★'.repeat(Math.round(b.rating))}</span> ${b.rating}</td>
            </tr>`).join('')}
        </table>`;
        document.getElementById('topRatedTable').innerHTML = html;
    },

    renderPredictionChart(dist) {
        if (!dist) {
            document.getElementById('predictionChart').innerHTML = '<p style="color:var(--text-muted)">No predictions yet. Train the model first.</p>';
            return;
        }
        const total = Object.values(dist).reduce((a, b) => a + b, 0);
        const tagColors = {
            'most_likely_booked': '#4f8cf7',
            'late_return': '#e85d5d',
            'on_time_return': '#4caf7d',
            'less_likely_booked': '#e8a847',
            'untagged': '#5c6078',
        };
        const html = Object.entries(dist).map(([tag, count]) => {
            const pct = ((count / total) * 100).toFixed(1);
            const color = tagColors[tag] || '#5c6078';
            return `<div class="bar-row">
                <span class="bar-label">${formatTag(tag)}</span>
                <div class="bar-track"><div class="bar-fill" style="width:${pct}%;background:${color}">${pct}%</div></div>
                <span class="bar-value">${count.toLocaleString()}</span>
            </div>`;
        }).join('');
        document.getElementById('predictionChart').innerHTML = html;
    },

    renderLateReturns(books) {
        const html = `<table class="mini-table">
            ${books.map(b => `<tr>
                <td><strong>${escapeHtml(b.title)}</strong></td>
                <td style="text-align:right;color:var(--accent-red)">${(b.lateRate * 100).toFixed(1)}%</td>
                <td style="text-align:right;color:var(--text-secondary)">${b.checkouts} checkouts</td>
            </tr>`).join('')}
        </table>`;
        document.getElementById('lateReturnTable').innerHTML = html;
    },

    renderCategoryChart(stats) {
        const entries = Object.entries(stats).sort((a, b) => b[1].count - a[1].count);
        const maxCount = entries[0]?.[1].count || 1;
        const html = entries.map(([cat, s], i) => {
            const pct = ((s.count / maxCount) * 100).toFixed(1);
            const color = CHART_COLORS[i % CHART_COLORS.length];
            return `<div class="bar-row">
                <span class="bar-label">${cat}</span>
                <div class="bar-track"><div class="bar-fill" style="width:${pct}%;background:${color}">${s.count}</div></div>
                <span class="bar-value">${s.totalCheckouts.toLocaleString()} co</span>
            </div>`;
        }).join('');
        document.getElementById('categoryChart').innerHTML = html;
    }
};

// Helpers
function escapeHtml(str) {
    const div = document.createElement('div');
    div.textContent = str;
    return div.innerHTML;
}

function formatTag(tag) {
    return tag.replace(/_/g, ' ').replace(/\b\w/g, c => c.toUpperCase());
}
