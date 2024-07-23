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

document.getElementById('config').value = defaultConfig;
const updateTimeline = () => {
    const configText = document.getElementById('config').value;
    const config = jsyaml.load(configText);

    const timeline = document.getElementById('timeline');
    const legend = document.getElementById('legend');
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

const loadFile = () => {
    const fileInput = document.getElementById('fileInput');
    const file = fileInput.files[0];
    if (file) {
        const reader = new FileReader();
        reader.onload = (e) => {
            document.getElementById('config').value = e.target.result;
            updateTimeline();
        };
        reader.readAsText(file);
    }
};

const saveYAML = () => {
    const configText = document.getElementById('config').value;
    const blob = new Blob([configText], { type: 'text/yaml;charset=utf-8' });
    const url = URL.createObjectURL(blob);
    const link = document.createElement('a');
    link.href = url;
    link.download = 'life_timeline_config.yaml';
    link.click();
    URL.revokeObjectURL(url);
};

// Event listeners
document.getElementById('fileInput').addEventListener('change', loadFile);
document.getElementById('saveButton').addEventListener('click', saveYAML);
document.getElementById('updateButton').addEventListener('click', updateTimeline);

// Initial timeline update
updateTimeline();