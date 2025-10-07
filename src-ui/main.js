const { invoke } = window.__TAURI__.tauri;

let selectedCell = null;

async function user_input(keyPress) {
    // Learn more about Tauri commands at https://v1.tauri.app/v1/guides/features/command
    let grid_string =
        await invoke("user_change", { user_input: parseInt(keyPress),
                                      cell_index: parseInt(selectedCell.getAttribute('data-index')) });

    for (let index = 0; index <= 80; index++) {
        let nextCell = document.querySelector(`.cell[data-index="${index}"]`);
        if (grid_string.charAt((index*2)+1) === '0') {
            nextCell.textContent = '';
            nextCell.classList.remove('user-set');
            nextCell.classList.remove('solved');
            nextCell.classList.remove('empty-error');
        } else if (grid_string.charAt((index*2)+1) === 'x') {
            nextCell.textContent = 'X';
            nextCell.classList.add('empty-error');
        } else {
            if (grid_string.charAt(index*2) === 'u') {
                nextCell.classList.add('user-set');
            } else {
                nextCell.classList.add('solved');
            }
            nextCell.textContent = grid_string.charAt((index*2)+1);
        }
    }
}

window.addEventListener("DOMContentLoaded", () => {
    document.querySelector("#puzzle-form").addEventListener("submit", (e) => {
        e.preventDefault();
        user_input();
    });

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
            user_input(e.key);
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
