const { invoke } = window.__TAURI__.tauri;

let bqlMsgElement = null;
let selectedCell = null;

async function bqlTest(e, cellNumber) {
    // Learn more about Tauri commands at https://v1.tauri.app/v1/guides/features/command
    bqlMsgElement.textContent = `Kee pressd: ${e.key}, Cell number: ${cellNumber}`;
    //bqlMsgElement.textContent = await invoke("greet", { name: e.key });
}

window.addEventListener("DOMContentLoaded", () => {
    bqlMsgElement = document.querySelector("#bql-msg");
    bqlMsgElement.textContent = "Ready to go!";

    const grid = document.getElementById('sudokuGrid');

    // Add click event listener to highlight the selected cell
    grid.addEventListener('click', (e) => {
        if (e.target.classList.contains('cell')) {
            if (selectedCell) {
                selectedCell.classList.remove('selected');
            }
            selectedCell = e.target;
            selectedCell.classList.add('selected');
        }
    });

    // Add keyboard event listener to set values in the selected cell
    document.addEventListener('keydown', (e) => {
        const dataIndex = parseInt(selectedCell.getAttribute('data-index'));
        if (selectedCell && e.key >= '1' && e.key <= '9') {
            bqlTest(e, dataIndex);
            selectedCell.textContent = e.key;
        } else if (selectedCell &&
                   ((e.key === 'Tab') ||
                    (e.key === 'ArrowRight') ||
                    (e.key === 'ArrowLeft') ||
                    (e.key === 'ArrowUp') ||
                    (e.key === 'ArrowDown'))) {
            let nextIndex;
            if (e.key === 'ArrowLeft') {
                nextIndex = (dataIndex - 1 + 81) % 81;
            } else if (e.key === 'Tab' || e.key === 'ArrowRight') {
                nextIndex = (dataIndex + 1) % 81;
            } else if (e.key === 'ArrowDown') {
                nextIndex = (dataIndex + 9) % 81;
            } else if (e.key === 'ArrowUp') {
                nextIndex = (dataIndex - 9 + 81) % 81;
            }
            const nextCell = document.querySelector(`.cell[data-index="${nextIndex}"]`);
            if (nextCell) {
                selectedCell.classList.remove('selected');
                selectedCell = nextCell;
                selectedCell.classList.add('selected');
            }
        }
    });
});
