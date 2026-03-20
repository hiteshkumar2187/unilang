// Analytics component

const Analytics = {
    async load() {
        try {
            const data = await api.dashboard();
            this.renderCategoryAnalytics(data.categoryStats);
        } catch (e) {
            console.error('Analytics load error:', e);
        }
    },

    renderCategoryAnalytics(stats) {
        const container = document.getElementById('categoryAnalyticsTable');
        if (!stats || Object.keys(stats).length === 0) {
            container.innerHTML = '<p style="color:var(--text-muted)">No analytics data available.</p>';
            return;
        }

        const entries = Object.entries(stats).sort((a, b) => b[1].count - a[1].count);
        const totalBooks = entries.reduce((sum, [, s]) => sum + s.count, 0);
        const totalCheckouts = entries.reduce((sum, [, s]) => sum + s.totalCheckouts, 0);

        const html = `<table class="data-table">
            <thead>
                <tr>
                    <th>Category</th>
                    <th>Books</th>
                    <th>% of Library</th>
                    <th>Total Checkouts</th>
                    <th>Avg Checkouts/Book</th>
                    <th>Avg Rating</th>
                    <th>Checkout Share</th>
                </tr>
            </thead>
            <tbody>
                ${entries.map(([cat, s]) => {
                    const pctLib = ((s.count / totalBooks) * 100).toFixed(1);
                    const avgCO = s.count > 0 ? (s.totalCheckouts / s.count).toFixed(1) : '0';
                    const coShare = totalCheckouts > 0 ? ((s.totalCheckouts / totalCheckouts) * 100).toFixed(1) : '0';
                    const avgRating = s.averageRating ? s.averageRating.toFixed(1) : '—';
                    const isHot = parseFloat(coShare) > parseFloat(pctLib) * 1.2;
                    return `<tr>
                        <td><strong>${cat}</strong></td>
                        <td>${s.count.toLocaleString()}</td>
                        <td>${pctLib}%</td>
                        <td>${s.totalCheckouts.toLocaleString()}</td>
                        <td>${avgCO}</td>
                        <td>${avgRating !== '—' ? `<span class="stars">${'★'.repeat(Math.round(parseFloat(avgRating)))}</span> ${avgRating}` : '—'}</td>
                        <td>
                            <span style="color:${isHot ? 'var(--accent-green)' : 'var(--text-secondary)'};font-weight:${isHot ? '600' : '400'}">
                                ${coShare}%${isHot ? ' ↑' : ''}
                            </span>
                        </td>
                    </tr>`;
                }).join('')}
            </tbody>
            <tfoot>
                <tr style="font-weight:600;border-top:2px solid var(--border)">
                    <td>Total</td>
                    <td>${totalBooks.toLocaleString()}</td>
                    <td>100%</td>
                    <td>${totalCheckouts.toLocaleString()}</td>
                    <td>${(totalCheckouts / totalBooks).toFixed(1)}</td>
                    <td></td>
                    <td>100%</td>
                </tr>
            </tfoot>
        </table>`;

        container.innerHTML = html;
    }
};
