// ML Predictions component

const Predictions = {
    async load() {
        try {
            const stats = await api.getPredictionStats();
            this.renderModelStatus(stats);
            this.renderTagDistribution(stats.tagDistribution);
            this.renderRiskTable(stats.highRiskBooks || []);
        } catch (e) {
            console.error('Predictions load error:', e);
            document.getElementById('modelStatus').innerHTML =
                '<p style="color:var(--text-muted)">No model trained yet. Click "Train ML Model" to get started.</p>';
        }
    },

    renderModelStatus(stats) {
        const container = document.getElementById('modelStatus');
        if (!stats.modelTrained) {
            container.innerHTML = `
                <div style="text-align:center;padding:20px;color:var(--text-muted)">
                    <p style="font-size:40px;margin-bottom:12px">🤖</p>
                    <p>No model trained yet.</p>
                    <p style="margin-top:8px">Click <strong>"Train ML Model"</strong> in the header to train the prediction engine.</p>
                </div>`;
            return;
        }

        const metrics = [
            { label: 'Model Type', value: stats.modelType || 'Ensemble (RF + GB)' },
            { label: 'Training Samples', value: (stats.trainingSamples || 0).toLocaleString() },
            { label: 'Accuracy', value: stats.accuracy ? `${(stats.accuracy * 100).toFixed(1)}%` : '—' },
            { label: 'Cross-Val Score', value: stats.cvScore ? `${(stats.cvScore * 100).toFixed(1)}%` : '—' },
            { label: 'Books Tagged', value: (stats.booksTagged || 0).toLocaleString() },
            { label: 'Last Trained', value: stats.lastTrained || '—' },
        ];

        container.innerHTML = metrics.map(m => `
            <div class="metric-row">
                <span class="metric-label">${m.label}</span>
                <span class="metric-value">${m.value}</span>
            </div>
        `).join('');
    },

    renderTagDistribution(dist) {
        const container = document.getElementById('tagDistributionDetail');
        if (!dist || Object.keys(dist).length === 0) {
            container.innerHTML = '<p style="color:var(--text-muted)">No predictions available.</p>';
            return;
        }

        const total = Object.values(dist).reduce((a, b) => a + b, 0);
        const tagColors = {
            'most_likely_booked': { color: '#4f8cf7', bg: 'rgba(79,140,247,0.15)' },
            'late_return': { color: '#e85d5d', bg: 'rgba(232,93,93,0.15)' },
            'on_time_return': { color: '#4caf7d', bg: 'rgba(76,175,125,0.15)' },
            'less_likely_booked': { color: '#e8a847', bg: 'rgba(232,168,71,0.15)' },
            'untagged': { color: '#5c6078', bg: 'rgba(92,96,120,0.15)' },
        };

        const html = Object.entries(dist)
            .sort((a, b) => b[1] - a[1])
            .map(([tag, count]) => {
                const pct = ((count / total) * 100).toFixed(1);
                const tc = tagColors[tag] || tagColors['untagged'];
                return `
                    <div class="metric-row">
                        <span><span class="tag" style="background:${tc.bg};color:${tc.color}">${formatTag(tag)}</span></span>
                        <span class="metric-value">${count.toLocaleString()} <small style="color:var(--text-muted);font-weight:400">(${pct}%)</small></span>
                    </div>`;
            }).join('');

        container.innerHTML = html;
    },

    renderRiskTable(books) {
        const container = document.getElementById('riskTable');
        if (!books.length) {
            container.innerHTML = '<p style="color:var(--text-muted);padding:12px 0">No late return risk data available.</p>';
            return;
        }

        const html = `<table class="data-table">
            <thead>
                <tr>
                    <th>Title</th>
                    <th>Author</th>
                    <th>Category</th>
                    <th>Late Rate</th>
                    <th>Checkouts</th>
                    <th>Prediction</th>
                </tr>
            </thead>
            <tbody>
                ${books.map(b => `<tr>
                    <td><strong>${escapeHtml(b.title)}</strong></td>
                    <td>${escapeHtml(b.author)}</td>
                    <td><span class="tag">${b.category}</span></td>
                    <td><span style="color:var(--accent-red);font-weight:600">${(b.lateRate * 100).toFixed(1)}%</span></td>
                    <td>${b.checkouts}</td>
                    <td>${b.predictionTag ? `<span class="tag tag-${b.predictionTag}">${formatTag(b.predictionTag)}</span>` : '—'}</td>
                </tr>`).join('')}
            </tbody>
        </table>`;

        container.innerHTML = html;
    }
};
