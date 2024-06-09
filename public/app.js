

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
        await populate_todos();
    });

    return deleteBtn;
}

function createToggleCompleteButton(todo) {
    const toggleButton = document.createElement('button');
    toggleButton.classList.add('btn');
    toggleButton.textContent = todo.done ? 'Reset' : 'Complete';
    toggleButton.addEventListener('click', async() => {
        console.log(todo, 'todo')
        await update_todo({
            ...todo,
            done: !todo.done
        });
        await populate_todos();
    });

    return toggleButton;
}

const TODO_API = "/api/todos";

async function delete_todo(todo_id) {
    const result = await fetch(`${TODO_API}/delete/${todo_id}`, {
        method: 'DELETE'
    });
    const response = await result.json();
    console.log(response);
    return response;
}

async function update_todo(todo) {
    console.log(todo)
    const result = await fetch(`${TODO_API}/edit`, {
        method: 'PUT',
        body: JSON.stringify(todo),
        headers: {
            'Content-Type': 'application/json'
        }
    });
    const response = await result.json();
    console.log(response);
    return response;
}

async function fetch_todos() {
    const result = await fetch(TODO_API);
    const todos = await result.json();
    return todos;
}