const { invoke } = window.__TAURI__.core;

let gameMessageElement;
let solutionCountElement;
let gridElement;
let selectedCell = null;

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

function set_solution_count(text) {
    solutionCountElement.textContent = text;
}

function render_grid(gridData) {
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

    set_solution_count(gridData.remainingSolutions.text);

    // Update message based on grid state
    if (error_found) {
        set_message_bad("Oops!  Something is wrong");
    } else if (gridData.isSolved) {
        set_message_good("Congratulations!  Puzzle Solved!");
    } else {
        set_message_ok("Use arrows or click to select cells and type 1 - 9 to set values.");
    }
}

function selected_cell_index() {
    return parseInt(selectedCell.getAttribute('data-index'), 10);
}

async function user_action(request) {
    try {
        const gridData = await invoke("user_change", { request });
        render_grid(gridData);
    } catch (error) {
        const error_message = typeof error === "string" ? error : JSON.stringify(error);
        set_message_bad(error_message);
    }
}

window.addEventListener("DOMContentLoaded", () => {
    gameMessageElement = document.querySelector("#game-message");
    solutionCountElement = document.querySelector("#solution-count");
    gridElement = document.querySelector("#sudokuGrid");
    const clearButton = document.querySelector("#clear-button");

    selectedCell = document.querySelector(`.cell[data-index="0"]`);
    selectedCell.classList.add('selected');

    clearButton.addEventListener("click", () => {
        set_message_ok("Clearing the puzzle. Hang on...");
        user_action({ action: "clear_grid" });
    });

    // Add click event listener to highlight the selected cell
    gridElement.addEventListener('click', (e) => {
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
        const dataIndex = parseInt(selectedCell.getAttribute('data-index'), 10);
        if (selectedCell) {
            if ((e.key >= '1' && e.key <= '9') || e.key === 'Backspace' || e.key === 'Delete') {
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
                    e.preventDefault();
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
            if (e.key >= '1' && e.key <= '9') {
                user_action({
                    action: "set_cell",
                    cell_index: selected_cell_index(),
                    value: parseInt(e.key, 10),
                });
            } else if (e.key === 'Backspace' || e.key === 'Delete') {
                user_action({
                    action: "clear_cell",
                    cell_index: selected_cell_index(),
                });
            } else if (e.key === 'C' || e.key === 'c') {
                user_action({ action: "clear_grid" });
            } else if (e.key === 'D' || e.key === 'd') {
                user_action({ action: "set_debug", enabled: true });
            } else if (e.key === 'O' || e.key === 'o') {
                user_action({ action: "set_debug", enabled: false });
            }
        }
    });

    user_action({ action: "get_grid" });
});
