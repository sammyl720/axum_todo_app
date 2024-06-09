const createTodoDialog = document.querySelector('dialog.create_todo_dialog');
const openDialogBtn = document.getElementById('open_dialog_btn');
const cancelBtn = document.getElementById('cancel_create_btn');
const createTodoForm = document.getElementById('create_todo_form');

createTodoForm.addEventListener('submit', async (event) => {
    event.preventDefault();
    const todo_description = createTodoForm.description.value;
    const result = await add_todo(todo_description);
    populate_todos();
    notify(`Created todo with id ${result.id}`);
    createTodoForm.description.value = "";
    createTodoDialog.close();
});

openDialogBtn.addEventListener('click', () => {
    createTodoDialog.showModal();
});

cancelBtn.addEventListener('click',() => {
    createTodoDialog.close();
});

document.addEventListener("DOMContentLoaded", async() => {
    await populate_todos();
})

async function populate_todos() {
    const todos = await fetch_todos();
    const todosEl = document.getElementById("todos");
    todosEl.innerHTML = "";
    todos.forEach(todo => {
        todosEl.appendChild(createTodoElement(todo))
    });
    notify('Todo list updated')
}

function createTodoElement(todo) {
    const element = document.createElement('div');
    element.classList.add('todo-item');
    element.dataset.id = `${todo.id}`;
    if (todo.done) {
        element.dataset.done = "1";
    }
    element.innerHTML = `<p class="todo-description">${todo.description}<p> <span>(${todo.done ? '' : 'Not'} Completed)</span>`;
    element.appendChild(createActionButtons(todo));
    return element;
}

function createActionButtons(todo) {
    const element = document.createElement('div');
    element.classList.add('todo__action-buttons');
    element.appendChild(createToggleCompleteButton(todo));
    element.appendChild(createDeleteActionButton(todo));
    return element;
}
function createDeleteActionButton(todo) {
    const deleteBtn = document.createElement('button');
    deleteBtn.classList.add('btn')
    deleteBtn.classList.add('btn--danger');
    deleteBtn.textContent = "Remove"
    deleteBtn.addEventListener('click', async() => {
        await delete_todo(todo.id);
        notify(`Todo with ${todo.id} was deleted`);
        await populate_todos();
    });

    return deleteBtn;
}

function createToggleCompleteButton(todo) {
    const toggleButton = document.createElement('button');
    toggleButton.classList.add('btn');
    toggleButton.textContent = todo.done ? 'Reset' : 'Complete';
    toggleButton.addEventListener('click', async() => {
        const done = !todo.done;
        await update_todo({
            ...todo,
            done,
        });
        notify(`Todo with id ${todo.id} marked as ${done ? '' : 'not'} completed`)
        await populate_todos();
    });

    return toggleButton;
}

const TODO_API = "/api/todos";

async function add_todo(description) {
    const response = await fetch(TODO_API, {
        method: 'POST',
        body: JSON.stringify({ description }),
        headers: {
            'Content-Type': 'application/json'
        }
    });

    const result = await response.json();
    return result; 
}
async function delete_todo(todo_id) {
    const result = await fetch(`${TODO_API}/delete/${todo_id}`, {
        method: 'DELETE'
    });
    const response = await result.json();
    return response;
}

async function update_todo(todo) {
    const result = await fetch(`${TODO_API}/edit`, {
        method: 'PUT',
        body: JSON.stringify(todo),
        headers: {
            'Content-Type': 'application/json'
        }
    });
    const response = await result.json();
    return response;
}

async function fetch_todos() {
    const result = await fetch(TODO_API);
    const todos = await result.json();
    return todos;
}

function notify(message) {
    console.log(`${new Date().toUTCString()}: ${message}`)
}