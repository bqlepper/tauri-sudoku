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
    let grid_string =
        await invoke("user_change", { user_input: parseInt(keyPress),
                                      cell_index: parseInt(selectedCell.getAttribute('data-index')) });

    // Check for error message
    if ((grid_string.charAt(0) !== '-') &&
        (grid_string.charAt(0) !== 'u') &&
        (grid_string.charAt(0) !== 's')) {
        set_message_bad(grid_string);
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
        set_message_bad("Oops!  Something is wrong");
    } else if (solved === true) {
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
