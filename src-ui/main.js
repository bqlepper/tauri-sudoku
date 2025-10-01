const { invoke } = window.__TAURI__.tauri;

let userInputEl;
let puzzleMsgEl;
let selectedCell = null;

async function user_input() {
  // Learn more about Tauri commands at https://v1.tauri.app/v1/guides/features/command
  puzzleMsgEl.textContent =
      await invoke("get_puzzle", { user_input: userInputEl.value,
                                   cell_index: parseInt(selectedCell.getAttribute('data-index')) });
}

window.addEventListener("DOMContentLoaded", () => {
    userInputEl = document.querySelector("#greet-input");
    puzzleMsgEl = document.querySelector("#greet-msg");
    document.querySelector("#greet-form").addEventListener("submit", (e) => {
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
            userInputEl.value = e.key;
            user_input();
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
