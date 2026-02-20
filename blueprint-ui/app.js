// MuninOS Visual Interface - JavaScript

class BlueprintUI {
    constructor() {
        this.canvas = document.getElementById('blueprint-canvas');
        this.ctx = this.canvas.getContext('2d');
        this.socket = null;
        this.thoughtBubble = document.getElementById('thought-bubble');
        this.thoughtText = document.getElementById('thought-text');
        this.resultGrid = document.getElementById('result-grid');
        this.historyList = document.getElementById('history-list');
        this.voiceStatus = document.getElementById('voice-status');
        this.systemStatus = document.getElementById('system-status');

        this.init();
    }

    init() {
        this.resizeCanvas();
        this.connectWebSocket();
        this.startAnimation();
        this.updateTime();

        setInterval(() => this.updateTime(), 1000);
    }

    resizeCanvas() {
        const rect = this.canvas.parentElement.getBoundingClientRect();
        this.canvas.width = rect.width;
        this.canvas.height = 400;
    }

    connectWebSocket() {
        // Connect to MuninOS core via WebSocket
        const wsUrl = `ws://${window.location.host}/ws`;
        try {
            this.socket = new WebSocket('ws://localhost:8765');

            this.socket.onopen = () => {
                this.updateStatus('connected');
                this.systemStatus.textContent = '● System Ready';
                this.systemStatus.className = 'status-ok';
            };

            this.socket.onmessage = (event) => {
                const data = JSON.parse(event.data);
                this.handleMessage(data);
            };

            this.socket.onerror = () => {
                this.updateStatus('disconnected');
            };
        } catch (e) {
            console.log('WebSocket not available - running in demo mode');
        }
    }

    handleMessage(data) {
        switch (data.type) {
            case 'thought':
                this.showThought(data.content);
                break;
            case 'result':
                this.showResults(data.content);
                break;
            case 'status':
                this.updateStatus(data.status);
                break;
            case 'command':
                this.addToHistory(data.command, data.response);
                break;
        }
    }

    showThought(text) {
        this.thoughtBubble.classList.remove('hidden');
        this.thoughtText.textContent = text;

        // Simulate typing
        this.thoughtText.style.opacity = 0;
        let opacity = 0;
        const fadeIn = setInterval(() => {
            opacity += 0.1;
            this.thoughtText.style.opacity = Math.min(opacity, 1);
            if (opacity >= 1) clearInterval(fadeIn);
        }, 50);

        setTimeout(() => {
            this.thoughtBubble.classList.add('hidden');
        }, 5000);
    }

    showResults(results) {
        this.resultGrid.innerHTML = '';
        this.resultGrid.classList.remove('hidden');

        results.forEach(result => {
            const card = document.createElement('div');
            card.className = 'result-card';
            card.innerHTML = `
                <div style="font-weight: 600; margin-bottom: 0.5rem;">${result.title}</div>
                <div style="color: #888; font-size: 0.85rem;">${result.subtitle}</div>
            `;
            this.resultGrid.appendChild(card);
        });

        setTimeout(() => {
            this.resultGrid.classList.add('hidden');
        }, 10000);
    }

    addToHistory(command, response) {
        const item = document.createElement('div');
        item.className = 'history-item';
        item.innerHTML = `
            <div>
                <div>${command}</div>
                <div style="color: #00ff88; font-size: 0.85rem; margin-top: 0.25rem;">${response || ''}</div>
            </div>
            <div class="history-time">${new Date().toLocaleTimeString()}</div>
        `;
        this.historyList.insertBefore(item, this.historyList.firstChild);
    }

    updateStatus(status) {
        switch (status) {
            case 'listening':
                this.voiceStatus.textContent = '🎤 Listening...';
                break;
            case 'thinking':
                this.voiceStatus.textContent = '🤔 Thinking';
                this.systemStatus.textContent = '● Processing';
                this.systemStatus.className = 'status-thinking';
                break;
            case 'connected':
                this.voiceStatus.textContent = '🟢 Connected';
                break;
        }
    }

    updateTime() {
        const time = document.getElementById('time');
        time.textContent = new Date().toLocaleTimeString();
    }

    // Animation loop for visual effects
    startAnimation() {
        let time = 0;

        const animate = () => {
            this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);

            // Draw ambient background
            this.drawAmbientBackground(time);

            // Draw connection nodes
            this.drawConnectionNodes(time);

            requestAnimationFrame(animate);
            time += 0.016;
        };

        animate();
    }

    drawAmbientBackground(time) {
        const gradient = this.ctx.createRadialGradient(
            this.canvas.width / 2, this.canvas.height / 2, 0,
            this.canvas.width / 2, this.canvas.height / 2, this.canvas.width / 2
        );
        gradient.addColorStop(0, 'rgba(0, 255, 136, 0.03)');
        gradient.addColorStop(1, 'rgba(0, 0, 0, 0)');

        this.ctx.fillStyle = gradient;
        this.ctx.fillRect(0, 0, this.canvas.width, this.canvas.height);
    }

    drawConnectionNodes(time) {
        const nodes = [
            { x: this.canvas.width * 0.2, y: this.canvas.height * 0.3 },
            { x: this.canvas.width * 0.5, y: this.canvas.height * 0.4 },
            { x: this.canvas.width * 0.8, y: this.canvas.height * 0.3 },
            { x: this.canvas.width * 0.35, y: this.canvas.height * 0.7 },
            { x: this.canvas.width * 0.65, y: this.canvas.height * 0.7 },
        ];

        // Draw connections
        this.ctx.strokeStyle = 'rgba(0, 255, 136, 0.1)';
        this.ctx.lineWidth = 1;

        for (let i = 0; i < nodes.length; i++) {
            for (let j = i + 1; j < nodes.length; j++) {
                this.ctx.beginPath();
                this.ctx.moveTo(nodes[i].x, nodes[i].y);
                this.ctx.lineTo(nodes[j].x, nodes[j].y);
                this.ctx.stroke();
            }
        }

        // Draw nodes
        nodes.forEach((node, i) => {
            const x = node.x + Math.sin(time * 2 + i) * 10;
            const y = node.y + Math.cos(time * 1.5 + i) * 10;

            this.ctx.beginPath();
            this.ctx.arc(x, y, 4, 0, Math.PI * 2);
            this.ctx.fillStyle = '#00ff88';
            this.ctx.fill();

            // Glow effect
            this.ctx.beginPath();
            this.ctx.arc(x, y, 8, 0, Math.PI * 2);
            this.ctx.fillStyle = 'rgba(0, 255, 136, 0.2)';
            this.ctx.fill();
        });
    }
}

// Initialize UI when DOM is ready
document.addEventListener('DOMContentLoaded', () => {
    window.blueprintUI = new BlueprintUI();
});
