const apps = [
    // Creative Application
    { name: "Flatseal", category: "Creative Application", format: "Flatpak", desc: "Flatpak App Managed Software" },
    { name: "ytDownloder", category: "Creative Application", format: "Flatpak", desc: "Video Downloader Software" },
    { name: "Packet", category: "Creative Application", format: "Flatpak", desc: "Quick share For Linux" },
    { name: "Inkscape", category: "Creative Application", format: "Flatpak", desc: "Vector Image Editor" },
    { name: "VLC", category: "Creative Application", format: "Flatpak", desc: "Video Player Software" },
    { name: "Upscayl", category: "Creative Application", format: "Flatpak", desc: "Image Upscaling Software" },
    { name: "Pinta", category: "Creative Application", format: "Flatpak", desc: "General Image Editor" },
    { name: "Discord", category: "Creative Application", format: "Flatpak", desc: "Social Media / Voice Chat" },
    { name: "Pods", category: "Creative Application", format: "Flatpak", desc: "Containers Manager" },
    { name: "HandBrake", category: "Creative Application", format: "Flatpak", desc: "Video Compressor" },
    { name: "OBS Studio", category: "Creative Application", format: "Flatpak", desc: "Video Recorder & Streamer" },
    { name: "Valot", category: "Creative Application", format: "Flatpak", desc: "Note & Task tracking with alarm" },
    { name: "Collector", category: "Creative Application", format: "Flatpak", desc: "Drag and drop everything in one place" },
    { name: "Gitte", category: "Creative Application", format: "Flatpak", desc: "Git Client Desktop Software" },
    { name: "Kdenlive", category: "Creative Application", format: "Flatpak", desc: "Video Editor Software" },
    { name: "Bazaar", category: "Creative Application", format: "Flatpak", desc: "App store for Flatpak Applications" },
    { name: "Akizip", category: "Creative Application", format: "Flatpak", desc: "Archive manager (7z, ZIP, TAR)" },
    { name: "BudsLink", category: "Creative Application", format: "Flatpak", desc: "Air buds Control for Linux" },
    { name: "Emojify", category: "Creative Application", format: "Flatpak", desc: "Emoji finder" },
    { name: "Xournal++", category: "Creative Application", format: "Flatpak", desc: "Digital notebook / PDF Annotator" },
    { name: "Drawy", category: "Creative Application", format: "Flatpak", desc: "Draw notebook" },
    { name: "Gradia", category: "Creative Application", format: "Flatpak", desc: "Screenshot Utility" },
    { name: "Scribus", category: "Creative Application", format: "Flatpak", desc: "Vector Image Print / Desktop Publishing" },

    // Dual boot Manager
    { name: "rEFInd", category: "Tools", format: "deb", desc: "Dual boot Manager" },

    // Development IDE
    { name: "VS Code", category: "Development IDE", format: "deb", desc: "Powerful code editor by Microsoft" },
    { name: "Qoder", category: "Development IDE", format: "deb", desc: "Code Editor" },
    { name: "Antigravity", category: "Development IDE", format: "dev", desc: "Advanced Agentic Coding Environment" },
    { name: "Zed", category: "Development IDE", format: "Flatpak", desc: "High-performance multiplayer code editor" },
    { name: "VsCodium", category: "Development IDE", format: "Flatpak", desc: "Telemetry-free VS Code build" },

    // Browser
    { name: "Chrome", category: "Browser", format: "deb", desc: "Google Web Browser" },
    { name: "Brave", category: "Browser", format: "Flatpak", desc: "Privacy-focused Web Browser" },

    // Tools
    { name: "Zram", category: "Tools", format: "deb", desc: "Memory compression in RAM" },
    { name: "Fzf", category: "Tools", format: "deb", desc: "Command-line fuzzy finder" },
    { name: "ls-sensor", category: "Tools", format: "deb", desc: "Hardware sensor monitoring" },

    // Dev Tools
    { name: "Git", category: "Dev Tools", format: "deb", desc: "Version Control System" },
    { name: "Node.js", category: "Dev Tools", format: "deb", desc: "JavaScript runtime built on V8" },
    { name: "Bun", category: "Dev Tools", format: "deb", desc: "Fast all-in-one JavaScript runtime" },
    { name: "curl", category: "Dev Tools", format: "deb", desc: "Command line tool for transferring data" },
    { name: "wget", category: "Dev Tools", format: "deb", desc: "Network utility to retrieve files from the web" }
];

const appGrid = document.getElementById('appGrid');
const searchInput = document.getElementById('searchInput');
const filterBtns = document.querySelectorAll('.filter-btn');

let currentFilter = 'all';
let searchQuery = '';

function renderApps() {
    appGrid.innerHTML = '';
    
    const filteredApps = apps.filter(app => {
        const matchesSearch = app.name.toLowerCase().includes(searchQuery.toLowerCase()) || 
                              app.desc.toLowerCase().includes(searchQuery.toLowerCase()) ||
                              app.category.toLowerCase().includes(searchQuery.toLowerCase());
        
        const matchesCategory = currentFilter === 'all' || app.category === currentFilter;
        
        return matchesSearch && matchesCategory;
    });

    if(filteredApps.length === 0) {
        appGrid.innerHTML = '<div style="grid-column: 1/-1; text-align: center; color: var(--text-secondary); padding: 3rem;">No applications found matching your criteria.</div>';
        return;
    }

    filteredApps.forEach((app, index) => {
        const formatClass = app.format.toLowerCase() === 'flatpak' ? 'format-flatpak' : 
                            (app.format.toLowerCase() === 'deb' ? 'format-deb' : 'format-dev');

        const card = document.createElement('div');
        card.className = 'app-card';
        card.style.animationDelay = `${index * 0.05}s`;
        
        card.innerHTML = `
            <div class="card-header">
                <h3 class="app-name">${app.name}</h3>
                <span class="format-badge ${formatClass}">${app.format}</span>
            </div>
            <div class="app-category">${app.category}</div>
            <p class="app-desc">${app.desc}</p>
        `;
        
        appGrid.appendChild(card);
    });
}

searchInput.addEventListener('input', (e) => {
    searchQuery = e.target.value;
    renderApps();
});

filterBtns.forEach(btn => {
    btn.addEventListener('click', () => {
        // Update active class
        filterBtns.forEach(b => b.classList.remove('active'));
        btn.classList.add('active');
        
        // Update filter and render
        currentFilter = btn.dataset.filter;
        renderApps();
    });
});

// Initial render
renderApps();
