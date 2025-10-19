const { invoke } = window.__TAURI__.tauri;

let gameMessageElement;
let selectedCell = null;

const clear_input = '10';
const solve_input = '11';
const debug_on = '12';
const debug_off = '13';

async function user_input(keyPress) {
    // Learn more about Tauri commands at https://v1.tauri.app/v1/guides/features/command
    let grid_string =
        await invoke("user_change", { user_input: parseInt(keyPress),
                                      cell_index: parseInt(selectedCell.getAttribute('data-index')) });

    // Check for error message
    if ((grid_string.charAt(0) !== '-') &&
        (grid_string.charAt(0) !== 'u') &&
        (grid_string.charAt(0) !== 's')) {
        gameMessageElement.textContent = grid_string;
        gameMessageElement.classList.remove("message-good");
        gameMessageElement.classList.add("message-bad");
        return;
    }

    let solved = true;
    let error_found = false;
    // Parse the return string that represents the grid state
    // The string is 162 characters long, representing 81 cells
    // Each cell is represented by two characters:
    // First character: 'u' for user-set, 's' for solved
    // Second character: '0' for empty, '1'-'9' for numbers, 'x' for error (invalid input)
    for (let index = 0; index <= 80; index++) {
        let nextCell = document.querySelector(`.cell[data-index="${index}"]`);
        if (grid_string.charAt((index*2)+1) === '0') {
            solved = false;
            nextCell.textContent = '';
            nextCell.classList.remove('user-set');
            nextCell.classList.remove('solved');
            nextCell.classList.remove('empty-error');
        } else if (grid_string.charAt((index*2)+1) === 'x') {
            error_found = true;
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
    if (error_found === true) {
        gameMessageElement.textContent = "Oops!  Something is wrong";
        gameMessageElement.classList.remove("message-good");
        gameMessageElement.classList.add("message-bad");
    } else if (solved === true) {
        gameMessageElement.textContent = "Congratulations!  Puzzle Solved!";
        gameMessageElement.classList.add("message-good");
        gameMessageElement.classList.remove("message-bad");
    } else {
        gameMessageElement.textContent = "";
        gameMessageElement.classList.remove("message-good");
        gameMessageElement.classList.remove("message-bad");
    }
}

window.addEventListener("DOMContentLoaded", () => {
    gameMessageElement = document.querySelector("#game-message");
    const clearButton = document.querySelector("#clear-button");
    const searchButton = document.querySelector("#search-button");
    clearButton.addEventListener("click", () => {
        user_input(clear_input);
    });
    searchButton.addEventListener("click", () => {
        gameMessageElement.textContent = "Searching for a solution.  Hang on...";
        gameMessageElement.classList.add("message-good");
        gameMessageElement.classList.remove("message-bad");
        user_input(solve_input);
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
        if (selectedCell) {
            if (e.key >= '0' && e.key <= '9') {
                gameMessageElement.textContent = "Processing Input.  Hang on...";
                gameMessageElement.classList.add("message-good");
                gameMessageElement.classList.remove("message-bad");
                user_input(e.key);
            } else if (e.key === 'Backspace' || e.key === 'Delete') {
                user_input('0');
            } else if (e.key === 'C' || e.key === 'c') {
                user_input(clear_input);
            } else if (e.key === 'S' || e.key === 's') {
                user_input(solve_input);
            } else if (e.key === 'D' || e.key === 'd') {
                user_input(debug_on);
            } else if (e.key === 'O' || e.key === 'o') {
                user_input(debug_off);
            } else if ((e.key === 'Tab') ||
                       (e.key === 'ArrowRight') ||
                       (e.key === 'ArrowLeft') ||
                       (e.key === 'ArrowUp') ||
                       (e.key === 'ArrowDown')) {
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
        }
    });
});
