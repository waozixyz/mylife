const defaultConfig = `name: John Doe
date_of_birth: "2000-01"
life_expectancy: 80
life_periods:
  - name: Childhood
    start: "2000-01"
    color: "#FF6666"  # Dark Red
  - name: Teenage Years
    start: "2013-01"
    color: "#66FF66"  # Dark Green
  - name: Early Adulthood
    start: "2018-01"
    color: "#6666FF"  # Dark Blue
  - name: Career Growth
    start: "2023-01"
    color: "#FFFF66"  # Dark Yellow`;

let config = jsyaml.load(defaultConfig);

if (!config.life_expectancy) {
    config.life_expectancy = 80;
}

function createLifePeriodElement(period = { name: '', start: '', color: '#000000' }) {
    const div = document.createElement('div');
    div.className = 'life-period';
    div.innerHTML = `
        <input type="text" class="period-name" placeholder="Period Name" value="${period.name}" required>
        <input type="month" class="period-start" value="${period.start}" required>
        <input type="color" class="period-color" value="${period.color}">
        <button type="button" class="remove-period">Remove</button>
    `;
    div.querySelector('.remove-period').addEventListener('click', () => {
        div.remove();
        updateConfigAndTimeline();
    });
    
    div.querySelectorAll('input').forEach(input => {
        input.addEventListener('change', updateConfigAndTimeline);
    });
    
    return div;
}

function populateForm() {
    const nameInput = document.getElementById('name');
    const dateOfBirthInput = document.getElementById('dateOfBirth');
    const lifeExpectancyInput = document.getElementById('lifeExpectancy')
    const configTextarea = document.getElementById('config');

    if (nameInput) nameInput.value = config.name;
    if (dateOfBirthInput) dateOfBirthInput.value = config.date_of_birth;
    if (lifeExpectancyInput) lifeExpectancyInput.value = config.life_expectancy;

    const periodsContainer = document.getElementById('lifePeriods');
    if (periodsContainer) {
        periodsContainer.innerHTML = '';
        config.life_periods.forEach(period => {
            periodsContainer.appendChild(createLifePeriodElement(period));
        });
    }

    if (configTextarea) configTextarea.value = jsyaml.dump(config);
}

function updateConfig() {
    const nameInput = document.getElementById('name');
    const dateOfBirthInput = document.getElementById('dateOfBirth');
    const lifeExpectancyInput = document.getElementById('lifeExpectancy');
    const configTextarea = document.getElementById('config');
    
    if (nameInput) config.name = nameInput.value;
    if (dateOfBirthInput) config.date_of_birth = dateOfBirthInput.value;
    if (lifeExpectancyInput) config.life_expectancy = parseInt(lifeExpectancyInput.value, 10);

    config.life_periods = Array.from(document.querySelectorAll('.life-period')).map(el => ({
        name: el.querySelector('.period-name')?.value || '',
        start: el.querySelector('.period-start')?.value || '',
        color: el.querySelector('.period-color')?.value || '#000000'
    }));

    if (configTextarea) configTextarea.value = jsyaml.dump(config);
}

function updateConfigAndTimeline() {
    updateConfig();
    updateTimeline();
}

const updateTimeline = () => {
    const timeline = document.getElementById('timeline');
    const legend = document.getElementById('legend');
    if (!timeline || !legend) return;

    timeline.innerHTML = '';
    legend.innerHTML = '';

    const dob = new Date(config.date_of_birth);
    const today = new Date();
    const grid = document.createElement('div');
    grid.className = 'grid';

    const maxMonths = config.life_expectancy * 12; // Calculate the total number of months based on life expectancy

    for (let i = 0; i < maxMonths; i++) {
        const cell = document.createElement('div');
        cell.className = 'cell';
        const currentDate = new Date(dob.getFullYear(), dob.getMonth() + i, 1);

        let color = 'white';
        if (currentDate <= today) {
            for (const period of config.life_periods) {
                const startDate = new Date(period.start);
                if (currentDate.getFullYear() === startDate.getFullYear() && 
                    currentDate.getMonth() === startDate.getMonth()) {
                    color = period.color;
                    break;
                } else if (currentDate > startDate) {
                    color = period.color;
                } else {
                    break;
                }
            }
        }

        cell.style.backgroundColor = color;
        grid.appendChild(cell);
    }

    timeline.appendChild(grid);

    config.life_periods.forEach(period => {
        const legendItem = document.createElement('div');
        legendItem.className = 'legend';
        const colorBox = document.createElement('div');
        colorBox.className = 'legend-color';
        colorBox.style.backgroundColor = period.color;
        const text = document.createElement('span');
        text.textContent = `${period.name} (from ${period.start})`;
        legendItem.append(colorBox, text);
        legend.appendChild(legendItem);
    });
};

function attachEventListeners() {
    document.getElementById('configForm').addEventListener('submit', (e) => {
        e.preventDefault();
        updateTimeline();
    });

    document.getElementById('addPeriod').addEventListener('click', () => {
        const newPeriod = createLifePeriodElement();
        document.getElementById('lifePeriods').appendChild(newPeriod);
        updateConfigAndTimeline();
    });

    document.getElementById('fileInput').addEventListener('change', (e) => {
        const file = e.target.files[0];
        if (file) {
            const reader = new FileReader();
            reader.onload = (e) => {
                config = jsyaml.load(e.target.result);

                if (!config.life_expectancy) {
                    config.life_expectancy = 80;
                }
                populateForm();
                updateTimeline();
            };
            reader.readAsText(file);
        }
    });

    document.getElementById('saveButton').addEventListener('click', () => {
        updateConfig();
        const yamlString = jsyaml.dump(config);
        const blob = new Blob([yamlString], { type: 'text/yaml;charset=utf-8' });
        const url = URL.createObjectURL(blob);
        const link = document.createElement('a');
        link.href = url;
        link.download = 'life_timeline_config.yaml';
        link.click();
        URL.revokeObjectURL(url);
    });

    document.getElementById('name').addEventListener('change', updateConfigAndTimeline);
    document.getElementById('dateOfBirth').addEventListener('change', updateConfigAndTimeline);
    document.getElementById('lifeExpectancy').addEventListener('change', updateConfigAndTimeline); 
}

const configPanel = document.getElementById('configPanel');
const modal = document.getElementById("modal");
const openConfigBtn = document.getElementById("openConfig");
const closeBtn = document.getElementsByClassName("close")[0];
const modalContent = document.getElementById("modalContent");
const container = document.querySelector(".container");

function moveConfigPanel(toModal) {
    if (toModal) {
        modalContent.appendChild(configPanel);
    } else {
        container.insertBefore(configPanel, container.firstChild);
    }
}

function toggleModal(show) {
    modal.style.display = show ? "block" : "none";
}

openConfigBtn.onclick = function() {
    moveConfigPanel(true);
    toggleModal(true);
}

if (closeBtn) {
    closeBtn.onclick = function() {
        toggleModal(false);
    }
} else {
    console.error("Close button not found. Make sure an element with class 'close' exists in the modal.");
}

window.onclick = function(event) {
    if (event.target == modal) {
        toggleModal(false);
    }
}
let previousWindowSize = null;

function checkScreenSize() {
    const currentWindowSize = window.innerWidth;
    if (previousWindowSize !== currentWindowSize) {
        if (currentWindowSize <= 768) {
            moveConfigPanel(true);
            toggleModal(true);
        } else {
            moveConfigPanel(false);
            toggleModal(false);
        }
        previousWindowSize = currentWindowSize;
    }
}

checkScreenSize();
window.addEventListener('resize', checkScreenSize);

populateForm();
updateTimeline();
attachEventListeners();



// Function to get the current configuration and serialize it
function getConfig() {
    updateConfig();
    return config;
}

// Function to serialize configuration and share it via URL
function shareConfigURL() {
    const config = getConfig(); // Get the current config object
    const configString = encodeURIComponent(JSON.stringify(config));
    const url = `${window.location.origin}${window.location.pathname}?config=${configString}`;
    navigator.clipboard.writeText(url).then(() => {
        alert('Config URL copied to clipboard!');
    }).catch(err => {
        console.error('Failed to copy URL: ', err);
    });
}

// Function to load the configuration from the URL parameter
function loadConfigFromURL() {
    const params = new URLSearchParams(window.location.search);
    if (params.has('config')) {
        const configString = decodeURIComponent(params.get('config'));
        config = JSON.parse(configString);
        populateForm();
        updateTimeline();
    }
}

// Attach the shareConfigURL function to the share button
document.getElementById('shareButton').addEventListener('click', shareConfigURL);

// Call this function on page load to load config from URL if present
document.addEventListener('DOMContentLoaded', () => {
    loadConfigFromURL();
    populateForm();
    updateTimeline();
    attachEventListeners();
});