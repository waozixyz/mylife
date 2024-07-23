const defaultConfig = `name: John Doe
date_of_birth: "2000-01"
life_periods:
  - name: Childhood
    start: "2000-01"
    color: "#FFB3BA"
  - name: Teenage Years
    start: "2013-01"
    color: "#BAFFC9"
  - name: Early Adulthood
    start: "2018-01"
    color: "#BAE1FF"
  - name: Career Growth
    start: "2023-01"
    color: "#FFFFBA"`;

let config = jsyaml.load(defaultConfig);

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
    
    // Add event listeners to update on change
    div.querySelectorAll('input').forEach(input => {
        input.addEventListener('change', updateConfigAndTimeline);
    });
    
    return div;
}

function populateForm() {
    const nameInput = document.getElementById('name');
    const dateOfBirthInput = document.getElementById('dateOfBirth');
    const configTextarea = document.getElementById('config');
    
    if (nameInput) nameInput.value = config.name;
    if (dateOfBirthInput) dateOfBirthInput.value = config.date_of_birth;
    
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
    const configTextarea = document.getElementById('config');
    
    if (nameInput) config.name = nameInput.value;
    if (dateOfBirthInput) config.date_of_birth = dateOfBirthInput.value;
    
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

    for (let i = 0; i < 25 * 48; i++) {
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

// Add event listeners to name and date of birth inputs
document.getElementById('name').addEventListener('change', updateConfigAndTimeline);
document.getElementById('dateOfBirth').addEventListener('change', updateConfigAndTimeline);

// Initial setup
populateForm();
updateTimeline();
