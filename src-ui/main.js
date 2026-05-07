const { invoke } = window.__TAURI__.core;

let gameMessageElement;
let selectedCell = null;

const clear_input = '10';
const solve_input = '11';
const debug_on = '12';
const debug_off = '13';

function set_message_ok(message) {
    gameMessageElement.textContent = message;
    gameMessageElement.classList.add("message-ok");
    gameMessageElement.classList.remove("message-good");
    gameMessageElement.classList.remove("message-bad");
}

function set_message_bad(message) {
    gameMessageElement.textContent = message;
    gameMessageElement.classList.remove("message-good");
    gameMessageElement.classList.remove("message-ok");
    gameMessageElement.classList.add("message-bad");
}

function set_message_good(message) {
    gameMessageElement.textContent = message;
    gameMessageElement.classList.add("message-good");
    gameMessageElement.classList.remove("message-bad");
    gameMessageElement.classList.remove("message-ok");
}

async function user_input(keyPress) {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    let grid_json =
        await invoke("user_change", { user_input: parseInt(keyPress),
                                      cell_index: parseInt(selectedCell.getAttribute('data-index')) });

    // Parse JSON response
    let gridData;
    try {
        gridData = JSON.parse(grid_json);
    } catch (e) {
        // If JSON parse fails, it's an error message
        set_message_bad(grid_json);
        return;
    }

    let error_found = false;

    // Update all cells from JSON data
    for (let index = 0; index <= 80; index++) {
        let nextCell = document.querySelector(`.cell[data-index="${index}"]`);
        let cellData = gridData.cells[index];

        // Clear all classes first
        nextCell.classList.remove('user-set', 'solved', 'empty-error');

        if (cellData.isError) {
            error_found = true;
            nextCell.textContent = 'X';
            nextCell.classList.add('empty-error');
        } else if (cellData.value === 0) {
            nextCell.textContent = '';
        } else {
            nextCell.textContent = cellData.value;
            if (cellData.isUserSet) {
                nextCell.classList.add('user-set');
            } else if (cellData.isSolved) {
                nextCell.classList.add('solved');
            }
        }
    }

    // Update message based on grid state
    if (error_found) {
        set_message_bad("Oops!  Something is wrong");
    } else if (gridData.isSolved) {
        set_message_good("Congratulations!  Puzzle Solved!");
    } else {
        set_message_ok("Use arrows or click to select cells and type 1 - 9 to set values.");
    }
}

window.addEventListener("DOMContentLoaded", () => {
    gameMessageElement = document.querySelector("#game-message");
    const clearButton = document.querySelector("#clear-button");
    const searchButton = document.querySelector("#search-button");

    selectedCell = document.querySelector(`.cell[data-index="0"]`);
    selectedCell.classList.add('selected');

    clearButton.addEventListener("click", () => {
        user_input(clear_input);
    });

    searchButton.addEventListener("click", () => {
        set_message_ok("Searching for a solution.  Hang on...");
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
                set_message_ok("Processing your input.  Hang on...");
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

    // Add keyboard event listener to set values in the selected cell
    document.addEventListener('keyup', (e) => {
        if (selectedCell) {
            if (e.key >= '0' && e.key <= '9') {
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
            }
        }
    });
});
